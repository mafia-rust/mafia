use crate::{
    game::{
        ability_input::{
            ability_selection::AvailableAbilitySelection,
            selection_type::one_player_option_selection::AvailableOnePlayerOptionSelection,
            AbilityID, AbilityInput, AvailableAbilityInput
        },
        attack_power::AttackPower, grave::GraveKiller, phase::PhaseType, player::PlayerReference,
        role::Priority, role_list::RoleSet, tag::Tag, visit::{Visit, VisitTag}, Game
    }, 
    packet::ToClientPacket
};

use super::{detained::Detained, insider_group::InsiderGroupID, night_visits::NightVisits};

#[derive(Default)]
pub struct SyndicateGunItem {
    player_with_gun: Option<PlayerReference>,
    gun_target: Option<PlayerReference>,
}

impl SyndicateGunItem {
    pub fn give_gun(game: &mut Game, player: PlayerReference) {
        Self::take_gun(game);
        game.syndicate_gun_item.player_with_gun = Some(player);

        for insider in InsiderGroupID::Mafia.players(game).clone() {
            insider.push_player_tag(game, player, Tag::SyndicateGun);
            insider.send_packet(game, ToClientPacket::YourSyndicateGunItemData{
                shooter: game.syndicate_gun_item.player_with_gun,
                target: game.syndicate_gun_item.gun_target
            });
        }
    }
    pub fn take_gun(game: &mut Game) {
        game.syndicate_gun_item.player_with_gun = None;
        game.syndicate_gun_item.gun_target = None;

        for insider in InsiderGroupID::Mafia.players(game).clone() {
            insider.remove_player_tag_on_all(game, Tag::SyndicateGun);
            insider.send_packet(game, ToClientPacket::YourSyndicateGunItemData{
                shooter: game.syndicate_gun_item.player_with_gun,
                target: game.syndicate_gun_item.gun_target
            });
        }
    }

    pub fn target_gun(game: &mut Game, player: Option<PlayerReference>) {
        game.syndicate_gun_item.gun_target = player;
        Self::send_gun_data(game);
    }

    pub fn send_gun_data(game: &Game) {
        for insider in InsiderGroupID::Mafia.players(game).clone() {
            insider.send_packet(game, ToClientPacket::YourSyndicateGunItemData{
                shooter: game.syndicate_gun_item.player_with_gun,
                target: game.syndicate_gun_item.gun_target
            });
        }
    }

    //available ability
    pub fn available_ability_input(game: &Game, actor: PlayerReference) -> AvailableAbilityInput {
        let mut out = AvailableAbilityInput::default();

        if let Some(player_with_gun) = game.syndicate_gun_item.player_with_gun {
            if actor == player_with_gun {
                out.insert_ability(
                    AbilityID::SyndicateGunItemShoot, 
                    AvailableAbilitySelection::OnePlayerOption{
                        selection: AvailableOnePlayerOptionSelection(
                            PlayerReference::all_players(game)
                                .into_iter()
                               .filter(|target|
                                    actor != *target &&
                                    !Detained::is_detained(game, actor) &&
                                    actor.alive(game) &&
                                    target.alive(game) &&
                                    !InsiderGroupID::in_same_revealed_group(game, actor, *target))
                                .map(|p|Some(p))
                                .chain(std::iter::once(None))
                                .collect()
                        )
                    }
                );


                out.insert_ability(
                    AbilityID::SyndicateGunItemGive, 
                    AvailableAbilitySelection::OnePlayerOption{
                        selection: AvailableOnePlayerOptionSelection(
                            PlayerReference::all_players(game)
                                .into_iter()
                               .filter(|target|
                                    actor != *target &&
                                    actor.alive(game) &&
                                    target.alive(game) &&
                                    InsiderGroupID::Mafia.is_player_in_revealed_group(game, *target))
                                .map(|p|Some(p))
                                .chain(std::iter::once(None))
                                .collect()
                        )
                    }
                );
            }
        }

        out
    }


    //event listeners
    pub fn on_phase_start(game: &mut Game, phase: PhaseType) {
        if phase == PhaseType::Night {
            Self::target_gun(game, None);
        }
    }
    pub fn on_any_death(game: &mut Game, player: PlayerReference) {
        if game.syndicate_gun_item.player_with_gun.is_some_and(|p|p==player) {
            game.syndicate_gun_item.player_with_gun = None;
            game.syndicate_gun_item.gun_target = None;

            for insider in InsiderGroupID::Mafia.players(game).clone() {
                insider.remove_player_tag_on_all(game, Tag::SyndicateGun);
            }
            for insider in InsiderGroupID::Mafia.players(game).iter()
                .filter(|p|p.alive(game))
                .cloned()
                .collect::<Vec<_>>()
            {
                SyndicateGunItem::give_gun(game, insider);
            }
        }
    }
    pub fn on_night_priority(game: &mut Game, priority: Priority) {
        match priority {
            Priority::TopPriority => {
                let Some(player_with_gun) = game.syndicate_gun_item.player_with_gun else {return}; 
                let Some(gun_target) = game.syndicate_gun_item.gun_target else {return}; 
                NightVisits::add_visit(
                    game, 
                    Visit::new(player_with_gun, gun_target, true, VisitTag::SyndicateGunItem)
                );
            }
            Priority::Kill => {
                let targets: Vec<(PlayerReference, PlayerReference)> = NightVisits::all_visits(game)
                    .iter()
                    .filter(|visit| visit.tag == VisitTag::SyndicateGunItem)
                    .map(|visit| (visit.visitor, visit.target))
                    .collect();

                for (attacker, target) in targets {
                    target.try_night_kill_single_attacker(
                        attacker,
                        game,
                        GraveKiller::RoleSet(RoleSet::Mafia),
                        AttackPower::Basic,
                        false
                    );
                }
            }
            _ => {}
        }
    }
    pub fn on_ability_input_received(game: &mut Game, actor_ref: PlayerReference, ability_input: AbilityInput) {
        if let Some(player_with_gun) = game.syndicate_gun_item.player_with_gun {
            if actor_ref != player_with_gun {
                return;
            }
        }else{
            return;
        }

        if let Some(selection) = ability_input
            .get_player_option_selection_if_id(AbilityID::SyndicateGunItemGive)
        {
            if let Some(target) = selection.0 {
                if 
                    actor_ref != target &&
                    target.alive(game) &&
                    InsiderGroupID::Mafia.is_player_in_revealed_group(game, target) 
                {
                    SyndicateGunItem::give_gun(game, target);
                }
            }
        }else if let Some(selection) = ability_input
            .get_player_option_selection_if_id(AbilityID::SyndicateGunItemShoot) 
        {

            if let Some(target) = selection.0 {
                if 
                    actor_ref != target &&
                    !Detained::is_detained(game, actor_ref) &&
                    actor_ref.alive(game) &&
                    target.alive(game) &&
                    !InsiderGroupID::in_same_revealed_group(game, actor_ref, target) &&
                    game.day_number() > 1
                {
                    SyndicateGunItem::target_gun(game, selection.0);
                }else{
                    return;
                }
            }else{
                SyndicateGunItem::target_gun(game, None);
            }

            game.add_message_to_chat_group(
                crate::game::chat::ChatGroup::Mafia, 
                crate::game::chat::ChatMessageVariant::SyndicateGunTarget {
                    shooter: actor_ref.index(),
                    target: selection.0.map(|f|f.index()),
                }
            );
        }
    }
}