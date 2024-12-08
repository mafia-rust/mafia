use rand::seq::SliceRandom;

use crate::{game::{ 
    ability_input::{AbilitySelection, AvailableAbilitySelection, ControllerID, ControllerParametersMap, PlayerListSelection}, attack_power::AttackPower, chat::{ChatGroup, ChatMessageVariant}, grave::GraveKiller, phase::PhaseType, player::PlayerReference, role::{Priority, RoleState}, role_list::RoleSet, visit::Visit, Game
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
    pub fn controller_parameters_map(game: &Game)->ControllerParametersMap{
        let mut out = ControllerParametersMap::default();

        let players_with_gun = Self::players_with_gun(game);

        let available_backup_players = PlayerReference::all_players(game)
            .filter(|p|
                InsiderGroupID::Mafia.is_player_in_revealed_group(game, *p) &&
                p.alive(game) &&
                !players_with_gun.contains(p)
            )
            .collect::<VecSet<_>>();

        out.combine_overwrite(
            ControllerParametersMap::new_controller_fast(
                game,
                ControllerID::syndicate_choose_backup(),
                AvailableAbilitySelection::new_player_list(
                    available_backup_players,
                    false,
                    Some(1)
                ),
                AbilitySelection::new_player_list(vec![]),
                false,
                None,
                false,
                players_with_gun.clone()
            )
        );

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
                    ControllerParametersMap::new_controller_fast(
                        game,
                        ControllerID::syndicate_backup_attack(),
                        AvailableAbilitySelection::new_player_list(
                            attackable_players,
                            false,
                            Some(1)
                        ),
                        AbilitySelection::new_player_list(vec![]),
                        !backup.alive(game) || Detained::is_detained(game, *backup) || game.day_number() <= 1,
                        Some(PhaseType::Obituary),
                        false,
                        vec_set!(*backup).union(&players_with_gun)
                    )
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
    pub fn on_night_priority(game: &mut Game, priority: Priority){
        if game.day_number() <= 1 {return}
        match priority {
            Priority::TopPriority => {
                let Some(PlayerListSelection(backup)) = game.saved_controllers.get_controller_current_selection_player_list(ControllerID::syndicate_choose_backup()) else {return};
                let Some(backup) = backup.first() else {return};

                let Some(PlayerListSelection(backup_target)) = game.saved_controllers.get_controller_current_selection_player_list(ControllerID::syndicate_backup_attack()) else {return};
                let Some(backup_target) = backup_target.first() else {return};

                let new_visit = Visit::new(*backup, *backup_target, true, crate::game::visit::VisitTag::SyndicateBackupAttack);
                NightVisits::add_visit(game, new_visit);
            }
            Priority::Deception => {
                if Self::players_with_gun(&game).into_iter().any(|p|!p.night_blocked(game) && p.alive(game)) {
                    NightVisits::clear_visits_with_predicate(game, |v|v.tag == crate::game::visit::VisitTag::SyndicateBackupAttack);
                }
            }
            Priority::Kill => {

                let all_backup_visits: Vec<Visit> = NightVisits::all_visits(game).into_iter().filter(|v|v.tag == crate::game::visit::VisitTag::SyndicateBackupAttack).cloned().collect();
                for backup_visit in all_backup_visits {
                    backup_visit.target.try_night_kill_single_attacker(
                        backup_visit.visitor, game, GraveKiller::RoleSet(RoleSet::Mafia),
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
                InsiderGroupID::Mafia.is_player_in_revealed_group(game, p) &&
                RoleSet::MafiaKilling.get_roles().contains(&p.role(game))
        );

        if !killing_role_exists{
            //give random syndicate insider the gun
            let insiders = PlayerReference::all_players(game)
                .filter(|p| InsiderGroupID::Mafia.is_player_in_revealed_group(game, *p))
                .collect::<Vec<_>>();

            let Some(insider) = insiders.choose(&mut rand::thread_rng()) else {return};

            SyndicateGunItem::give_gun(game, *insider);
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
        let living_players_to_convert = PlayerReference::all_players(game).into_iter().filter(
            |p|
            InsiderGroupID::Mafia.is_player_in_revealed_group(game, *p) &&
            RoleSet::Mafia.get_roles().contains(&p.role(game)) &&
            p.alive(game)
        ).collect::<Vec<_>>();

        //if they already have a mafia killing then return
        if living_players_to_convert.iter().any(|p|
            RoleSet::MafiaKilling.get_roles().contains(&p.role(game))
        ) {return;}
        
        //choose random mafia to be mafia killing
        let random_mafia = living_players_to_convert.choose(&mut rand::thread_rng());
        
        if let Some(random_mafia) = random_mafia {
            random_mafia.set_role_and_win_condition_and_revealed_group(game, role);
        }
    }
}