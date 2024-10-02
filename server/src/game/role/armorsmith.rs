use std::vec;

use rand::thread_rng;
use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{GetClientRoleState, Priority, RoleStateImpl};

#[derive(Clone, Debug)]
pub struct Armorsmith {
    open_shops_remaining: u8,
    night_open_shop: bool,
    night_protected_players: Vec<PlayerReference>,
    players_armor: Vec<PlayerReference>,
    
    night_selection: <Self as RoleStateImpl>::RoleActionChoice,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleState {
    open_shops_remaining: u8,
    night_selection: <Armorsmith as RoleStateImpl>::RoleActionChoice,
}

impl Default for Armorsmith {
    fn default() -> Self {
        Self { 
            open_shops_remaining: 2,
            night_open_shop: false,
            night_protected_players: Vec::new(),
            players_armor: Vec::new(),
            night_selection: <Self as RoleStateImpl>::RoleActionChoice::default(),
        }
    }
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Armorsmith {
    type ClientRoleState = ClientRoleState;
    type RoleActionChoice = super::common_role::RoleActionChoiceBool;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Armorsmith => {
                if self.open_shops_remaining > 0 && self.night_selection.boolean{
                    actor_ref.set_role_state(game, 
                        Armorsmith {
                            open_shops_remaining: self.open_shops_remaining.saturating_sub(1),
                            night_open_shop: true,
                            night_protected_players: Vec::new(),
                            ..self
                        }
                    );
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
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != PhaseType::Night {return};
        if !crate::game::role::common_role::default_action_choice_boolean_is_valid(game, actor_ref) {return}
        if self.open_shops_remaining == 0 {return}
        self.night_selection = action_choice;
        actor_ref.set_role_state(game, self);
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        vec![]
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        match phase {
            PhaseType::Night => {
                actor_ref.set_role_state(game, Armorsmith{
                    night_open_shop: false,
                    night_protected_players: Vec::new(),
                    night_selection: <Self as RoleStateImpl>::RoleActionChoice::default(),
                    ..self
                });
            }
            _ => {}
        }
            
    }
}
impl GetClientRoleState<ClientRoleState> for Armorsmith {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState {
            open_shops_remaining: self.open_shops_remaining,
            night_selection: self.night_selection,
        }
    }
}