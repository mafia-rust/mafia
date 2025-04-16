use rand::seq::IndexedRandom;
use serde::Serialize;

use crate::game::ability_input::ControllerParametersMap;
use crate::game::event::on_midnight::{MidnightVariables, OnMidnightPriority};
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::Game;
use super::{common_role, ControllerID, GetClientRoleState, Role, RoleStateImpl};

#[derive(Clone, Debug)]
pub struct Armorsmith {
    open_shops_remaining: u8,
    night_open_shop: bool,

    protected_visiting_players: Vec<PlayerReference>,
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
            protected_visiting_players: Vec::new(),
            players_armor: Vec::new()
        }
    }
}


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Armorsmith {
    type ClientRoleState = ClientRoleState;
    fn on_midnight(mut self, game: &mut Game, midnight_variables: &mut MidnightVariables, actor_ref: PlayerReference, priority: OnMidnightPriority) {

        match priority {
            OnMidnightPriority::Heal => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                if let Some(visit) = actor_visits.first() {
                    if self.open_shops_remaining != 0 {
                        self.night_open_shop = true;
                        self.open_shops_remaining = self.open_shops_remaining.saturating_sub(1);

                        actor_ref.increase_defense_to(game, midnight_variables, DefensePower::Protection);


                        let visitors = actor_ref.all_night_visitors_cloned(game);

                        for visitor in visitors.iter(){
                            visitor.increase_defense_to(game, midnight_variables, DefensePower::Protection);
                        }
                        if visitors.contains(&visit.target){
                            self.players_armor.push(visit.target);
                        }else if let Some(random_visitor) = visitors.choose(&mut rand::rng()) {
                            self.players_armor.push(*random_visitor);
                        }

                        self.protected_visiting_players = visitors;
                    }
                };

                for player in self.players_armor.iter(){
                    player.increase_defense_to(game, midnight_variables, DefensePower::Protection);
                }

                actor_ref.set_role_state(game, self);
            }
            OnMidnightPriority::Investigative => {
                for protected_player in self.protected_visiting_players.iter(){
                    if protected_player.night_attacked(midnight_variables){
                        actor_ref.push_night_message(midnight_variables, ChatMessageVariant::TargetWasAttacked);
                        protected_player.push_night_message(midnight_variables, ChatMessageVariant::YouWereProtected);
                    }
                }

                for player_armor in self.players_armor.clone() {
                    if player_armor.night_attacked(midnight_variables){
                        actor_ref.push_night_message(midnight_variables, ChatMessageVariant::TargetWasAttacked);
                        player_armor.push_night_message(midnight_variables, ChatMessageVariant::YouWereProtected);

                        player_armor.push_night_message(midnight_variables, ChatMessageVariant::ArmorsmithArmorBroke);
                        self.players_armor.retain(|x| *x != player_armor);
                    }
                }

                actor_ref.set_role_state(game, self);
            }
            _ => {}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::Armorsmith, 0))
            .single_player_selection_typical(actor_ref, false, true)
            .night_typical(actor_ref)
            .add_grayed_out_condition(self.open_shops_remaining == 0)
            .build_map()
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<crate::game::visit::Visit> {
        common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Armorsmith, 0),
            false
        )
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(game, 
            Armorsmith{
                night_open_shop: false,
                protected_visiting_players: Vec::new(),
                ..self
            });
    }
    fn new_state(game: &Game) -> Self {
        Self{
            open_shops_remaining: game.num_players().div_ceil(5),
            ..Self::default()
        }
    }
}
impl GetClientRoleState<ClientRoleState> for Armorsmith {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState {
            open_shops_remaining: self.open_shops_remaining
        }
    }
}