use crate::game::{chat::ChatGroup, player::PlayerIndex, Game, visit::Visit, team::Team};


pub(super) fn can_night_target(actor_index: PlayerIndex, target_index: PlayerIndex, game: &Game) -> bool {
    
    actor_index != target_index &&
    game.get_unchecked_player(actor_index).night_variables.chosen_targets.len() < 1 &&
    game.get_unchecked_player(actor_index).alive &&
    game.get_unchecked_player(target_index).alive &&
    (
        game.get_unchecked_player(actor_index).get_role().get_team() == None ||
        (
            game.get_unchecked_player(actor_index).get_role().get_team() != game.get_unchecked_player(target_index).get_role().get_team()
        )
    )
    
}

pub(super) fn convert_targets_to_visits(actor_index: PlayerIndex, targets: Vec<PlayerIndex>, game: &Game, astral: bool, attack: bool) -> Vec<Visit> {
    if targets.len() > 0{
        vec![Visit{ target: targets[0], astral: false, attack: false }]
    }else{
        Vec::new()
    }
}

pub(super) fn get_current_send_chat_groups(actor_index: PlayerIndex, game: &Game, night_chat_groups: Vec<ChatGroup>) -> Vec<ChatGroup> {
    if !game.get_unchecked_player(actor_index).alive{
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

pub(super) fn on_role_creation(actor_index: PlayerIndex, game: &mut Game){

    let actor_role = game.get_unchecked_mut_player(actor_index).get_role();


    //set a role tag for themselves
    game.get_unchecked_mut_player(actor_index).role_labels.insert(actor_index, actor_role);

    //if they are on a team. set tags for their teammates
    if actor_role.get_team().is_some(){

        for other_index in 0..(game.players.len() as PlayerIndex){
            if actor_index == other_index {
                continue;
            }
            let other_role = game.get_unchecked_mut_player(other_index).get_role();

            if actor_role.get_team() == other_role.get_team() {
                game.get_unchecked_mut_player(actor_index).role_labels.insert(other_index, other_role);
            }
        }
    }
}




