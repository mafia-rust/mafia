use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::ability_input::ControllerParametersMap;
use crate::game::attack_power::AttackPower;
use crate::game::chat::ChatMessageVariant;
use crate::game::event::on_midnight::OnMidnightPriority;
use crate::game::event::on_whisper::{OnWhisper, WhisperFold, WhisperPriority};
use crate::game::{attack_power::DefensePower, grave::GraveKiller};
use crate::game::player::{PlayerIndex, PlayerReference};

use crate::game::visit::Visit;

use crate::game::Game;
use super::{common_role, ControllerID, Role, RoleStateImpl};


#[derive(Debug, Clone, Serialize, Default)]
pub struct Ojo;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Ojo {
    type ClientRoleState = Ojo;
    fn on_midnight(self, game: &mut Game, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        match priority {
            OnMidnightPriority::Kill => {
                if game.day_number() == 1 {return}
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                if let Some(visit) = actor_visits.first(){
                    let target_ref = visit.target;
                    
                    target_ref.try_night_kill_single_attacker(
                        actor_ref, game, 
                        GraveKiller::Role(Role::Ojo), 
                        AttackPower::ArmorPiercing, 
                        true,
                        true
                    );
                }
            },
            OnMidnightPriority::Investigative => {
                PlayerReference::all_players(game)
                    .for_each(|player_ref|{

                    let mut players: Vec<PlayerIndex> = player_ref.all_night_visits_cloned(game).into_iter().map(|p|p.target.index()).collect();
                    players.shuffle(&mut rand::rng());

                    actor_ref.push_night_message(game, 
                        ChatMessageVariant::WerewolfTrackingResult{
                            tracked_player: player_ref.index(), 
                            players
                        }
                    );
                });
            }
            _ => ()
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::Ojo, 0))
            .single_player_selection_typical(actor_ref, false, false)
            .night_typical(actor_ref)
            .add_grayed_out_condition(game.day_number() <= 1)
            .build_map()  
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Ojo, 0),
            true
        )
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {
        PlayerReference::all_players(game).for_each(|p|actor_ref.insert_role_label(game, p));
    }
    fn on_remove_role_label(self, game: &mut Game, actor_ref: PlayerReference, player: PlayerReference, concealed_player: PlayerReference){
        if player != actor_ref {return};

        actor_ref.insert_role_label(game, concealed_player);
    }
    fn on_whisper(self, game: &mut Game, actor_ref: PlayerReference, event: &OnWhisper, fold: &mut WhisperFold, priority: WhisperPriority) {
        if priority == WhisperPriority::Send && !fold.cancelled && event.receiver != actor_ref && event.sender != actor_ref {
            actor_ref.add_private_chat_message(game, ChatMessageVariant::Whisper {
                from_player_index: event.sender.into(),
                to_player_index: event.receiver.into(),
                text: event.message.clone()
            });
        }
    }
}