use std::collections::HashSet;

use crate::game::{
    chat::ChatGroup, components::{detained::Detained, generic_ability::{GenericAbilityID, GenericAbilitySaveComponent, GenericAbilitySelectionType}, puppeteer_marionette::PuppeteerMarionette}, game_conclusion::GameConclusion, modifiers::{ModifierType, Modifiers}, phase::{PhaseState, PhaseType}, player::PlayerReference, role_list::RoleSet, visit::Visit, win_condition::WinCondition, Game
};

use super::{reporter::Reporter, medium::Medium, InsiderGroupID, Role, RoleState};



pub(super) fn can_night_select(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    
    actor_ref != target_ref &&
    !Detained::is_detained(game, actor_ref) &&
    actor_ref.selection(game).is_empty() &&
    actor_ref.alive(game) &&
    target_ref.alive(game) &&
    !InsiderGroupID::in_same_revealed_group(game, actor_ref, target_ref)
}

pub(super) fn convert_selection_to_visits(_game: &Game, _actor_ref: PlayerReference, target_refs: Vec<PlayerReference>, attack: bool) -> Vec<Visit> {
    if !target_refs.is_empty() {
        vec![Visit{ target: target_refs[0], attack }]
    } else {
        Vec::new()
    }
}

pub(super) fn convert_generic_ability_to_visits(game: &Game, actor_ref: PlayerReference, ability_id: GenericAbilityID, attack: bool) -> Vec<Visit> {
    GenericAbilitySaveComponent::get_saved_input(game, actor_ref, ability_id)
        .map(|selection|
            match selection {
                GenericAbilitySelectionType::UnitSelection => vec![Visit{ target: actor_ref, attack }],
                GenericAbilitySelectionType::OnePlayerOptionSelection { selection } => match selection.0 {
                    Some(target_ref) => vec![Visit{ target: target_ref, attack }],
                    None => Vec::new(),
                },
                GenericAbilitySelectionType::TwoRoleOptionSelection { selection } => {
                    let mut out = Vec::new();
                    for player in PlayerReference::all_players(game){
                        if Some(player.role(game)) == selection.0 {
                            out.push(Visit{ target: player, attack });
                        }else if Some(player.role(game)) == selection.1 {
                            out.push(Visit{ target: player, attack });
                        }
                    }
                    out
                }
                GenericAbilitySelectionType::TwoRoleOutlineOptionSelection { selection } => {
                    let mut out = vec![];
                    if let Some(chosen_outline) = selection.0{
                        let (_, player) = chosen_outline.deref_as_role_and_player_originally_generated(game);
                        out.push(Visit{ target: player, attack: false });
                    }
                    if let Some(chosen_outline) = selection.1{
                        let (_, player) = chosen_outline.deref_as_role_and_player_originally_generated(game);
                        out.push(Visit{ target: player, attack: false });
                    }
                    out
                },
            }
        )
        .unwrap_or_default()
}

pub(super) fn get_current_send_chat_groups(game: &Game, actor_ref: PlayerReference, mut night_chat_groups: Vec<ChatGroup>) -> HashSet<ChatGroup> {
    if 
        !actor_ref.alive(game) && 
        !Modifiers::modifier_is_enabled(game, ModifierType::DeadCanChat)
    {
        if PuppeteerMarionette::marionettes_and_puppeteer(game).contains(&actor_ref){
            return vec![ChatGroup::Dead, ChatGroup::Puppeteer].into_iter().collect();
        }
        return vec![ChatGroup::Dead].into_iter().collect();
    }
    if actor_ref.night_silenced(game){
        return HashSet::new();
    }

    match game.current_phase() {
        PhaseState::Briefing => HashSet::new(),
        PhaseState::Obituary => {
            let mut evil_chat_groups = HashSet::new();

            if InsiderGroupID::Puppeteer.is_player_in_revealed_group(game, actor_ref) {
                evil_chat_groups.insert(ChatGroup::Puppeteer);
            }
            if InsiderGroupID::Cult.is_player_in_revealed_group(game, actor_ref) {
                evil_chat_groups.insert(ChatGroup::Cult);
            }
            if InsiderGroupID::Mafia.is_player_in_revealed_group(game, actor_ref) {
                evil_chat_groups.insert(ChatGroup::Mafia);
            }

            evil_chat_groups
        },
        PhaseState::Discussion 
        | PhaseState::Nomination {..}
        | PhaseState::Judgement {..} 
        | PhaseState::FinalWords {..}
        | PhaseState::Dusk => vec![ChatGroup::All].into_iter().collect(),
        &PhaseState::Testimony { player_on_trial, .. } => {
            if player_on_trial == actor_ref {
                vec![ChatGroup::All].into_iter().collect()
            } else {
                HashSet::new()
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
                out.push(ChatGroup::Dead);
            }
            if PlayerReference::all_players(game)
                .any(|p|
                    match p.role_state(game) {
                        RoleState::Reporter(Reporter{interviewed_target: Some(interviewed_target_ref), ..}) => {
                            *interviewed_target_ref == actor_ref
                        },
                        _ => false
                    }
                )
            {
                out.push(ChatGroup::Interview);
            }


            let mut jail_or_night_chats = 
            if Detained::is_detained(game, actor_ref) && PlayerReference::all_players(game).any(|detainer|
                match detainer.role_state(game) {
                    RoleState::Jailor(jailor) => {
                        jailor.jailed_target_ref == Some(actor_ref)
                    },
                    _ => false
                }
            ) {
                vec![ChatGroup::Jail]
            }else if Detained::is_detained(game, actor_ref) && PlayerReference::all_players(game).any(|detainer|
                match detainer.role_state(game) {
                    RoleState::Kidnapper(kidnapper) => {
                        kidnapper.jailed_target_ref == Some(actor_ref)
                    },
                    _ => false
                }
            ) {
                vec![ChatGroup::Kidnapped]
            }else{
                if InsiderGroupID::Puppeteer.is_player_in_revealed_group(game, actor_ref){
                    night_chat_groups.push(ChatGroup::Puppeteer);
                }
                if InsiderGroupID::Mafia.is_player_in_revealed_group(game, actor_ref){
                    night_chat_groups.push(ChatGroup::Mafia);
                }
                if InsiderGroupID::Cult.is_player_in_revealed_group(game, actor_ref){
                    night_chat_groups.push(ChatGroup::Cult);
                }
                night_chat_groups
            };


            out.append(&mut jail_or_night_chats);
            out.into_iter().collect()
        },
    }
}
pub(super) fn get_current_receive_chat_groups(game: &Game, actor_ref: PlayerReference) -> HashSet<ChatGroup> {
    let mut out = Vec::new();

    out.push(ChatGroup::All);

    if !actor_ref.alive(game){
        out.push(ChatGroup::Dead);
    }

    if InsiderGroupID::Mafia.is_player_in_revealed_group(game, actor_ref) {
        out.push(ChatGroup::Mafia);
    }
    if InsiderGroupID::Cult.is_player_in_revealed_group(game, actor_ref) {
        out.push(ChatGroup::Cult);
    }
    if InsiderGroupID::Puppeteer.is_player_in_revealed_group(game, actor_ref){
        out.push(ChatGroup::Puppeteer);
    }


    if Detained::is_detained(game, actor_ref) {
        if PlayerReference::all_players(game).any(|detainer|
            match detainer.role_state(game) {
                RoleState::Jailor(jailor) => {
                    jailor.jailed_target_ref == Some(actor_ref)
                },
                _ => false
            }
        ) {
            out.push(ChatGroup::Jail);
        }
        if PlayerReference::all_players(game).any(|detainer|
            match detainer.role_state(game) {
                RoleState::Kidnapper(kidnapper) => {
                    kidnapper.jailed_target_ref == Some(actor_ref)
                },
                _ => false
            }
        ) {
            out.push(ChatGroup::Kidnapped);
        }
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
        out.push(ChatGroup::Dead);
    }
    if 
        game.current_phase().phase() == PhaseType::Night && 
        PlayerReference::all_players(game)
            .any(|p|
                match p.role_state(game) {
                    RoleState::Reporter(Reporter{interviewed_target: Some(interviewed_target_ref), ..}) => {
                        *interviewed_target_ref == actor_ref
                    },
                    _ => false
                }
            )
    {
        out.push(ChatGroup::Interview);
    }

    out.into_iter().collect()
}

///Only works for roles that win based on end game condition
pub(super) fn default_win_condition(role: Role) -> WinCondition {

    if RoleSet::Mafia.get_roles().contains(&role) {
        WinCondition::GameConclusionReached{win_if_any: vec![GameConclusion::Mafia].into_iter().collect()}

    }else if RoleSet::Cult.get_roles().contains(&role) {
        WinCondition::GameConclusionReached{win_if_any: vec![GameConclusion::Cult].into_iter().collect()}

    }else if RoleSet::Town.get_roles().contains(&role) {
        WinCondition::GameConclusionReached{win_if_any: vec![GameConclusion::Town].into_iter().collect()}

    }else if RoleSet::Fiends.get_roles().contains(&role) {
        WinCondition::GameConclusionReached{win_if_any: vec![GameConclusion::Fiends].into_iter().collect()}

    }else if RoleSet::Minions.get_roles().contains(&role) {
        WinCondition::GameConclusionReached{win_if_any: GameConclusion::all().into_iter().filter(|end_game_condition|
            match end_game_condition {
                GameConclusion::Town | GameConclusion::Draw => false,
                _ => true
            }
        ).collect()}

    }else{
        WinCondition::RoleStateWon
    }
}