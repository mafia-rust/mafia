use rand::thread_rng;
use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::Game;
use super::{common_role, BooleanSelection, ControllerID, GetClientRoleState, Priority, Role, RoleStateImpl};

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
            open_shops_remaining: 3,
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
            Priority::Heal => {

                let selected_to_open_shop = if let Some(BooleanSelection(true)) =
                    game.saved_controllers.get_controller_current_selection_boolean(
                        ControllerID::role(actor_ref, Role::Armorsmith, 0)
                    ){
                        true
                    }else{
                        false
                    };
                
                if
                    !actor_ref.night_blocked(game) &&
                    (self.open_shops_remaining > 0) &&
                    selected_to_open_shop
                {
                    self.night_open_shop = true;
                    self.open_shops_remaining = self.open_shops_remaining.saturating_sub(1);


                    actor_ref.increase_defense_to(game, DefensePower::Protection);

                    let visitors = actor_ref.all_night_visitors_cloned(game);

                    for visitor in visitors.iter(){
                        visitor.increase_defense_to(game, DefensePower::Protection);
                    }

                    if let Some(random_visitor) = visitors.choose(&mut thread_rng()) {
                        self.players_armor.push(random_visitor.clone());
                    }

                    self.night_protected_players = visitors;
                }


                for player in self.players_armor.iter(){
                    player.increase_defense_to(game, DefensePower::Protection);
                }

                actor_ref.set_role_state(game, self);
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

                actor_ref.set_role_state(game, self);
            }
            _ => {}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        common_role::controller_parameters_map_boolean(
            game,
            actor_ref,
            self.open_shops_remaining > 0,
            ControllerID::role(actor_ref, Role::Armorsmith, 0)
        )
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