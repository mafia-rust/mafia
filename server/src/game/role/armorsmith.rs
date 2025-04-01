use rand::seq::IndexedRandom;
use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::ability_input::ControllerParametersMap;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::Game;
use super::{common_role, ControllerID, GetClientRoleState, Priority, Role, RoleState, RoleStateImpl};

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
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(visit) = actor_visits.first() else {return};
                let target = visit.target;

                if self.open_shops_remaining == 0 {return}
                    
                self.night_open_shop = true;
                self.open_shops_remaining = self.open_shops_remaining.saturating_sub(1);


                actor_ref.increase_defense_to(game, DefensePower::Protection);

                let visitors = actor_ref.all_night_visitors_cloned(game);

                for visitor in visitors.iter(){
                    visitor.increase_defense_to(game, DefensePower::Protection);
                }

                if visitors.contains(&target){
                    self.players_armor.push(target);
                }else if let Some(random_visitor) = visitors.choose(&mut rand::rng()) {
                    self.players_armor.push(*random_visitor);
                }

                self.night_protected_players = visitors;

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

                for player_armor in self.players_armor.clone() {
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
                night_protected_players: Vec::new(),
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

impl Armorsmith {
    pub fn has_armorsmith_armor(player: PlayerReference, game: &Game) -> bool {
        for other in PlayerReference::all_players(game){
            if let RoleState::Armorsmith(state) = other.role_state(game) {
                if state.players_armor.contains(&player) {
                    return true;
                }
            }
        }
        false
    }
    /// Should not be run during the do_night_action events
    /// Attack power is only used for checking if the armorsmith should be told their target was protected
    pub fn break_armor_day(game: &mut Game, player: PlayerReference, attack_power: AttackPower) -> bool {
        let mut had_armor: bool = false;
        for smith in PlayerReference::all_players(game){
            let RoleState::Armorsmith(state) = smith.role_state(game) else {continue};
            if state.players_armor.contains(&player) {
                had_armor = true;
                smith.set_role_state(game, RoleState::Armorsmith(Armorsmith { 
                    open_shops_remaining: state.open_shops_remaining, 
                    night_open_shop: state.night_open_shop, 
                    night_protected_players: state.night_protected_players.clone(), 
                    players_armor: state.players_armor.iter().filter(|p|**p!=player).copied().collect(),
                }));
                player.add_private_chat_message(game, ChatMessageVariant::ArmorsmithArmorBroke);
                if !attack_power.can_pierce(DefensePower::Protection) {
                    smith.add_private_chat_message(game, ChatMessageVariant::TargetWasAttacked);
                }
            }
        }
        had_armor
    }
}