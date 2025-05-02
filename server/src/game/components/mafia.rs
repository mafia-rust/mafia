use rand::seq::IndexedRandom;

use crate::{game::{
    ability_input::{AvailablePlayerListSelection, ControllerParametersMap}, attack_power::{AttackPower, DefensePower},
    chat::{ChatGroup, ChatMessageVariant},
    event::{
        on_add_insider::OnAddInsider,
        on_midnight::{MidnightVariables, OnMidnight, OnMidnightPriority}, on_remove_insider::OnRemoveInsider
    },
    grave::GraveKiller, phase::PhaseType, player::PlayerReference, role::RoleState,
    role_list::RoleSet, visit::{Visit, VisitTag}, ControllerID, Game, PlayerListSelection
}, vec_set::{vec_set, VecSet}};

use super::{detained::Detained, fragile_vest::FragileVests, insider_group::InsiderGroupID, night_visits::NightVisits, player_component::PlayerComponent, syndicate_gun_item::SyndicateGunItem, tags::Tags};

#[derive(Clone)]
pub struct Mafia;
impl Game{
    pub fn mafia(&self)->&Mafia{
        &self.mafia
    }
    pub fn set_mafia(&mut self, mafia: Mafia){
        self.mafia = mafia;
    }
}
impl Mafia{
    pub fn on_visit_wardblocked(_game: &mut Game, midnight_variables: &mut MidnightVariables, visit: Visit){
        NightVisits::retain(midnight_variables, |v|
            v.tag != VisitTag::SyndicateBackupAttack || v.visitor != visit.visitor
        );
    }
    pub fn on_player_roleblocked(_game: &mut Game, midnight_variables: &mut MidnightVariables, player: PlayerReference){
        NightVisits::retain(midnight_variables, |v|
            v.tag != VisitTag::SyndicateBackupAttack || v.visitor != player
        );
    }

    pub fn controller_parameters_map(game: &Game)->ControllerParametersMap{
        let players_with_gun = Self::syndicate_killing_players(game);

        let available_backup_players = PlayerReference::all_players(game)
            .filter(|p|
                InsiderGroupID::Mafia.contains_player(game, *p) &&
                p.alive(game) &&
                !players_with_gun.contains(p)
            )
            .collect::<VecSet<_>>();

        let mut out = ControllerParametersMap::builder(game)
            .id(ControllerID::syndicate_choose_backup())
            .available_selection(AvailablePlayerListSelection {
                available_players: available_backup_players,
                can_choose_duplicates: false,
                max_players: Some(1)
            })
            .allow_players(players_with_gun.clone())
            .build_map();

        if let Some(PlayerListSelection(player_list)) = ControllerID::syndicate_choose_backup().get_player_list_selection(game){
            if let Some(backup) = player_list.first(){

                let attackable_players = PlayerReference::all_players(game)
                    .filter(|p|
                        !InsiderGroupID::Mafia.contains_player(game, *p) &&
                        p.alive(game) &&
                        *p != *backup
                    )
                    .collect::<VecSet<_>>();

                out.combine_overwrite(
                    ControllerParametersMap::builder(game)
                        .id(ControllerID::syndicate_backup_attack())
                        .available_selection(AvailablePlayerListSelection {
                            available_players: attackable_players,
                            can_choose_duplicates: false,
                            max_players: Some(1)
                        })
                        .add_grayed_out_condition(!backup.alive(game) || Detained::is_detained(game, *backup) || game.day_number() <= 1)
                        .reset_on_phase_start(PhaseType::Obituary)
                        .allow_players(players_with_gun.union(&vec_set!(*backup)))
                        .build_map()
                );
            }
        }

        out
    }
    
    pub fn syndicate_killing_players(game: &Game)->VecSet<PlayerReference>{
        PlayerReference::all_players(game)
            .filter(|p|
                InsiderGroupID::Mafia.contains_player(game, *p) &&
                (
                    SyndicateGunItem::player_with_gun(&game.syndicate_gun_item).is_some_and(|f|f==*p) ||
                    RoleSet::MafiaKilling.get_roles().contains(&p.role(game))
                )
            )
            .collect::<VecSet<_>>()
    }
    pub fn on_phase_start(_game: &mut Game, _phase: PhaseType){
    }
    pub fn on_midnight(game: &mut Game, _event: &OnMidnight, midnight_variables: &mut MidnightVariables, priority: OnMidnightPriority){
        if game.day_number() <= 1 {return}
        match priority {
            OnMidnightPriority::TopPriority => {
                let Some(PlayerListSelection(backup)) = ControllerID::syndicate_choose_backup().get_player_list_selection(game) else {return};
                let Some(backup) = backup.first() else {return};

                let Some(PlayerListSelection(backup_target)) = ControllerID::syndicate_backup_attack().get_player_list_selection(game) else {return};
                let Some(backup_target) = backup_target.first() else {return};

                let new_visit = Visit::new(*backup, *backup_target, true, crate::game::visit::VisitTag::SyndicateBackupAttack);
                NightVisits::add_visit(midnight_variables, new_visit);
            }
            OnMidnightPriority::Deception => {
                if Self::syndicate_killing_players(game).into_iter().any(|p|!p.night_blocked(midnight_variables) && p.alive(game)) {
                    NightVisits::retain(midnight_variables, |v|v.tag != crate::game::visit::VisitTag::SyndicateBackupAttack);
                }
            }
            OnMidnightPriority::Kill => {

                let all_backup_visits: Vec<Visit> = NightVisits::all_visits(midnight_variables).into_iter().filter(|v|v.tag == crate::game::visit::VisitTag::SyndicateBackupAttack).copied().collect();
                for backup_visit in all_backup_visits {
                    backup_visit.target.try_night_kill_single_attacker(
                        backup_visit.visitor, game, midnight_variables, GraveKiller::RoleSet(RoleSet::Mafia),
                        AttackPower::Basic, false
                    );
                    game.add_message_to_chat_group(ChatGroup::Mafia, 
                        ChatMessageVariant::GodfatherBackupKilled { backup: backup_visit.visitor.index() }
                    );
                }
            }
            _ => {}
        }
    }
    pub fn on_game_start(game: &mut Game) {

        let killing_role_exists = PlayerReference::all_players(game).any(
            |p|
                InsiderGroupID::Mafia.contains_player(game, p) &&
                RoleSet::MafiaKilling.get_roles().contains(&p.role(game))
        );

        if !killing_role_exists{
            //give random syndicate insider the gun
            let insiders = PlayerReference::all_players(game)
                .filter(|p| InsiderGroupID::Mafia.contains_player(game, *p))
                .collect::<Vec<_>>();

            let Some(insider) = insiders.choose(&mut rand::rng()) else {return};

            SyndicateGunItem::give_gun_to_player(game, *insider);
            PlayerComponent::<FragileVests>::add_defense_item(game, *insider, DefensePower::Armored, vec_set![*insider]);
        }
    }

    pub fn on_controller_selection_changed(game: &mut Game, controller_id: ControllerID){
        if controller_id != ControllerID::syndicate_choose_backup() {return};

        let backup = controller_id.get_player_list_selection(game)
            .and_then(|b|b.0.first().copied());

        if let Some(backup) = backup{
            Tags::set_tagged(game, super::tags::TagSetID::SyndicateBackup, &vec_set![backup]);
        }
    }

    /// - This must go after rolestate on any death
    /// - Godfathers backup should become godfather if godfather dies as part of the godfathers ability
    pub fn on_any_death(game: &mut Game, dead_player: PlayerReference){
        if RoleSet::MafiaKilling.get_roles().contains(&dead_player.role(game)) {
            Mafia::give_mafia_killing_role(game, dead_player.role_state(game).clone());
        }
    }
    pub fn on_role_switch(game: &mut Game, old: RoleState, _new: RoleState) {
        if RoleSet::MafiaKilling.get_roles().contains(&old.role()) {
            Mafia::give_mafia_killing_role(game, old);
        }
    }
    pub fn on_add_insider(game: &mut Game, _event: &OnAddInsider, _fold: &mut (), _priority: ()){
        Tags::set_viewers(game, super::tags::TagSetID::SyndicateBackup, &InsiderGroupID::Mafia.players(game).clone());
    }
    pub fn on_remove_insider(game: &mut Game, _event: &OnRemoveInsider, _fold: &mut (), _priority: ()){
        Tags::set_viewers(game, super::tags::TagSetID::SyndicateBackup, &InsiderGroupID::Mafia.players(game).clone());
    }


    pub fn give_mafia_killing_role(
        game: &mut Game,
        role: RoleState
    ){
        let living_players_to_convert = PlayerReference::all_players(game)
            .filter(|p|
                p.alive(game) &&
                InsiderGroupID::Mafia.contains_player(game, *p)
            )
            .collect::<Vec<_>>();

        //if they already have a mafia killing then return
        if living_players_to_convert.iter().any(|p|
            RoleSet::MafiaKilling.get_roles().contains(&p.role(game))
        ) {return;}
        
        //choose random mafia to be mafia killing
        let random_mafia = living_players_to_convert.choose(&mut rand::rng());
        
        if let Some(random_mafia) = random_mafia {
            random_mafia.set_role_and_win_condition_and_revealed_group(game, role);
        }
    }
}