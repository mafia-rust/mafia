use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::ChatMessageVariant;
use crate::game::components::poison::{Poison, PoisonAlert, PoisonObscure};
use crate::game::grave::{GraveInformation, GraveKiller, GraveReference};
use crate::game::player::PlayerReference;

use crate::game::tag::Tag;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::vec_set::VecSet;

use super::{GetClientRoleState, Priority, Role, RoleState, RoleStateImpl};

#[derive(Debug, Clone, Default)]
pub struct Spiral{
    pub spiraling: VecSet<PlayerReference>
}

#[derive(Debug, Clone, Serialize)]
pub struct ClientRoleState;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Spiral {
    type ClientRoleState = ClientRoleState;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        let mut new_spiraling = self.spiraling.clone();

        if priority != Priority::Poison { return };
        
        if self.spiraling.is_empty() {
            if let Some(visit) = actor_ref.night_visits(game).first(){
                let target_ref = visit.target;
                
                Spiral::start_player_spiraling(game, &mut new_spiraling, actor_ref, target_ref);
            }
        } else {
            for spiraling_player in self.spiraling.clone() {
                for other_player_ref in spiraling_player.all_visitors(game)
                    .into_iter().filter(|other_player_ref|
                        other_player_ref.alive(game) &&
                        *other_player_ref != actor_ref // Let doctor self-heal
                    ).collect::<Vec<PlayerReference>>()
                {
                    Spiral::start_player_spiraling(game, &mut new_spiraling, actor_ref, other_player_ref);
                }
            }
        }

        actor_ref.set_role_state(game, RoleState::Spiral(Spiral{spiraling: new_spiraling}));
    }
    
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref) && self.spiraling.is_empty()
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, true)
    }
    fn on_grave_added(self, game: &mut Game, actor_ref: PlayerReference, grave_ref: GraveReference){
        let mut new_spiraling = self.spiraling.clone();
        let dead_ref = grave_ref.deref(game).player;

        if self.spiraling.contains(&dead_ref) {
            actor_ref.add_private_chat_message(game, ChatMessageVariant::PlayerRoleAndAlibi{
                player: grave_ref.deref(game).player,
                role: grave_ref.deref(game).player.role(game),
                will: grave_ref.deref(game).player.will(game).to_string(),
            });

            grave_ref.deref_mut(game).information = GraveInformation::Obscured;

            actor_ref.remove_player_tag(game, dead_ref, Tag::Spiraling);
            new_spiraling.remove(&dead_ref);

            actor_ref.set_role_state(game, RoleState::Spiral(Spiral{spiraling: new_spiraling}));
        }
    }
    fn before_role_switch(self, game: &mut Game, actor_ref: PlayerReference, player: PlayerReference, _old: RoleState, _new: RoleState) {
        if player == actor_ref {
            actor_ref.remove_player_tag_on_all(game, Tag::Spiraling);
        }
    }
}

impl Spiral {
    fn start_player_spiraling(game: &mut Game, new_spiraling: &mut VecSet<PlayerReference>, actor_ref: PlayerReference, target_ref: PlayerReference) {
        let mut attackers = VecSet::new();
        attackers.insert(actor_ref);

        Poison::poison_player(game, target_ref, 
            AttackPower::ArmorPiercing, 
            GraveKiller::Role(Role::Spiral), 
            attackers, 
            false, 
            PoisonAlert::NoAlert,
            PoisonObscure::Obscured
        );

        new_spiraling.insert(target_ref);
        actor_ref.push_player_tag(game, target_ref, Tag::Spiraling);
    }
}

impl GetClientRoleState<ClientRoleState> for Spiral {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}