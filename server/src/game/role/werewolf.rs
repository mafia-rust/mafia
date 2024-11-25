use std::collections::HashSet;

use rand::thread_rng;
use rand::seq::SliceRandom;

use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::ChatMessageVariant;
use crate::game::grave::GraveKiller;
use crate::game::player::{PlayerIndex, PlayerReference};

use crate::game::tag::Tag;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{GetClientRoleState, Priority, Role, RoleState, RoleStateImpl};


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

                if game.day_number() == 1 || game.day_number() == 3 {
                    return;
                }

                match actor_ref.night_visits_cloned(game).first() {
                    //rampage at target
                    Some(first_visit) => {
                        let target_ref = first_visit.target;                        

                        for other_player_ref in 
                            target_ref.all_visitors(game).into_iter().filter(|p|actor_ref!=*p)
                            .collect::<Vec<PlayerReference>>()
                        {
                            other_player_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Werewolf), AttackPower::ArmorPiercing, true);
                        }
                        target_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Werewolf), AttackPower::ArmorPiercing, true);
                    },



                    //rampage at home
                    None => {
                        for other_player_ref in 
                            actor_ref.all_visitors(game).into_iter().filter(|p|actor_ref!=*p)
                            .collect::<Vec<PlayerReference>>()
                        {
                            other_player_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Werewolf), AttackPower::ArmorPiercing, true);
                        }
                        
                    },
                }
            },
            Priority::Investigative => {
                
                

                //on night 1 and 3, werewolf can track the scent of players who visit them and their target
                if game.day_number() == 1 || game.day_number() == 3 {
                    

                    let mut newly_tracked_players: Vec<PlayerReference> = actor_ref.all_visitors(game).into_iter().filter(|p|actor_ref!=*p).collect();
                
                    if let Some(first_visit) = actor_ref.night_visits_cloned(game).first() {
                        let target_ref = first_visit.target;
                        
                        newly_tracked_players.push(target_ref);
                    }

                    //this should remove duplicates
                    newly_tracked_players.append(&mut self.tracked_players.clone());
                    let tracked_players: HashSet<PlayerReference> = newly_tracked_players.into_iter().collect();
                    let tracked_players: Vec<PlayerReference> = tracked_players.into_iter().collect();

                    //send the list to the werewolf using tags
                    for player_ref in tracked_players.iter() {
                        if actor_ref.player_has_tag(game, *player_ref, Tag::WerewolfTracked) == 0 {
                            actor_ref.push_player_tag(game, *player_ref, Tag::WerewolfTracked);
                        }
                    }

                    actor_ref.set_role_state(game, RoleState::Werewolf(Werewolf {
                        tracked_players
                    }));

                }

                //track the scent of players
                let RoleState::Werewolf(werewolf) = actor_ref.role_state(game) else {
                    unreachable!("Werewolf role state should be Werewolf")
                };
                let tracked_players = werewolf.tracked_players.clone();
                tracked_players.into_iter().for_each(|player_ref|{

                    let mut players: Vec<PlayerIndex> = player_ref.tracker_seen_visits(game).into_iter().map(|p|p.target.index()).collect();
                    players.shuffle(&mut thread_rng());

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
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref)
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, true)
    }
}
impl GetClientRoleState<ClientRoleState> for Werewolf {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}