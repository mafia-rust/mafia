use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleStateImpl};


#[derive(Debug, Clone, Serialize)]
pub struct Wizard{
    level: u8,
    current_spell: Spell
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Spell{
    None, //default 

    //Level 1

    Meditate, //Level up
    Hex, //Hex two players to die at the end of the game if only ones alive
    Poison, //Poison someone to attack 3 days later 
    Shield, //Protect yourself until attacked
    Invisibility, //Gives innocent aura and visits target

    //Level 3

    Absorb, //Protects self and levels up for each attack blocked
    Reflect, //reverses visits
    Medusa, //All visitors cleaned and attacked
    Clarity, //Shows all players who are a specific role
    Polymorph, //Silence and roleblock a player

    //Level 5

    Smite, //protection-piercing attack
    Lightning, //Attacks if create circut

    //Level 7

    //Level 9
    Ascend, //Announces that wizard has ascended and will win at end of night if not executed. Hexed players can not vote. requires high level
    
    
    

}

impl Default for Wizard {
    fn default() -> Self {
        Self { level: 0, current_spell: Spell::None }
    }
}

pub(super) const FACTION: Faction = Faction::Neutral;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(0);

impl RoleStateImpl for Wizard {
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {Some(Team::Mafia)}


    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {

        match self.current_spell{
            _ => {}
        }


        if priority != Priority::Kill {return}
        
        if let Some(visit) = actor_ref.night_visits(game).first(){
            let target_ref = visit.target;
            if target_ref.night_jailed(game){
                actor_ref.push_night_message(game, ChatMessage::TargetJailed);
                return
            }
    
            target_ref.try_night_kill(actor_ref, game, GraveKiller::Faction(Faction::Mafia), 1, true);
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
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![ChatGroup::Mafia])
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