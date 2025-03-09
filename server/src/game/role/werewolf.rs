use rand::seq::SliceRandom;

use serde::Serialize;

use crate::game::ability_input::{AbilitySelection, AvailableAbilitySelection};
use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::ChatMessageVariant;
use crate::game::components::night_visits::NightVisits;
use crate::game::grave::GraveKiller;
use crate::game::player::{PlayerIndex, PlayerReference};

use crate::game::tag::Tag;
use crate::game::visit::{Visit, VisitTag};
use crate::game::phase::PhaseType;

use crate::game::Game;
use crate::vec_set::{vec_set, VecSet};
use super::{ControllerID, ControllerParametersMap, PlayerListSelection, GetClientRoleState, Priority, Role, RoleStateImpl};


#[derive(Clone, Debug, Default)]
pub struct Werewolf{
    pub tracked_players: VecSet<PlayerReference>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ClientRoleState;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

const ENRAGED_PERCENT: f32 = 2f32/3f32;

impl RoleStateImpl for Werewolf {
    type ClientRoleState = ClientRoleState;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            //priority completely burgered so sammy told me to make my own priority but i didn't want to so i just made it heal
            Priority::Heal => {
                let visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(first_visit) = visits.first() else {return};

                let target_ref = first_visit.target;
                let enraged = self.tracked_players.len() as f32 >= ENRAGED_PERCENT * PlayerReference::all_players(game)
                    .filter(|p|p.alive(game)||*p==actor_ref)
                    .count() as f32;

                if !enraged && target_ref.all_night_visits_cloned(game).is_empty() {return}
                    
                NightVisits::all_visits_mut(game)
                    .into_iter()
                    .filter(|visit| 
                        visit.target == target_ref && visit.visitor == actor_ref && visit.tag == VisitTag::Role  
                    ).for_each(|visit| {
                        visit.attack = true;
                    });
            }
            Priority::Kill => {
                let visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(first_visit) = visits.first() else {return};
                let target_ref = first_visit.target;

                //If player is untracked, track them
                if !self.tracked_players.contains(&target_ref) {

                    self.track_player(game, actor_ref, target_ref);

                    actor_ref.set_role_state(game, self);
                    
                } else {
                    //Dont attack or rampage first night
                    if game.day_number() <= 1 {return}
                
                    //rampage target
                    for other_player in NightVisits::all_visits(game).into_iter()
                        .filter(|visit|
                            *first_visit != **visit &&
                            visit.target == target_ref
                        )
                        .map(|v|v.visitor)
                        .collect::<Vec<_>>()
                    {
                        other_player.try_night_kill_single_attacker(
                            actor_ref,
                            game,
                            GraveKiller::Role(Role::Werewolf),
                            AttackPower::ArmorPiercing,
                            true
                        );
                    }
                    
                    //If target visits, attack them
                    if first_visit.attack {
                        target_ref.try_night_kill_single_attacker(
                            actor_ref,
                            game,
                            GraveKiller::Role(Role::Werewolf),
                            AttackPower::ArmorPiercing,
                            true
                        );
                    } 
                }
                
            },
            Priority::Investigative => {
                //track sniffed players visits

                self.tracked_players
                    .into_iter()
                    .for_each(|player_ref|{

                    let mut players: Vec<PlayerIndex> = player_ref.tracker_seen_visits(game).into_iter().map(|p|p.target.index()).collect();
                    players.shuffle(&mut rand::rng());

                    actor_ref.push_night_message(game, 
                        ChatMessageVariant::WerewolfTrackingResult{
                            tracked_player: player_ref.index(), 
                            players
                        }
                    );
                });
            },
            _ => {}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            true,
            false,
            ControllerID::role(actor_ref, Role::Werewolf, 0)
        ).combine_overwrite_owned(
            ControllerParametersMap::new_controller_fast(
                game,
                ControllerID::role(actor_ref, Role::Werewolf, 1),
                AvailableAbilitySelection::new_player_list(
                    PlayerReference::all_players(game)
                        .into_iter()
                        .filter(|player|
                            player.alive(game) && *player != actor_ref
                        )
                        .collect(),
                        false,
                        Some(1)
                    ),
                AbilitySelection::new_player_list(Vec::new()),
                actor_ref.ability_deactivated_from_death(game),
                Some(PhaseType::Night),
                false,
                vec_set!(actor_ref)
            )
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Werewolf, 0),
            false
        )
    }

    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        match phase {
            PhaseType::Night => {

                //Mark chosen player as tracked on phase start: night
                if let Some(PlayerListSelection(target)) = game.saved_controllers.get_controller_current_selection_player_list(
                    ControllerID::role(actor_ref, Role::Werewolf, 1)
                ) {
                    if let Some(target) = target.first() {
                        self.track_player(game, actor_ref, *target);
                    };
                };

                for player in self.tracked_players.iter() {
                    player.add_private_chat_message(game, ChatMessageVariant::WerewolfTracked);
                }
                actor_ref.set_role_state(game, self);
            },
            _ => {}
        }
    }
}
impl Werewolf{
    fn track_player(&mut self, game: &mut Game, actor: PlayerReference, target: PlayerReference){
        if self.tracked_players.contains(&target){return}
        actor.push_player_tag(game, target, Tag::WerewolfTracked);
        self.tracked_players.insert(target);
    }
}
impl GetClientRoleState<ClientRoleState> for Werewolf {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}