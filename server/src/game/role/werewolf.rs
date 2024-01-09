use std::collections::HashSet;

use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::FactionAlignment;
use crate::game::tag::Tag;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleStateImpl, Role, RoleState};


#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Werewolf{
    pub tracked_players: Vec<PlayerReference>,
}

pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::NeutralKilling;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;

impl RoleStateImpl for Werewolf {
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {1}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {None}

    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        

        match priority {
            Priority::Kill => {

                if game.day_number() == 1 || game.day_number() == 3 {
                    return;
                }

                match actor_ref.night_visits(game).first() {
                    //rampage at target
                    Some(first_visit) => {
                        let target_ref = first_visit.target;
                        if target_ref.night_jailed(game){
                            actor_ref.push_night_message(game, ChatMessage::TargetJailed);
                            return
                        }
                        

                        for other_player_ref in 
                            target_ref.veteran_seen_players(game).into_iter().filter(|p|actor_ref!=*p)
                            .collect::<Vec<PlayerReference>>()
                        {
                            other_player_ref.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Werewolf), 2, true);
                        }
                        target_ref.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Werewolf), 2, true);
                    },



                    //rampage at home
                    None => {
                        if actor_ref.night_jailed(game){
                            //kill all jailors NOT trying to execute me
                            for jailor_ref in PlayerReference::all_players(game){
                                if 
                                    jailor_ref.alive(game) && 
                                    jailor_ref.role(game) == Role::Jailor &&
                                    jailor_ref.night_visits(game).into_iter().all(|visit|visit.target!=actor_ref)
                                {
                                    jailor_ref.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Werewolf), 2, true);
                                }
                            }
                        }else{
                            for other_player_ref in 
                                actor_ref.veteran_seen_players(game).into_iter().filter(|p|actor_ref!=*p)
                                .collect::<Vec<PlayerReference>>()
                            {
                                other_player_ref.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Werewolf), 2, true);
                            }
                        }
                    },
                }
            },
            Priority::Investigative => {
                
                

                //on night 1 and 3, werewolf can track the scent of players who visit them and their target
                if game.day_number() == 1 || game.day_number() == 3 {
                    

                    let mut tracked_players: Vec<PlayerReference> = actor_ref.veteran_seen_players(game).into_iter().filter(|p|actor_ref!=*p).collect();
                
                    if let Some(first_visit) = actor_ref.night_visits(game).first() {
                        let target_ref = first_visit.target;

                        if target_ref.night_jailed(game){
                            actor_ref.push_night_message(game, ChatMessage::TargetJailed);
                        }else{
                            tracked_players.push(target_ref);
                        }
                    }

                    //this should remove duplicates
                    tracked_players.append(&mut self.tracked_players.clone());
                    let tracked_players: HashSet<PlayerReference> = tracked_players.into_iter().collect();
                    let tracked_players: Vec<PlayerReference> = tracked_players.into_iter().collect();

                    actor_ref.remove_player_tag_on_all(game, Tag::WerewolfTracked);

                    //send the list to the werewolf using tags
                    for player_ref in tracked_players.iter() {
                        actor_ref.push_player_tag(game, *player_ref, Tag::WerewolfTracked);
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
                    actor_ref.push_night_message(game, 
                        ChatMessage::WerewolfTrackingResult{
                            tracked_player: player_ref.index(), 
                            players: player_ref.tracker_seen_visits(game).into_iter().map(|p|p.target.index()).collect()
                        }
                    );
                });
            },
            _ => {}
        }
    }
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_target(game, actor_ref, target_ref)
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, true)
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType){
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}