use rand::seq::IndexedRandom;

use crate::game::{
    ability_input::*, attack_power::AttackPower,
    event::on_midnight::{OnMidnight, OnMidnightPriority}, grave::GraveKiller,
    phase::PhaseType, player::PlayerReference, 
    role_list::RoleSet, tag::Tag, visit::{Visit, VisitTag}, Game
};

use super::{detained::Detained, insider_group::InsiderGroupID, night_visits::NightVisits};

#[derive(Default)]
pub struct SyndicateGunItem {
    player_with_gun: Option<PlayerReference>
}

impl SyndicateGunItem {
    pub fn on_visit_wardblocked(game: &mut Game, visit: Visit){
        NightVisits::retain(game, |v|
            v.tag != VisitTag::SyndicateGunItem || v.visitor != visit.visitor
        );
    }
    pub fn on_player_roleblocked(game: &mut Game, player: PlayerReference){
        NightVisits::retain(game, |v|
            v.tag != VisitTag::SyndicateGunItem || v.visitor != player
        );
    }

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
            ControllerParametersMap::combine([
                ControllerParametersMap::builder(game)
                    .id(ControllerID::syndicate_gun_item_shoot())
                    .single_player_selection_typical(player_with_gun, false, false)
                    .night_typical(player_with_gun)
                    .add_grayed_out_condition(game.day_number() <= 1)
                    .build_map(),
                ControllerParametersMap::builder(game)
                    .id(ControllerID::syndicate_gun_item_give())
                    .available_selection(AvailablePlayerListSelection {
                        available_players: PlayerReference::all_players(game)
                            .filter(|target|
                                player_with_gun != *target &&
                                target.alive(game) &&
                                InsiderGroupID::Mafia.is_player_in_revealed_group(game, *target))
                            .collect(),
                        can_choose_duplicates: false,
                        max_players: Some(1)
                    })
                    .add_grayed_out_condition(
                        Detained::is_detained(game, player_with_gun) ||
                        !player_with_gun.alive(game)
                    )
                    .reset_on_phase_start(PhaseType::Obituary)
                    .dont_save()
                    .allow_players([player_with_gun])
                    .build_map()
            ])
        }else{
            ControllerParametersMap::default()
        }
    }

    //event listeners
    pub fn on_any_death(game: &mut Game, player: PlayerReference) {
        if game.syndicate_gun_item.player_with_gun.is_none_or(|p|p!=player) {return}
        let players_to_convert = InsiderGroupID::Mafia.players(game)
            .iter()
            .filter(|p|
                p.alive(game)
            )
            .collect::<Vec<_>>();
        if players_to_convert.is_empty() {return}
        let Some(target) = (
            if let Some(PlayerListSelection(backup)) = game.saved_controllers
                .get_controller_current_selection_player_list(
                ControllerID::syndicate_choose_backup()
                ) {
                    if backup.first().is_some_and(|b|players_to_convert.contains(&b)) {
                        backup.first().copied()
                    } else {
                        players_to_convert.choose(&mut rand::rng()).copied().copied()
                    }
            } else {
                players_to_convert.choose(&mut rand::rng()).copied().copied()
            }
        ) else {return};
        Self::give_gun(game, target);
    }

    pub fn give_gun_to_insider(game: &mut Game){
        if game.syndicate_gun_item.player_with_gun.is_some_and(|p|
            p.alive(game) && 
            InsiderGroupID::Mafia.is_player_in_revealed_group(game, p)
        ) {return}
        game.syndicate_gun_item.player_with_gun = None;

        for insider in InsiderGroupID::Mafia.players(game).clone() {
            insider.remove_player_tag_on_all(game, Tag::SyndicateGun);
        }
        let Some(insider) = InsiderGroupID::Mafia.players(game)
            .iter()
            .find(|p|p.alive(game))
            .copied() else {return};
        SyndicateGunItem::give_gun(game, insider);
    } 

    pub fn on_midnight(game: &mut Game, _event: &OnMidnight, _fold: &mut (), priority: OnMidnightPriority) {
        if game.day_number() <= 1 {return}
        match priority {
            OnMidnightPriority::TopPriority => {
                let Some(player_with_gun) = game.syndicate_gun_item.player_with_gun else {return}; 

                let Some(PlayerListSelection(gun_target)) = game.saved_controllers
                    .get_controller_current_selection_player_list(ControllerID::syndicate_gun_item_shoot()) else {return};
                let Some(gun_target) = gun_target.first() else {return};

                NightVisits::add_visit(
                    game, 
                    Visit::new(player_with_gun, *gun_target, true, VisitTag::SyndicateGunItem)
                );
            }
            OnMidnightPriority::Kill => {
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
        if game.syndicate_gun_item.player_with_gun.is_none_or(|p|p != actor_ref) {return}

        let Some(PlayerListSelection(target)) = ability_input
            .get_player_list_selection_if_id(ControllerID::SyndicateGunItemGive)
        else {return};
        let Some(target) = target.first() else {return};

        if actor_ref != *target &&
            target.alive(game) &&
            InsiderGroupID::Mafia.is_player_in_revealed_group(game, *target) 
        {
            SyndicateGunItem::give_gun(game, *target);
        }
    }
}