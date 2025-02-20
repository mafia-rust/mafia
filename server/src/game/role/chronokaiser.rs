use serde::Serialize;

use crate::game::chat::ChatGroup;
use crate::game::components::confused::Confused;
use crate::game::phase::PhaseType;
use crate::game::Game;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;

use super::{RoleState, RoleStateImpl};

#[derive(Clone, Debug, Serialize)]
pub struct Chronokaiser {
    pub seconds_per_decrement: f32,
}

impl Default for Chronokaiser {
    fn default() -> Self {
        Chronokaiser{seconds_per_decrement: 1f32}
    }
}

pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Chronokaiser {
    type ClientRoleState = Chronokaiser;
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){

        if actor_ref.ability_deactivated_from_death(game) {
            return;
        }
        let time_reduction_percent = Self::get_time_reduction_percent(game);
        let mut seconds_per_decrement: f32 = self.seconds_per_decrement;

        if phase == PhaseType::Discussion && Self::get_time_reduction_percent(game) != 0{
            game.add_message_to_chat_group(ChatGroup::All, 
                ChatMessageVariant::ChronokaiserSpeedUp { percent: time_reduction_percent}
            );
            if Confused::is_confused_not_possess_confused(game, actor_ref) {                
                seconds_per_decrement *= (time_reduction_percent - Self::TIME_REDUCTION_PERCENT_PER_DAY) as f32 / time_reduction_percent as f32
            };
        }
        
        let new_total_time_ratio = (time_reduction_percent + 100) as f32 / 100.0;
        game.phase_machine.time_remaining = game.phase_machine.time_remaining.div_f32(new_total_time_ratio);
        
        game.phase_machine.seconds_per_decrement *= seconds_per_decrement;

        actor_ref.set_role_state(game, RoleState::Chronokaiser(Chronokaiser{
            seconds_per_decrement
        }));
            
    }
}

impl Chronokaiser {
    const TIME_REDUCTION_PERCENT_PER_DAY: u32 = 60;

    pub fn get_time_reduction_percent(game: &Game)->u32{
        game.day_number().saturating_sub(1) as u32 * Self::TIME_REDUCTION_PERCENT_PER_DAY
    }
    pub fn won(game: &Game, actor_ref: PlayerReference)->bool{
        actor_ref.alive(game)
    }
}