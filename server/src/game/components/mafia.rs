use crate::game::{phase::PhaseType, player::PlayerReference, role::{godfather::Godfather, Role, RoleState}, role_list::Faction, Game};
use rand::prelude::SliceRandom;

impl Game {
    pub fn mafia(&self)->&Mafia{
        &self.mafia
    }
    pub fn set_mafia(&mut self, mafia: Mafia){
        self.mafia = mafia;
    }
}
#[derive(Default, Clone)]
pub struct Mafia;
impl Mafia{
    pub fn on_phase_start(self, game: &mut Game, _phase: PhaseType){
        //This depends on role_state.on_phase_start being called before this
        Mafia::ensure_mafia_can_kill(game);
    }
    pub fn on_game_start(self, game: &mut Game) {
        //This depends on role_state.on_any_death being called before this
        Mafia::ensure_mafia_can_kill(game);
    }
    pub fn on_any_death(self, game: &mut Game){
        //This depends on role_state.on_any_death being called before this
        Mafia::ensure_mafia_can_kill(game);
    }
    pub fn on_member_role_switch(self, game: &mut Game, _actor: PlayerReference) {
        Mafia::ensure_mafia_can_kill(game);
    }

    fn ensure_mafia_can_kill(game: &mut Game){

        for player_ref in PlayerReference::all_players(game){
            if (player_ref.role(game) == Role::Godfather || player_ref.role(game) == Role::Mafioso) && player_ref.alive(game) { 
                return;
            }
        }

        //if no mafia killing exists, the code can reach here
        let list_of_living_mafia = PlayerReference::all_players(game)
            .filter(|p| 
                p.role(game).faction() == Faction::Mafia && p.alive(game)
            )
            .collect::<Vec<PlayerReference>>();
        
        //choose random mafia to be godfather
        let random_mafia = list_of_living_mafia.choose(&mut rand::thread_rng());

        if let Some(random_mafia) = random_mafia{
            random_mafia.set_role(game, RoleState::Godfather(Godfather::default()));
        }
    }
}