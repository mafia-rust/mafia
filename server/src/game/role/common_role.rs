use crate::game::{chat::ChatGroup, player::{PlayerIndex, PlayerReference}, Game, visit::Visit, team::Team, role_list::Faction};


pub(super) fn can_night_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    
    actor_ref != target_ref &&
    actor_ref.deref(game).chosen_targets().len() < 1 &&
    *actor_ref.deref(game).alive() &&
    *target_ref.deref(game).alive() &&
    !Team::same_team(
        actor_ref.deref(game).role(), 
        target_ref.deref(game).role()
    )
}

pub(super) fn convert_targets_to_visits(game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>, astral: bool, attack: bool) -> Vec<Visit> {
    if target_refs.len() > 0{
        vec![Visit{ target: target_refs[0], astral, attack }]
    }else{
        Vec::new()
    }
}

pub(super) fn get_current_send_chat_groups(game: &Game, actor_ref: PlayerReference, night_chat_groups: Vec<ChatGroup>) -> Vec<ChatGroup> {
    if !actor_ref.deref(game).alive(){
        return vec![ChatGroup::Dead];
    }

    match game.phase_machine.current_state {
        crate::game::phase::PhaseType::Morning => vec![],
        crate::game::phase::PhaseType::Discussion => vec![ChatGroup::All],
        crate::game::phase::PhaseType::Voting => vec![ChatGroup::All],
        crate::game::phase::PhaseType::Testimony => {if game.player_on_trial == Some(actor_index) {vec![ChatGroup::All]} else {vec![]}},
        crate::game::phase::PhaseType::Judgement => vec![ChatGroup::All],
        crate::game::phase::PhaseType::Evening => vec![ChatGroup::All],
        crate::game::phase::PhaseType::Night => night_chat_groups,
    }
}
pub(super) fn get_current_recieve_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    let mut out = Vec::new();

    out.push(ChatGroup::All);

    if !actor_ref.deref(game).alive(){
        out.push(ChatGroup::Dead);
    }

    if actor_ref.deref(game).role().faction_alignment().faction() == Faction::Mafia {
        out.push(ChatGroup::Mafia);
    }
    if actor_ref.deref(game).role().faction_alignment().faction() == Faction::Coven {
        out.push(ChatGroup::Coven);
    }

    out
}


pub(super) fn on_role_creation(game: &mut Game, actor_ref: PlayerReference){

    let actor_role = actor_ref.deref(game).role();

    //set a role tag for themselves
    actor_ref.deref(game).insert_role_label(*actor_ref.index(), actor_role);

    //if they are on a team. set tags for their teammates
    for other_ref in PlayerReference::all_players(game){
        if actor_ref == other_ref{
            continue;
        }
        let other_role = other_ref.deref(game).role();

        if Team::same_team(actor_role, other_role) {
            other_ref.deref_mut(game).insert_role_label(*actor_ref.index(), actor_role);
        }
    }
}




