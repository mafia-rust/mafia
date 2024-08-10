use crate::game::{phase::PhaseType, player::PlayerReference, role::Role, role_list::{Faction, RoleSet}, Game};
use rand::prelude::SliceRandom;

#[derive(Clone)]
pub struct Mafia{
    //Must be mafia killing, forced at runtime
    main_killing_role: Role,
}
impl Default for Mafia{
    fn default()->Self{
        Self{
            main_killing_role: Role::Godfather
        }
    }
}
impl Game{
    pub fn mafia(&self)->&Mafia{
        &self.mafia
    }
    pub fn set_mafia(&mut self, mafia: Mafia){
        self.mafia = mafia;
    }
}
impl Mafia{
    pub fn on_phase_start(game: &mut Game, _phase: PhaseType){
        Mafia::ensure_mafia_can_kill(game);
    }
    pub fn on_game_start(game: &mut Game) {

        //find what mafia killing there is, if any
        for player in PlayerReference::all_players(game){
            if RoleSet::MafiaKilling.get_roles().iter().any(|role| {
                player.role(game) == *role
            }){
                game.set_mafia(Mafia{
                    main_killing_role: player.role(game)
                });
                return;
            }
        }

        Mafia::ensure_mafia_can_kill(game);
    }
    /// - This must go after rolestate on any death
    /// - Godfathers backup should become godfather if godfather dies as part of the godfathers ability
    pub fn on_any_death(game: &mut Game, dead_player: PlayerReference){
        if dead_player.role(game).faction() == Faction::Mafia {
            Mafia::ensure_mafia_can_kill(game);
        }
    }
    pub fn on_role_switch(game: &mut Game, old: Role, new: Role) {
        if old.faction() == Faction::Mafia || new.faction() == Faction::Mafia {
            Mafia::ensure_mafia_can_kill(game);
        }

        for a in Mafia::get_members(game) {
            for b in Mafia::get_members(game) {
                a.insert_role_label(game, b);
            }
        }
    }
    pub fn get_members(game: &Game)->Vec<PlayerReference>{
        PlayerReference::all_players(game).filter(
            |p| p.role(game).faction() == Faction::Mafia
        ).collect()
    }
    pub fn get_living_members(&self, game: &Game)->Vec<PlayerReference>{
        PlayerReference::all_players(game).filter(
            |p| p.role(game).faction() == Faction::Mafia && p.alive(game)
        ).collect()
    }

    fn ensure_mafia_can_kill(game: &mut Game){

        //check if there is a mafia killing role alive
        for player_ref in PlayerReference::all_players(game){
            if
                player_ref.alive(game) && 
                RoleSet::MafiaKilling.get_roles().iter().any(|role| {
                    player_ref.role(game) == *role
                })
            {
                return;
            }
        }

        //if no mafia killing exists, the code can reach here
        let list_of_living_mafia = PlayerReference::all_players(game)
            .filter(|p| 
                p.role(game).faction() == Faction::Mafia && p.alive(game)
            )
            .collect::<Vec<PlayerReference>>();
        
        //choose random mafia to be mafia killing
        let random_mafia = list_of_living_mafia.choose(&mut rand::thread_rng());
        

        if let Some(random_mafia) = random_mafia{
            random_mafia.set_role(game, game.mafia().main_killing_role.default_state());
        }
    }
}