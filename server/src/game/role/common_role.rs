use std::collections::HashSet;

use crate::{game::{
    ability_input::*,
    chat::ChatGroup,
    components::{
        detained::Detained,
        puppeteer_marionette::PuppeteerMarionette
    },
    game_conclusion::GameConclusion,
    modifiers::{ModifierType, Modifiers},
    phase::{PhaseState, PhaseType}, player::PlayerReference,
    role_list::RoleSet, visit::Visit, win_condition::WinCondition,
    Game
}, vec_set};

use super::{medium::Medium, reporter::Reporter, warden::Warden, InsiderGroupID, Role, RoleState};

pub fn controller_parameters_map_player_list_night_typical(
    game: &Game,
    actor_ref: PlayerReference,
    can_select_self: bool,
    can_select_insiders: bool,
    grayed_out: bool,
    ability_id: ControllerID,
) -> ControllerParametersMap {
    
    let grayed_out = 
        actor_ref.ability_deactivated_from_death(game) ||
        Detained::is_detained(game, actor_ref) ||
        grayed_out;

    ControllerParametersMap::new_controller_fast(
        game,
        ability_id,
        AvailableAbilitySelection::new_player_list(
            PlayerReference::all_players(game)
                .filter(|player|
                    if !player.alive(game){
                        false
                    }else if *player == actor_ref{
                        can_select_self
                    }else if InsiderGroupID::in_same_revealed_group(game, actor_ref, *player){
                        can_select_insiders
                    }else{
                        true
                    }

                )
                .collect(),
                false,
                Some(1)
            ),
        AbilitySelection::new_player_list(Vec::new()),
        grayed_out,
        Some(PhaseType::Obituary),
        false,
        vec_set!(actor_ref)
    )
}

pub fn controller_parameters_map_boolean(
    game: &Game,
    actor_ref: PlayerReference,
    grayed_out: bool,
    ability_id: ControllerID,
) -> ControllerParametersMap {
    let grayed_out = 
        actor_ref.ability_deactivated_from_death(game) || 
        Detained::is_detained(game, actor_ref) ||
        grayed_out;

    ControllerParametersMap::new_controller_fast(
        game,
        ability_id,
        AvailableAbilitySelection::Boolean,
        AbilitySelection::new_boolean(false),
        grayed_out,
        Some(PhaseType::Obituary),
        false,
        vec_set!(actor_ref)
    )
}


/// This function uses defaults. When using this function, consider if you need to override the defaults.
pub(super) fn convert_controller_selection_to_visits(game: &Game, actor_ref: PlayerReference, ability_id: ControllerID, attack: bool) -> Vec<Visit> {
    
    let Some(selection) = game.saved_controllers.get_controller_current_selection(ability_id) else {return Vec::new()};
    
    match selection {
        AbilitySelection::Unit => vec![Visit::new_none(actor_ref, actor_ref, attack)],
        AbilitySelection::TwoPlayerOption { selection } => {
            if let Some((target_1, target_2)) = selection.0 {
                vec![Visit::new_none(actor_ref, target_1, attack), Visit::new_none(actor_ref, target_2, attack)]
            }else{
                vec![]
            }
        },
        AbilitySelection::PlayerList { selection } => {
            selection.0
                .iter()
                .map(|target_ref| Visit::new_none(actor_ref, *target_ref, attack))
                .collect()
        }
        AbilitySelection::RoleOption { selection } => {
            let mut out = Vec::new();
            for player in PlayerReference::all_players(game){
                if Some(player.role(game)) == selection.0 {
                    out.push(Visit::new_none(actor_ref, player, attack));
                }
            }
            out
        }
        AbilitySelection::TwoRoleOption { selection } => {
            let mut out = Vec::new();
            for player in PlayerReference::all_players(game){
                if Some(player.role(game)) == selection.0 {
                    out.push(Visit::new_none(actor_ref, player, attack));
                }
                if Some(player.role(game)) == selection.1 {
                    out.push(Visit::new_none(actor_ref, player, attack));
                }
            }
            out
        }
        AbilitySelection::TwoRoleOutlineOption { selection } => {
            let mut out = vec![];
            if let Some(chosen_outline) = selection.0{
                let (_, player) = chosen_outline.deref_as_role_and_player_originally_generated(game);
                out.push(Visit::new_none(actor_ref, player, false));
            }
            if let Some(chosen_outline) = selection.1{
                let (_, player) = chosen_outline.deref_as_role_and_player_originally_generated(game);
                out.push(Visit::new_none(actor_ref, player, false));
            }
            out
        },
        _ => Vec::new()
    }
}

pub(super) fn get_current_send_chat_groups(game: &Game, actor_ref: PlayerReference, mut night_chat_groups: Vec<ChatGroup>) -> HashSet<ChatGroup> {
    if game.current_phase().phase() == PhaseType::Recess {
        return vec![ChatGroup::All].into_iter().collect()
    }
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
            let mut out = HashSet::new();

            //evil chat groups
            if InsiderGroupID::Puppeteer.is_player_in_revealed_group(game, actor_ref) {
                out.insert(ChatGroup::Puppeteer);
            }
            if InsiderGroupID::Cult.is_player_in_revealed_group(game, actor_ref) {
                out.insert(ChatGroup::Cult);
            }
            if InsiderGroupID::Mafia.is_player_in_revealed_group(game, actor_ref) {
                out.insert(ChatGroup::Mafia);
            }

            //medium
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
                out.insert(ChatGroup::Dead);
            }

            out
        },
        PhaseState::Discussion 
        | PhaseState::Nomination {..}
        | PhaseState::Judgement {..}
        | PhaseState::FinalWords {..}
        | PhaseState::Dusk 
        | PhaseState::Recess => vec![ChatGroup::All].into_iter().collect(),
        &PhaseState::Testimony { player_on_trial, .. } => {
            if player_on_trial == actor_ref {
                vec![ChatGroup::All].into_iter().collect()
            } else {
                HashSet::new()
            }
        },
        PhaseState::Night => {
            let mut out = vec![];
            //medium seance
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
            //reporter interview
            if 
                PlayerReference::all_players(game).any(|p|
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
            if
                PlayerReference::all_players(game).any(|p|
                    match p.role_state(game) {
                        RoleState::Warden(Warden{players_in_prison}) => {
                            players_in_prison.contains(&actor_ref)
                        },
                        _ => false
                    }
                )
            {
                out.push(ChatGroup::Warden);
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
        PlayerReference::all_players(game).any(|p|
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
    if 
        game.current_phase().phase() == PhaseType::Night && 
        PlayerReference::all_players(game).any(|detainer|
            match detainer.role_state(game) {
                RoleState::Warden(warden) => {
                    warden.players_in_prison.contains(&actor_ref)
                },
                _ => false
            }
        )
    {
        out.push(ChatGroup::Warden);
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
            !matches!(end_game_condition, 
                GameConclusion::Town | GameConclusion::Draw |
                GameConclusion::NiceList | GameConclusion::NaughtyList
            )
        ).collect()}

    }else{
        WinCondition::RoleStateWon
    }
}