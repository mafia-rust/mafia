use crate::{
    game::{
        ability_input::*,
        attack_power::AttackPower, grave::GraveKiller, phase::PhaseType, player::PlayerReference,
        role::{common_role, Priority},
        role_list::RoleSet, tag::Tag, visit::{Visit, VisitTag}, Game
    }, 
    vec_set::vec_set
};

use super::{detained::Detained, insider_group::InsiderGroupID, night_visits::NightVisits};

#[derive(Default)]
pub struct SyndicateGunItem {
    player_with_gun: Option<PlayerReference>
}

impl SyndicateGunItem {
    pub fn give_gun(game: &mut Game, player: PlayerReference) {
        Self::take_gun(game);
        game.syndicate_gun_item.player_with_gun = Some(player);

        for insider in InsiderGroupID::Mafia.players(game).clone() {
            insider.push_player_tag(game, player, Tag::SyndicateGun);
        }
    }
    pub fn take_gun(game: &mut Game) {
        game.syndicate_gun_item.player_with_gun = None;

        for insider in InsiderGroupID::Mafia.players(game).clone() {
            insider.remove_player_tag_on_all(game, Tag::SyndicateGun);
        }
    }

    pub fn player_with_gun(&self) -> Option<PlayerReference> {
        self.player_with_gun
    }

    //available ability
    pub fn controller_parameters_map(game: &Game) -> ControllerParametersMap {
        if let Some(player_with_gun) = game.syndicate_gun_item.player_with_gun {
            common_role::controller_parameters_map_player_list_night_typical(
                game,
                player_with_gun,
                false,
                false,
                game.day_number() <= 1,
                ControllerID::syndicate_gun_item_shoot()
            ).combine_overwrite_owned(
                ControllerParametersMap::new_controller_fast(
                    game,
                    ControllerID::syndicate_gun_item_give(),
                    AvailableAbilitySelection::new_player_list(
                        PlayerReference::all_players(game)
                            .into_iter()
                            .filter(|target|
                                player_with_gun != *target &&
                                target.alive(game) &&
                                InsiderGroupID::Mafia.is_player_in_revealed_group(game, *target))
                            .collect(),
                            false,
                            Some(1)
                    ),
                    AbilitySelection::new_player_list(vec![]),
                    Detained::is_detained(game, player_with_gun) ||
                    !player_with_gun.alive(game),
                    Some(PhaseType::Obituary),
                    true,
                    vec_set![player_with_gun],
                )
            )
        }else{
            ControllerParametersMap::default()
        }
    }


    //event listeners
    pub fn on_any_death(game: &mut Game, player: PlayerReference) {
        if game.syndicate_gun_item.player_with_gun.is_some_and(|p|p==player) {
            game.syndicate_gun_item.player_with_gun = None;

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
        if game.day_number() <= 1 {return}
        match priority {
            Priority::TopPriority => {
                let Some(player_with_gun) = game.syndicate_gun_item.player_with_gun else {return}; 

                let Some(PlayerListSelection(gun_target)) = game.saved_controllers
                    .get_controller_current_selection_player_list(ControllerID::syndicate_gun_item_shoot()) else {return};
                let Some(gun_target) = gun_target.first() else {return};

                NightVisits::add_visit(
                    game, 
                    Visit::new(player_with_gun, *gun_target, true, VisitTag::SyndicateGunItem)
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
    pub fn on_validated_ability_input_received(game: &mut Game, actor_ref: PlayerReference, ability_input: AbilityInput) {
        if let Some(player_with_gun) = game.syndicate_gun_item.player_with_gun {
            if actor_ref != player_with_gun {
                return;
            }
        }else{
            return;
        }

        let Some(PlayerListSelection(target)) = ability_input
            .get_player_list_selection_if_id(ControllerID::SyndicateGunItemGive)
        else {return};
        let Some(target) = target.first() else {return};

        if 
            actor_ref != *target &&
            target.alive(game) &&
            InsiderGroupID::Mafia.is_player_in_revealed_group(game, *target) 
        {
            SyndicateGunItem::give_gun(game, *target);
        }
    }
}