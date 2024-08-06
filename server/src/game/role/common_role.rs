use crate::game::{player_group::PlayerGroup, player::PlayerReference, Game, visit::Visit, role_list::Faction, phase::{PhaseState, PhaseType}, resolution_state::ResolutionState};

use super::{journalist::Journalist, medium::Medium, same_evil_team, RoleState};


pub(super) fn can_night_select(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    
    actor_ref != target_ref &&
    !actor_ref.night_jailed(game) &&
    actor_ref.selection(game).is_empty() &&
    actor_ref.alive(game) &&
    target_ref.alive(game) &&
    !same_evil_team(game, actor_ref, target_ref)
}

pub(super) fn convert_selection_to_visits(_game: &Game, _actor_ref: PlayerReference, target_refs: Vec<PlayerReference>, attack: bool) -> Vec<Visit> {
    if !target_refs.is_empty() {
        vec![Visit{ target: target_refs[0], attack }]
    } else {
        Vec::new()
    }
}

pub(super) fn get_current_send_chat_groups(game: &Game, actor_ref: PlayerReference, night_chat_groups: Vec<PlayerGroup>) -> Vec<PlayerGroup> {
    if !actor_ref.alive(game){
        return vec![PlayerGroup::Dead];
    }
    if actor_ref.night_silenced(game){
        return vec![];
    }

    match game.current_phase() {
        PhaseState::Briefing |
        PhaseState::Obituary => vec![],
        PhaseState::Discussion 
        | PhaseState::Nomination {..}
        | PhaseState::Judgement {..} 
        | PhaseState::FinalWords {..}
        | PhaseState::Dusk => vec![PlayerGroup::All],
        &PhaseState::Testimony { player_on_trial, .. } => {
            if player_on_trial == actor_ref {
                vec![PlayerGroup::All]
            } else {
                vec![]
            }
        },
        PhaseState::Night => {
            let mut out = vec![];
            if PlayerReference::all_players(game)
                .any(|med|{
                    match med.role_state(game) {
                        RoleState::Medium(Medium{ seanced_target: Some(seanced_target), .. }) => {
                            actor_ref == *seanced_target
                        },
                        _ => false
                    }
                })
            {
                out.push(PlayerGroup::Dead);
            }
            if PlayerReference::all_players(game)
                .any(|p|
                    match p.role_state(game) {
                        RoleState::Journalist(Journalist{interviewed_target: Some(interviewed_target_ref), ..}) => {
                            *interviewed_target_ref == actor_ref
                        },
                        _ => false
                    }
                )
            {
                out.push(PlayerGroup::Interview);
            }


            let mut jail_or_night_chats = if actor_ref.night_jailed(game){
                vec![PlayerGroup::Jail]
            } else {
                night_chat_groups
            };


            out.append(&mut jail_or_night_chats);
            out
        },
    }
}
pub(super) fn get_current_receive_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<PlayerGroup> {
    let mut out = Vec::new();

    out.push(PlayerGroup::All);

    if !actor_ref.alive(game){
        out.push(PlayerGroup::Dead);
    }

    if actor_ref.role(game).faction() == Faction::Mafia {
        out.push(PlayerGroup::Mafia);
    }
    if actor_ref.role(game).faction() == Faction::Cult {
        out.push(PlayerGroup::Cult);
    }
    if actor_ref.night_jailed(game){
        out.push(PlayerGroup::Jail);
    }
    if 
        game.current_phase().phase() == PhaseType::Night && 
        PlayerReference::all_players(game)
            .any(|med|{
                match med.role_state(game) {
                    RoleState::Medium(Medium{ seanced_target: Some(seanced_target), .. }) => {
                        actor_ref == *seanced_target
                    },
                    _ => false
                }
            })
    {
        out.push(PlayerGroup::Dead);
    }
    if 
        game.current_phase().phase() == PhaseType::Night && 
        PlayerReference::all_players(game)
            .any(|p|
                match p.role_state(game) {
                    RoleState::Journalist(Journalist{interviewed_target: Some(interviewed_target_ref), ..}) => {
                        *interviewed_target_ref == actor_ref
                    },
                    _ => false
                }
            )
    {
        out.push(PlayerGroup::Interview);
    }

    out
}

///Only works for roles that win based on end game condition
pub(super) fn get_won_game(game: &Game, actor_ref: PlayerReference) -> bool {
    if let Some(end_game_condition) = ResolutionState::game_is_over(game) {
        ResolutionState::can_win_with(game, actor_ref, end_game_condition)
    } else {
        false
    }
}