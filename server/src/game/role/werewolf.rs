use std::collections::HashSet;

use rand::seq::SliceRandom;

use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::ChatMessageVariant;
use crate::game::components::night_visits::NightVisits;
use crate::game::grave::GraveKiller;
use crate::game::player::{PlayerIndex, PlayerReference};

use crate::game::tag::Tag;
use crate::game::visit::Visit;
use crate::game::phase::PhaseType;

use crate::game::Game;
use super::{ControllerID, ControllerParametersMap, PlayerListSelection, GetClientRoleState, Priority, Role, RoleState, RoleStateImpl};


#[derive(Clone, Debug, Default)]
pub struct Werewolf{
    pub tracked_players: Vec<PlayerReference>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ClientRoleState;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Werewolf {
    type ClientRoleState = ClientRoleState;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Kill => {

                if game.day_number() == 1 {
                    return;
                }

                match actor_ref.untagged_night_visits_cloned(game).first() {
                    Some(first_visit) => {
                        let target_ref = first_visit.target;                        

                        

                        //If player is tracked, check if they are visiting, then attack them
                        if self.tracked_players.contains(&target_ref) {
                            if target_ref.untagged_night_visits(game).len() > 0 {
                                actor_ref.try_night_kill_single_attacker(target_ref, game, GraveKiller::Role(Role::Werewolf), AttackPower::ArmorPiercing, true);
                            }
                            else {
                            //rampage target    
                                for other_player_ref in 
                                target_ref.all_night_visitors_cloned(game).into_iter().filter(|p|actor_ref!=*p)
                                .collect::<Vec<PlayerReference>>() 
                                {
                                other_player_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Werewolf), AttackPower::ArmorPiercing, true);
                                }
                            }
                        }
                        else {
                            
                            //If player is not tracked, track them instead
                            if self.tracked_players.contains(&target_ref) {
                                return;
                            }
                            let mut tracked_players = self.tracked_players.clone();
                            tracked_players.push(target_ref);
                            actor_ref.set_role_state(game, RoleState::Werewolf(Werewolf {
                                tracked_players
                            }));
                            
                        }
                    },

                    //rampage at home
                    None => {
                        //for other_player_ref in 
                        //    actor_ref.all_night_visitors_cloned(game).into_iter().filter(|p|actor_ref!=*p)
                        //    .collect::<Vec<PlayerReference>>()
                        //{
                        //    other_player_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Werewolf), AttackPower::ArmorPiercing, true);
                        //}
                        
                    },
                }
            },
            Priority::Investigative => {

                //let mut newly_tracked_players: Vec<PlayerReference> = actor_ref.all_night_visitors_cloned(game).into_iter().filter(|p|actor_ref!=*p).collect();
            
                //if let Some(first_visit) = actor_ref.untagged_night_visits_cloned(game).first() {
                //    let target_ref = first_visit.target;
                    
                //    newly_tracked_players.push(target_ref);
                //}

                //this should remove duplicates
                //newly_tracked_players.append(&mut self.tracked_players.clone());
                //let tracked_players: HashSet<PlayerReference> = newly_tracked_players.into_iter().collect();
                //let tracked_players: Vec<PlayerReference> = tracked_players.into_iter().collect();

                //send the list to the werewolf using tags
                //for player_ref in tracked_players.iter() {
                //    if actor_ref.player_has_tag(game, *player_ref, Tag::WerewolfTracked) == 0 {
                //        actor_ref.push_player_tag(game, *player_ref, Tag::WerewolfTracked);
                //    }
                //}

                //actor_ref.set_role_state(game, RoleState::Werewolf(Werewolf {
                //    tracked_players
                //}));

                

                //track the scent of players
                let RoleState::Werewolf(werewolf) = actor_ref.role_state(game) else {
                    unreachable!("Werewolf role state should be Werewolf")
                };
                let tracked_players = werewolf.tracked_players.clone();
                tracked_players.into_iter().for_each(|player_ref|{

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
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Werewolf, 0),
            true
        )
    }

    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        match phase {
            PhaseType::Night => {

                //Mark chosen player as tracked on phase start: night
                let Some(PlayerListSelection(target)) = game.saved_controllers.get_controller_current_selection_player_list(
                    ControllerID::role(actor_ref, Role::Werewolf, 0)
                ) else {return};
                let Some(target) = target.first() else {return};

                if actor_ref.ability_deactivated_from_death(game) || !target.alive(game) {return};
                actor_ref.push_player_tag(game, *target, Tag::WerewolfTracked);
                self.tracked_players.push(*target);
                let tracked_players = self.tracked_players.clone();
                actor_ref.set_role_state(game, self);

                //Send tracked message to all tracked players
                for player in tracked_players {
                    player.add_private_chat_message(game, ChatMessageVariant::WerewolfTracked{ target: player });

                    
                }
            },
            _ => {}
        }
    }
}
impl GetClientRoleState<ClientRoleState> for Werewolf {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}