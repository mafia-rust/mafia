use rand::seq::IndexedRandom;

use crate::{game::{ 
    ability_input::{AvailablePlayerListSelection, ControllerID, ControllerParametersMap, PlayerListSelection}, attack_power::AttackPower, chat::{ChatGroup, ChatMessageVariant}, event::on_midnight::{OnMidnight, OnMidnightPriority}, grave::GraveKiller, phase::PhaseType, player::PlayerReference, role::RoleState, role_list::RoleSet, tag::Tag, visit::{Visit, VisitTag}, Game
}, vec_set::{vec_set, VecSet}};

use super::{detained::Detained, insider_group::InsiderGroupID, night_visits::NightVisits, syndicate_gun_item::SyndicateGunItem};

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
    pub fn on_visit_wardblocked(game: &mut Game, visit: Visit){
        NightVisits::retain(game, |v|
            v.tag != VisitTag::SyndicateBackupAttack || v.visitor != visit.visitor
        );
    }
    pub fn on_player_roleblocked(game: &mut Game, player: PlayerReference){
        NightVisits::retain(game, |v|
            v.tag != VisitTag::SyndicateBackupAttack || v.visitor != player
        );
    }

    pub fn controller_parameters_map(game: &Game)->ControllerParametersMap{
        let players_with_gun = Self::players_with_gun(game);

        let available_backup_players = PlayerReference::all_players(game)
            .filter(|p|
                InsiderGroupID::Mafia.is_player_in_revealed_group(game, *p) &&
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

        if let Some(PlayerListSelection(player_list)) = game.saved_controllers.get_controller_current_selection_player_list(
            ControllerID::syndicate_choose_backup()
        ){
            if let Some(backup) = player_list.first(){

                
                let attackable_players = PlayerReference::all_players(game)
                    .filter(|p|
                        !InsiderGroupID::Mafia.is_player_in_revealed_group(game, *p) &&
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
    
    pub fn players_with_gun(game: &Game)->VecSet<PlayerReference>{
        PlayerReference::all_players(game)
            .filter(|p|
                InsiderGroupID::Mafia.is_player_in_revealed_group(game, *p) &&
                (
                    SyndicateGunItem::player_with_gun(&game.syndicate_gun_item).is_some_and(|f|f==*p) ||
                    RoleSet::MafiaKilling.get_roles().contains(&p.role(game))
                )
            )
            .collect::<VecSet<_>>()
    }
    pub fn on_phase_start(_game: &mut Game, _phase: PhaseType){
    }
    pub fn on_midnight(game: &mut Game, _event: &OnMidnight, _fold: &mut (), priority: OnMidnightPriority){
        if game.day_number() <= 1 {return}
        match priority {
            OnMidnightPriority::TopPriority => {
                let Some(PlayerListSelection(backup)) = game.saved_controllers.get_controller_current_selection_player_list(ControllerID::syndicate_choose_backup()) else {return};
                let Some(backup) = backup.first() else {return};

                let Some(PlayerListSelection(backup_target)) = game.saved_controllers.get_controller_current_selection_player_list(ControllerID::syndicate_backup_attack()) else {return};
                let Some(backup_target) = backup_target.first() else {return};

                let new_visit = Visit::new(*backup, *backup_target, true, crate::game::visit::VisitTag::SyndicateBackupAttack);
                NightVisits::add_visit(game, new_visit);
            }
            OnMidnightPriority::Deception => {
                if Self::players_with_gun(game).into_iter().any(|p|!p.night_blocked(game) && p.alive(game)) {
                    NightVisits::retain(game, |v|v.tag != crate::game::visit::VisitTag::SyndicateBackupAttack);
                }
            }
            OnMidnightPriority::Kill => {

                let all_backup_visits: Vec<Visit> = NightVisits::all_visits(game).into_iter().filter(|v|v.tag == crate::game::visit::VisitTag::SyndicateBackupAttack).copied().collect();
                for backup_visit in all_backup_visits {
                    backup_visit.target.try_night_kill_single_attacker(
                        backup_visit.visitor, game, GraveKiller::RoleSet(RoleSet::Mafia),
                        AttackPower::Basic, false, true
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
                InsiderGroupID::Mafia.is_player_in_revealed_group(game, p) &&
                RoleSet::MafiaKilling.get_roles().contains(&p.role(game))
        );

        if !killing_role_exists{
            //give random syndicate insider the gun
            let insiders = PlayerReference::all_players(game)
                .filter(|p| InsiderGroupID::Mafia.is_player_in_revealed_group(game, *p))
                .collect::<Vec<_>>();

            let Some(insider) = insiders.choose(&mut rand::rng()) else {return};

            SyndicateGunItem::give_gun(game, *insider);
        }
    }

    pub fn on_controller_selection_changed(game: &mut Game, controller_id: ControllerID){
        if controller_id != ControllerID::syndicate_choose_backup() {return};

        let backup = 
            game.saved_controllers.get_controller_current_selection_player_list(controller_id)
            .and_then(|b|b.0.first().copied());

        
        for player_ref in PlayerReference::all_players(game){
            if !InsiderGroupID::Mafia.is_player_in_revealed_group(game, player_ref) {continue}
            player_ref.remove_player_tag_on_all(game, Tag::GodfatherBackup);
        }
        if let Some(backup) = backup{
            for player_ref in PlayerReference::all_players(game){
                if !InsiderGroupID::Mafia.is_player_in_revealed_group(game, player_ref) {continue}
                player_ref.push_player_tag(game, backup, Tag::GodfatherBackup);
            }
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


    pub fn give_mafia_killing_role(
        game: &mut Game,
        role: RoleState
    ){
        let living_players_to_convert = PlayerReference::all_players(game)
            .filter(|p|
                p.alive(game) &&
                InsiderGroupID::Mafia.is_player_in_revealed_group(game, *p)
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