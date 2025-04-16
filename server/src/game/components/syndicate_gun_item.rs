
use crate::{game::{
    ability_input::*, attack_power::AttackPower, event::{on_add_insider::OnAddInsider, on_midnight::{OnMidnight, OnMidnightPriority}, on_remove_insider::OnRemoveInsider}, grave::GraveKiller, phase::PhaseType, player::PlayerReference, role_list::RoleSet, visit::{Visit, VisitTag}, Game
}, vec_set};

use super::{detained::Detained, insider_group::InsiderGroupID, night_visits::NightVisits, tags::Tags};

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

    pub fn give_gun_to_player(game: &mut Game, player: PlayerReference) {
        Self::remove_gun(game);
        game.syndicate_gun_item.player_with_gun = Some(player);
        
        Tags::add_tag(game, super::tags::TagSetID::SyndicateGun, player);
    }
    pub fn remove_gun(game: &mut Game) {
        game.syndicate_gun_item.player_with_gun = None;

        Tags::set_tagged(game, super::tags::TagSetID::SyndicateGun, &vec_set![]);
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
    pub fn on_add_insider(game: &mut Game, _event: &OnAddInsider, _fold: &mut (), _priority: ()){
        Tags::set_viewers(game, super::tags::TagSetID::SyndicateGun, &InsiderGroupID::Mafia.players(game).clone());
    }
    pub fn on_remove_insider(game: &mut Game, _event: &OnRemoveInsider, _fold: &mut (), _priority: ()){
        Tags::set_viewers(game, super::tags::TagSetID::SyndicateGun, &InsiderGroupID::Mafia.players(game).clone());
    }
    pub fn on_any_death(game: &mut Game, player: PlayerReference) {
        if game.syndicate_gun_item.player_with_gun.is_some_and(|p|p==player) {
            Self::remove_gun(game);

            let player = InsiderGroupID::Mafia.players(game).iter().find(|p|p.alive(game));
            if let Some(player) = player {
                SyndicateGunItem::give_gun_to_player(game, *player);
            }
        }
    }
    pub fn on_midnight(game: &mut Game, _event: &OnMidnight, _fold: &mut (), priority: OnMidnightPriority) {
        if game.day_number() <= 1 {return}
        match priority {
            OnMidnightPriority::TopPriority => {
                let Some(player_with_gun) = game.syndicate_gun_item.player_with_gun else {return}; 

                let Some(PlayerListSelection(gun_target)) = ControllerID::syndicate_gun_item_shoot().get_player_list_selection(game) else {return};
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
            SyndicateGunItem::give_gun_to_player(game, *target);
        }
    }
}