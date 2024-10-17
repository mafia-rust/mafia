use rand::thread_rng;
use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{GetClientRoleState, Priority, RoleStateImpl};

#[derive(Clone, Debug)]
pub struct Armorsmith {
    open_shops_remaining: u8,
    night_open_shop: bool,
    night_protected_players: Vec<PlayerReference>,
    players_armor: Vec<PlayerReference>
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleState {
    open_shops_remaining: u8
}

impl Default for Armorsmith {
    fn default() -> Self {
        Self { 
            open_shops_remaining: 2,
            night_open_shop: false,
            night_protected_players: Vec::new(),
            players_armor: Vec::new()
        }
    }
}


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Armorsmith {
    type ClientRoleState = ClientRoleState;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::TopPriority => {
                if let Some(_) = actor_ref.night_visits(game).first(){
                    if self.open_shops_remaining > 0 {
                        actor_ref.set_role_state(game, 
                            Armorsmith {
                                night_open_shop: true,
                                ..self
                            }
                        );
                        actor_ref.set_night_visits(game, vec![]);
                    }
                }
            }
            Priority::Armorsmith => {
                match (self.night_open_shop, actor_ref.night_blocked(game)){
                    (true, true) => {
                        actor_ref.set_role_state(game, 
                            Armorsmith {
                                night_open_shop: false,
                                ..self
                            }
                        );
                    },
                    (true, false) => {
                        actor_ref.set_role_state(game, 
                            Armorsmith {
                                open_shops_remaining: self.open_shops_remaining.saturating_sub(1),
                                ..self
                            }
                        );
                    },
                    (false, true) => {},
                    (false, false) => {},
                }
            }
            Priority::Heal => {
                for player in self.players_armor.iter(){
                    player.increase_defense_to(game, DefensePower::Protection);
                }

                if self.night_open_shop {
                    actor_ref.increase_defense_to(game, DefensePower::Protection);

                    let visitors = actor_ref.all_visitors(game);

                    for visitor in visitors.iter(){
                        visitor.increase_defense_to(game, DefensePower::Protection);
                    }

                    if let Some(random_visitor) = visitors.choose(&mut thread_rng()) {
                        self.players_armor.push(random_visitor.clone());
                    }

                    actor_ref.set_role_state(game, Armorsmith{
                        night_protected_players: visitors,
                        ..self
                    });
                }
            }
            Priority::Investigative => {

                for protected_player in self.night_protected_players.iter(){
                    if protected_player.night_attacked(game){
                        actor_ref.push_night_message(game, ChatMessageVariant::TargetWasAttacked);
                        protected_player.push_night_message(game, ChatMessageVariant::YouWereProtected);
                    }
                }

                for player_armor in self.players_armor.clone().into_iter(){
                    if player_armor.night_attacked(game){
                        actor_ref.push_night_message(game, ChatMessageVariant::TargetWasAttacked);
                        player_armor.push_night_message(game, ChatMessageVariant::YouWereProtected);

                        player_armor.push_night_message(game, ChatMessageVariant::ArmorsmithArmorBroke);
                        self.players_armor.retain(|x| *x != player_armor);
                    }
                }

                actor_ref.set_role_state(game, Armorsmith{
                    ..self
                });
            }
            _ => {}
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        actor_ref == target_ref &&
        self.open_shops_remaining > 0 &&
        !crate::game::components::detained::Detained::is_detained(game, actor_ref) &&
        actor_ref.selection(game).is_empty() &&
        actor_ref.alive(game)
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(game, 
            Armorsmith{
                night_open_shop: false,
                night_protected_players: Vec::new(),
                ..self
            });
    }
}
impl GetClientRoleState<ClientRoleState> for Armorsmith {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState {
            open_shops_remaining: self.open_shops_remaining
        }
    }
}