use rand::seq::SliceRandom;

use crate::game::{modifiers::{mafia_hit_orders::MafiaHitOrders, ModifierType, Modifiers}, phase::PhaseType, player::PlayerReference, role::{Role, RoleState}, role_list::RoleSet, visit::Visit, Game};

use super::insider_group::InsiderGroupID;


const DEFAULT_MAFIA_KILLING_ROLE: Role = Role::Godfather;

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
    pub fn on_phase_start(_game: &mut Game, _phase: PhaseType){
    }
    pub fn on_game_start(game: &mut Game) {
        Mafia::give_mafia_killing_role(game, DEFAULT_MAFIA_KILLING_ROLE.default_state());
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

        if let Some(modifier) = Modifiers::get_modifier_inner::<MafiaHitOrders>(game, ModifierType::MafiaHitOrders) {
            if modifier.active() {
                return;
            }
        }

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

    pub fn mafia_killing_visits(game: &Game) -> Vec<Visit> {
        if let Some(mafia_killing_player) = 
            PlayerReference::all_players(game).into_iter()
            .find(|p| RoleSet::MafiaKilling.get_roles().contains(&p.role(game)))
        {
            mafia_killing_player.night_visits(game).clone()
        }else{
            vec![]
        }
    }
}