use crate::game::{chat::ChatGroup, player::PlayerReference, Game, visit::Visit, role_list::Faction, phase::{PhaseState, PhaseType}, team::Team};

use super::RoleState;


pub(super) fn can_night_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    
    actor_ref != target_ref &&
    !actor_ref.night_jailed(game) &&
    actor_ref.chosen_targets(game).is_empty() &&
    actor_ref.alive(game) &&
    target_ref.alive(game) &&
    !Team::same_team(actor_ref.role(game), target_ref.role(game))
}
pub(super) fn convert_targets_to_visits(_game: &Game, _actor_ref: PlayerReference, target_refs: Vec<PlayerReference>, astral: bool, attack: bool) -> Vec<Visit> {
    if !target_refs.is_empty() {
        vec![Visit{ target: target_refs[0], astral, attack }]
    } else {
        Vec::new()
    }
}

pub(super) fn get_current_send_chat_groups(game: &Game, actor_ref: PlayerReference, night_chat_groups: Vec<ChatGroup>) -> Vec<ChatGroup> {
    if !actor_ref.alive(game){
        return vec![ChatGroup::Dead];
    }
    if actor_ref.night_silenced(game){
        return vec![];
    }

    match game.current_phase() {
        PhaseState::Morning => vec![],
        PhaseState::Discussion 
        | PhaseState::Voting {..}
        | PhaseState::Judgement {..} 
        | PhaseState::Evening {..} => vec![ChatGroup::All],
        &PhaseState::Testimony { player_on_trial, .. } => {
            if player_on_trial == actor_ref {
                vec![ChatGroup::All]
            } else {
                vec![]
            }
        },
        PhaseState::Night => {
            let mut out = vec![];
            if PlayerReference::all_players(game).into_iter()
                .any(|med|{
                    if let RoleState::Medium(medium_state) = med.role_state(game){
                        if Some(actor_ref) == medium_state.seanced_target{
                            return true;
                        }
                    }
                    false
                })
            {
                out.push(ChatGroup::Seance);
            }


            let mut jail_or_night_chats = if actor_ref.night_jailed(game){
                vec![ChatGroup::Jail]
            } else {
                night_chat_groups
            };
            out.append(&mut jail_or_night_chats);
            out
        },
    }
}
pub(super) fn get_current_recieve_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    let mut out = Vec::new();

    out.push(ChatGroup::All);

    if !actor_ref.alive(game){
        out.push(ChatGroup::Dead);
    }

    if actor_ref.role(game).faction_alignment().faction() == Faction::Mafia {
        out.push(ChatGroup::Mafia);
    }
    if actor_ref.role(game).faction_alignment().faction() == Faction::Coven {
        out.push(ChatGroup::Coven);
    }
    if actor_ref.night_jailed(game){
        out.push(ChatGroup::Jail);
    }
    if
        game.current_phase().phase() == PhaseType::Night &&
        PlayerReference::all_players(game).into_iter()
            .any(|med|{
                if let RoleState::Medium(medium_state) = med.role_state(game){
                    if Some(actor_ref) == medium_state.seanced_target{
                        return true;
                    }
                }
                false
            })
    {
        out.push(ChatGroup::Seance);
    }

    out
}


pub(super) fn on_role_creation(game: &mut Game, actor_ref: PlayerReference){

    let actor_role = actor_ref.role(game);

    //set a role tag for themselves
    actor_ref.insert_role_label(game, actor_ref, actor_role);

    //if they are on a team. set labels for their teammates, and my label for my teammates
    for other_ref in PlayerReference::all_players(game){
        if actor_ref == other_ref{
            continue;
        }
        let other_role = other_ref.role(game);
        
        if Team::same_team(actor_role, other_role) {
            other_ref.insert_role_label(game, actor_ref, actor_role);
            actor_ref.insert_role_label(game, other_ref, other_role);
        }
    }
}




