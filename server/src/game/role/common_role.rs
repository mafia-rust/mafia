use crate::game::{chat::ChatGroup, player::PlayerReference, Game, visit::Visit, team::Team, role_list::Faction, phase::PhaseState};

use super::{Role, RoleState, coven_leader::CovenLeader, voodoo_master::VoodooMaster};


pub(super) fn can_night_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    
    actor_ref != target_ref &&
    !actor_ref.night_jailed(game) &&
    actor_ref.chosen_targets(game).is_empty() &&
    actor_ref.alive(game) &&
    target_ref.alive(game) &&
    !Team::same_team(
        actor_ref.role(game), 
        target_ref.role(game)
    )
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
        | PhaseState::Evening 
        | PhaseState::FinalWords {..} => vec![ChatGroup::All],
        &PhaseState::Testimony { player_on_trial, .. } => {
            if player_on_trial == actor_ref {
                vec![ChatGroup::All]
            } else {
                vec![]
            }
        },
        PhaseState::Night => {
            if actor_ref.night_jailed(game){
                vec![ChatGroup::Jail]
            } else {
                night_chat_groups
            }
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
    if actor_ref.role(game) == Role::Jailor || actor_ref.night_jailed(game){
        out.push(ChatGroup::Jail);
    }

    out
}


pub(super) fn on_role_creation(game: &mut Game, actor_ref: PlayerReference){

    let actor_role = actor_ref.role(game);

    //set a role tag for themselves
    actor_ref.insert_role_label(game, actor_ref, actor_role);

    //if they are on a team. set tags for their teammates
    for other_ref in PlayerReference::all_players(game){
        if actor_ref == other_ref{
            continue;
        }
        let other_role = other_ref.role(game);

        if Team::same_team(actor_role, other_role) {
            other_ref.insert_role_label(game, actor_ref, actor_role);
        }
    }
}



impl RoleState {
    pub fn has_necronomicon(&self)->bool{
        match self {
            RoleState::CovenLeader(CovenLeader { necronomicon }) => *necronomicon,
            RoleState::VoodooMaster(VoodooMaster { necronomicon }) => *necronomicon,
            _ => false
        }
    }
}


