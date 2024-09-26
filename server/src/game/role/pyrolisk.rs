use std::collections::HashSet;

use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::ChatMessageVariant;
use crate::game::event::before_role_switch::BeforeRoleSwitch;
use crate::game::grave::{GraveInformation, GraveKiller, GraveReference};
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::tag::Tag;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{GetClientRoleState, Priority, Role, RoleState, RoleStateImpl};

#[derive(Debug, Clone, Default)]
pub struct Pyrolisk{
    pub tagged_for_obscure: HashSet<PlayerReference>
}

#[derive(Debug, Clone, Serialize)]
pub struct ClientRoleState;

pub(super) const FACTION: Faction = Faction::Fiends;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Pyrolisk {
    type ClientRoleState = ClientRoleState;
    type RoleActionChoice = super::common_role::CommonRoleActionChoice;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        let mut tagged_for_obscure = self.tagged_for_obscure.clone();
        
        match priority {
            Priority::Kill => {
                if game.day_number() != 1 {
                    if let Some(visit) = actor_ref.night_visits(game).first(){
                        let target_ref = visit.target;
                        target_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Pyrolisk), AttackPower::ArmorPiercing, true);
                        
                        tagged_for_obscure.insert(target_ref);
                        actor_ref.push_player_tag(game, target_ref, Tag::MorticianTagged);
                    }

                    for other_player_ref in actor_ref.all_visitors(game)
                        .into_iter().filter(|other_player_ref|
                            other_player_ref.alive(game) &&
                            *other_player_ref != actor_ref
                        ).collect::<Vec<PlayerReference>>()
                    {
                        other_player_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Pyrolisk), AttackPower::ArmorPiercing, true);
                        
                        tagged_for_obscure.insert(other_player_ref);
                        actor_ref.push_player_tag(game, other_player_ref, Tag::MorticianTagged);
                    }
                }else{
                    if let Some(visit) = actor_ref.night_visits(game).first(){
                        let target_ref = visit.target;

                        tagged_for_obscure.insert(target_ref);
                        actor_ref.push_player_tag(game, target_ref, Tag::MorticianTagged);
                    }

                    for other_player_ref in actor_ref.all_visitors(game)
                        .into_iter().filter(|other_player_ref|
                            other_player_ref.alive(game) &&
                            *other_player_ref != actor_ref
                        ).collect::<Vec<PlayerReference>>()
                    {
                        tagged_for_obscure.insert(other_player_ref);
                        actor_ref.push_player_tag(game, other_player_ref, Tag::MorticianTagged);
                    }
                }
            }
            _=>{}
        }

        actor_ref.set_role_state(game, RoleState::Pyrolisk(Pyrolisk{tagged_for_obscure}));
    }
    
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref)
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, true)
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
        let mut tagged_for_obscure = self.tagged_for_obscure.clone();
        tagged_for_obscure.insert(actor_ref);
        actor_ref.push_player_tag(game, actor_ref, Tag::MorticianTagged);
        actor_ref.set_role_state(game, RoleState::Pyrolisk(Pyrolisk{tagged_for_obscure}));
    }
    fn on_grave_added(self, game: &mut Game, actor_ref: PlayerReference, grave_ref: GraveReference){

        let should_obscure = 
        //if they are tagged for obscure
        if let RoleState::Pyrolisk(Pyrolisk{tagged_for_obscure}) = actor_ref.role_state(game) {
            tagged_for_obscure.contains(&grave_ref.deref(game).player)
        }else{false};

        if should_obscure {
            actor_ref.add_private_chat_message(game, ChatMessageVariant::PlayerRoleAndAlibi{
                player: grave_ref.deref(game).player,
                role: grave_ref.deref(game).player.role(game),
                will: grave_ref.deref(game).player.will(game).to_string(),
            });

            grave_ref.deref_mut(game).information = GraveInformation::Obscured;
        }
    }
    fn before_role_switch(self, game: &mut Game, actor_ref: PlayerReference, event: BeforeRoleSwitch) {
        if event.player() == actor_ref && event.new_role().role() != Role::Mortician {
            actor_ref.remove_player_tag_on_all(game, Tag::MorticianTagged);
        }
    }
}
impl GetClientRoleState<ClientRoleState> for Pyrolisk {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}