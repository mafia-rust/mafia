use crate::game::{chat::ChatGroup, player::{PlayerIndex, PlayerReference}, Game, visit::Visit, team::Team, role_list::Faction};


pub(super) fn can_night_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    
    actor_index != target_index &&
    game.get_unchecked_player(actor_index).chosen_targets().len() < 1 &&
    *game.get_unchecked_player(actor_index).alive() &&
    *game.get_unchecked_player(target_index).alive() &&
    !Team::same_team(
        game.get_unchecked_player(actor_index).role(), 
        game.get_unchecked_player(target_index).role()
    )
}

pub(super) fn convert_targets_to_visits(game: &Game, actor_ref: PlayerReference, targets: Vec<PlayerReference>, astral: bool, attack: bool) -> Vec<Visit> {
    if targets.len() > 0{
        vec![Visit{ target: targets[0], astral, attack }]
    }else{
        Vec::new()
    }
}

pub(super) fn get_current_send_chat_groups(game: &Game, actor_index: PlayerReference, night_chat_groups: Vec<ChatGroup>) -> Vec<ChatGroup> {
    if !game.get_unchecked_player(actor_index).alive(){
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
pub(super) fn get_current_recieve_chat_groups(game: &Game, actor_index: PlayerReference) -> Vec<ChatGroup> {
    let player = game.get_unchecked_player(actor_index);

    let mut out = Vec::new();

    out.push(ChatGroup::All);

    if !game.get_unchecked_player(actor_index).alive(){
        out.push(ChatGroup::Dead);
    }

    if game.get_unchecked_player(actor_index).role().faction_alignment().faction() == Faction::Mafia {
        out.push(ChatGroup::Mafia);
    }
    if game.get_unchecked_player(actor_index).role().faction_alignment().faction() == Faction::Coven {
        out.push(ChatGroup::Coven);
    }

    out
}


pub(super) fn on_role_creation(actor_index: PlayerIndex, game: &mut Game){

    let actor_role = game.get_unchecked_mut_player(actor_index).role();

    //set a role tag for themselves
    game.get_unchecked_mut_player(actor_index).insert_role_label(actor_index, actor_role);

    //if they are on a team. set tags for their teammates
    for other_index in 0..(game.players.len() as PlayerIndex){
        if actor_index == other_index{
            continue;
        }
        let other_role = game.get_unchecked_mut_player(other_index).role();

        if Team::same_team(actor_role, other_role) {
            game.get_unchecked_mut_player(other_index).insert_role_label(actor_index, actor_role);
        }
    }
}




