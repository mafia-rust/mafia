use std::vec;

use serde::{Serialize, Deserialize};

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::FactionAlignment;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::jester::Jester;
use super::{Priority, RoleStateImpl, Role, RoleState};

#[derive(Clone, Debug, Serialize, Default)]
pub struct Doomsayer {
    pub guesses: [(PlayerReference, DoomsayerGuess); 3]
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum DoomsayerGuess{
    Mafia, Coven, #[default] Neutral,

    Jailor, Mayor, Transporter, 
    //No TI
    Doctor, Bodyguard, Crusader,
    Vigilante, Veteran, Deputy,
    Escort, Medium, Retributionist,
}
impl DoomsayerGuess{
    fn convert_to_guess(role: Role)->Option<DoomsayerGuess>{
        match role {
            Role::Jailor => Some(DoomsayerGuess::Jailor),
            Role::Mayor => Some(DoomsayerGuess::Mayor),
            Role::Transporter => Some(DoomsayerGuess::Transporter),
            Role::Sheriff | Role::Lookout | Role::Spy | Role::Tracker | Role::Seer => None, 
            Role::Doctor => Some(DoomsayerGuess::Doctor),
            Role::Bodyguard => Some(DoomsayerGuess::Bodyguard),
            Role::Crusader => Some(DoomsayerGuess::Crusader),
            Role::Vigilante => Some(DoomsayerGuess::Vigilante),
            Role::Veteran => Some(DoomsayerGuess::Veteran),
            Role::Deputy => Some(DoomsayerGuess::Deputy),
            Role::Escort => Some(DoomsayerGuess::Escort),
            Role::Medium => Some(DoomsayerGuess::Medium),
            Role::Retributionist => Some(DoomsayerGuess::Retributionist),
            Role::Mafioso | Role::Consort | Role::Blackmailer | Role::Consigliere | Role::Janitor | Role::Framer => Some(DoomsayerGuess::Mafia),
            Role::Witch => Some(DoomsayerGuess::Coven),
            Role::Jester | Role::Executioner | Role::Doomsayer | Role::Vampire => Some(DoomsayerGuess::Neutral),
        }
    }
    fn guess_matches_role(&self, role: Role)->bool{
        if let Some(guess) = Self::convert_to_guess(role) {
            *self == guess
        }else{
            false
        }
    }
}

pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::NeutralEvil;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);

impl RoleStateImpl for Doomsayer {
    fn suspicious(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    fn control_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {true}
    fn roleblock_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn end_game_condition(&self, _game: &Game, _actor_ref: PlayerReference) -> EndGameCondition {EndGameCondition::None}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {None}


    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::TopPriority {return;}

        if actor_ref.night_jailed(game) {return;}
        if actor_ref.night_roleblocked(game) {return;}
        if !actor_ref.alive(game) {return;}

        if self.guesses.iter().all(
            |(player, guess)|{
                guess.guess_matches_role(player.role(game)) &&
                *player != actor_ref &&
                player.alive(game)
            }
        ){
            actor_ref.add_chat_message(game, ChatMessage::DoomsayerWon);
            self.guesses[0].0.try_night_kill(actor_ref, game, GraveKiller::Role(super::Role::Doomsayer), 3);
            self.guesses[1].0.try_night_kill(actor_ref, game, GraveKiller::Role(super::Role::Doomsayer), 3);
            self.guesses[2].0.try_night_kill(actor_ref, game, GraveKiller::Role(super::Role::Doomsayer), 3);
            actor_ref.try_night_kill(actor_ref, game, GraveKiller::Suicide, 3);
        }else{
            actor_ref.add_chat_message(game, ChatMessage::DoomsayerFailed);
        }
    
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    }
    fn can_night_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, _game: &Game, _actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        vec![]
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_recieve_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
    }
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType) {
        
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {
        Doomsayer::check_and_convert_to_jester(game, actor_ref);
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
        Doomsayer::check_and_convert_to_jester(game, actor_ref);
    }
}
impl Doomsayer{
    pub fn check_and_convert_to_jester(game: &mut Game, actor_ref: PlayerReference){
        if 
            PlayerReference::all_players(game).into_iter().filter(|player|
                player.alive(game) && DoomsayerGuess::convert_to_guess(player.role(game)).is_some() && *player != actor_ref
            ).collect::<Vec<PlayerReference>>().len() < 3
        {
            actor_ref.set_role(game, RoleState::Jester(Jester::default()));
        }
    }
}