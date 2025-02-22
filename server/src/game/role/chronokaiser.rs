use serde::Serialize;

use crate::game::chat::ChatGroup;
use crate::game::phase::PhaseType;
use crate::game::Game;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;

use super::RoleStateImpl;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Chronokaiser;

pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Chronokaiser {
    type ClientRoleState = Chronokaiser;
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){

        if actor_ref.ability_deactivated_from_death(game) {
            return;
        }

        if phase == PhaseType::Discussion && Self::get_speed_up_percent(game) != 0{
            game.add_message_to_chat_group(ChatGroup::All, 
                ChatMessageVariant::ChronokaiserSpeedUp { percent: Self::get_speed_up_percent(game)}
            );
        }

        let new_speed_ratio = (Self::get_speed_up_percent(game) + 100) as f32 / 100.0;
        game.phase_machine.time_remaining = game.phase_machine.time_remaining.div_f32(new_speed_ratio);
    }
}

impl Chronokaiser {
    const SPEED_UP_PERCENT_PER_DAY: u32 = 60;
    pub fn get_speed_up_percent(game: &Game)->u32{
        game.day_number().saturating_sub(1) as u32 * Self::SPEED_UP_PERCENT_PER_DAY
    }
    pub fn won(game: &Game, actor_ref: PlayerReference)->bool{
        actor_ref.alive(game)
    }
}