use std::collections::HashSet;

use serde::Serialize;

use crate::game::chat::ChatMessageVariant;
use crate::game::player_group::PlayerGroup;
use crate::game::grave::{GraveInformation, GraveKiller, GraveReference};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::tag::Tag;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, Role, RoleState, RoleStateImpl};

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Pyrolisk{
    pub tagged_for_obscure: HashSet<PlayerReference>
}

pub(super) const FACTION: Faction = Faction::Fiends;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: u8 = 1;

impl RoleStateImpl for Pyrolisk {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        let mut tagged_for_obscure = self.tagged_for_obscure.clone();
        
        match priority {
            Priority::Kill => {
                if game.day_number() != 1 {
                    if let Some(visit) = actor_ref.night_visits(game).first(){
                        let target_ref = visit.target;
                        target_ref.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Pyrolisk), 2, true);
                        
                        tagged_for_obscure.insert(target_ref);
                        actor_ref.push_player_tag(game, target_ref, Tag::MorticianTagged);
                    }

                    for other_player_ref in actor_ref.all_visitors(game)
                        .into_iter().filter(|other_player_ref|
                            other_player_ref.alive(game) &&
                            *other_player_ref != actor_ref
                        ).collect::<Vec<PlayerReference>>()
                    {
                        other_player_ref.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Pyrolisk), 2, true);
                        
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
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target: PlayerReference) -> bool {
        false
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, true)
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<PlayerGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<PlayerGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType){
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
        let mut tagged_for_obscure = self.tagged_for_obscure.clone();
        tagged_for_obscure.insert(actor_ref);
        actor_ref.push_player_tag(game, actor_ref, Tag::MorticianTagged);
        actor_ref.set_role_state(game, RoleState::Pyrolisk(Pyrolisk{tagged_for_obscure}));
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
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
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}