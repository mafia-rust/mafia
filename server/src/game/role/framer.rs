use serde::Serialize;

use crate::game::chat::ChatMessageVariant;
use crate::game::components::detained::Detained;
use crate::game::{attack_power::DefensePower, player::PlayerReference};

use crate::game::visit::Visit;

use crate::game::Game;
use super::{RevealedGroupID, Priority, Role, RoleStateImpl};


#[derive(Clone, Debug, Default, Serialize)]
pub struct Framer;



pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Framer {
    type ClientRoleState = Framer;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {

        match priority {
            Priority::Deception => {
                let framer_visits = actor_ref.night_visits(game).clone();

                let Some(first_visit) = framer_visits.first() else {return};

                first_visit.target.set_night_framed(game, true);

                let Some(second_visit) = framer_visits.get(1) else {return};
            
                if !Detained::is_detained(game, first_visit.target) {
                    first_visit.target.set_night_appeared_visits(game, Some(vec![
                        Visit{ target: second_visit.target, attack: false }
                    ]));
                }

                actor_ref.set_night_visits(game, vec![first_visit.clone()]);
            },
            Priority::Investigative => {

                if actor_ref.alive(game) && actor_ref.night_blocked(game) {return;}

                let mut chat_messages = Vec::new();

                for player in PlayerReference::all_players(game){
                    if !RevealedGroupID::players_in_same_revealed_group(game, actor_ref, player) {continue;}

                    let visitors_roles: Vec<Role> = PlayerReference::all_appeared_visitors(player, game)
                        .iter()
                        .filter(|player|
                            player.win_condition(game)
                                .requires_only_this_resolution_state(crate::game::game_conclusion::GameConclusion::Town)
                        )
                        .map(|player| player.role(game))
                        .collect();


                    chat_messages.push(ChatMessageVariant::FramerResult{mafia_member: player.index(), visitors: visitors_roles});
                }

                for player in PlayerReference::all_players(game){
                    if !RevealedGroupID::players_in_same_revealed_group(game, actor_ref, player) {continue;}
                    for msg in chat_messages.iter(){
                        player.push_night_message(game, msg.clone());
                    }
                }
            },
            _ => {}
        }
        
    
        
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        
        !crate::game::components::detained::Detained::is_detained(game, actor_ref) &&
        actor_ref.alive(game) &&
        (
            actor_ref.selection(game).is_empty() &&
            actor_ref != target_ref &&
            target_ref.alive(game) &&
            !RevealedGroupID::players_in_same_revealed_group(game, actor_ref, target_ref)
        ) || 
        (
            actor_ref.selection(game).len() == 1
        )
        
    }
    fn convert_selection_to_visits(self, _game: &Game, _actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        if target_refs.len() == 2 {
            vec![
                Visit{ target: target_refs[0], attack: false }, 
                Visit{ target: target_refs[1], attack: false }
            ]
        } else if target_refs.len() == 1 {
            vec![
                Visit{ target: target_refs[0], attack: false }
            ]
        } else {
            Vec::new()
        }
    }
    fn default_revealed_groups(self) -> std::collections::HashSet<crate::game::components::revealed_group::RevealedGroupID> {
        vec![
            crate::game::components::revealed_group::RevealedGroupID::Mafia
        ].into_iter().collect()
    }
}
