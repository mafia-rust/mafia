use super::{player::PlayerReference, Game, role_list::Faction, role::{mafioso::Mafioso, Role}};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Team{
    Mafia, Vampires
}
impl Team{
    pub fn same_team(game: &Game, a: PlayerReference, b: PlayerReference)->bool{
        let Some(a) = a.role_state(game).team(game, a) else {return false};
        let Some(b) = b.role_state(game).team(game, b) else {return false};

        a == b
    }

    pub fn members(&self, game: &Game) -> Vec<PlayerReference> {
        PlayerReference::all_players(game)
            .iter()
            .filter(|p| p.team(game) == Some(*self))
            .cloned()
            .collect()
    }
    
    pub fn team_state(&self, teams: &Teams)->TeamState{
        match self{
            Team::Mafia => TeamState::Mafia(teams.mafia().clone()),
            Team::Vampires => TeamState::Vampires(teams.vampires().clone()),
        }
    }
}


pub enum TeamState{
    Mafia(Mafia), Vampires(Vampires)
}
impl TeamState{
    pub fn team(&self) -> Team{
        match self {
            TeamState::Mafia(t) => t.team(),
            TeamState::Vampires(t) => t.team(),
        }
    }
    pub fn on_creation(self, game: &mut Game){
        match self {
            TeamState::Mafia(t) => t.on_creation(game),
            TeamState::Vampires(t) => t.on_creation(game),
        }
    }
    pub fn on_phase_start(self, game: &mut Game){
        match self {
            TeamState::Mafia(t) => t.on_phase_start(game),
            TeamState::Vampires(t) => t.on_phase_start(game),
        }
    }
    pub fn on_any_death(self, game: &mut Game){
        match self {
            TeamState::Mafia(t) => t.on_any_death(game),
            TeamState::Vampires(t) => t.on_any_death(game),
        }
    }
    pub fn on_member_role_switch(self, game: &mut Game, actor: PlayerReference){
        match self {
            TeamState::Mafia(t) => t.on_member_role_switch(game, actor),
            TeamState::Vampires(t) => t.on_member_role_switch(game, actor),
        }
    }
}




pub trait TeamStateImpl : Clone{
    fn team(&self) -> Team;
    fn on_creation(self, game: &mut Game);
    fn on_phase_start(self, game: &mut Game);
    fn on_any_death(self, game: &mut Game);
    fn on_member_role_switch(self, game: &mut Game, actor: PlayerReference);
}

#[derive(Default)]
pub struct Teams{
    mafia: Mafia,
    vampires: Vampires
}
impl Teams{
    pub fn on_team_creation(game: &mut Game){
        game.teams.mafia.clone().on_creation(game);
        game.teams.vampires.clone().on_creation(game);
    }
    pub fn on_phase_start(game: &mut Game){
        game.teams.mafia.clone().on_phase_start(game);
        game.teams.vampires.clone().on_phase_start(game);
    }
    pub fn on_any_death(game: &mut Game){
        game.teams.mafia.clone().on_any_death(game);
        game.teams.vampires.clone().on_any_death(game);
    }

    pub fn mafia(&self)->&Mafia{
        &self.mafia
    }
    pub fn vampires(&self)->&Vampires{
        &self.vampires
    }

    pub fn set_mafia(&mut self, mafia: Mafia){
        self.mafia = mafia;
    }
    pub fn set_vampires(&mut self, vampires: Vampires){
        self.vampires = vampires;
    }
}


#[derive(Default, Clone)]
pub struct Mafia;
impl TeamStateImpl for Mafia{
    fn team(&self) -> Team {
        Team::Mafia
    }
    fn on_phase_start(self, game: &mut Game){
        Mafia::ensure_mafia_can_kill(game);
    }
    fn on_creation(self, game: &mut Game) {
        Mafia::ensure_mafia_can_kill(game);
    }
    fn on_any_death(self, game: &mut Game){
        Mafia::ensure_mafia_can_kill(game);
    }
    fn on_member_role_switch(self, _game: &mut Game, _actor: PlayerReference) {
        
    }
}
impl Mafia{
    fn ensure_mafia_can_kill(game: &mut Game){
        let mut main_mafia_killing_exists = false;

        for player_ref in PlayerReference::all_players(game){
            if player_ref.role(game) == Role::Mafioso && player_ref.alive(game) { 
                main_mafia_killing_exists = true;
                break;
            }
        }

        // TODO Set an order for roles for conversion
        if !main_mafia_killing_exists{
            for player_ref in PlayerReference::all_players(game){
                if player_ref.role(game).faction_alignment().faction() == Faction::Mafia && player_ref.alive(game){
                    player_ref.set_role(game, super::role::RoleState::Mafioso(Mafioso));
                    break;
                }
            }
        }
    }
}


#[derive(Default, Clone)]
pub struct Vampires {
    pub ordered_vampires: Vec<PlayerReference>,
    pub night_of_last_conversion: Option<u8>
}
impl TeamStateImpl for Vampires{
    fn team(&self) -> Team {
        Team::Vampires
    }
    fn on_phase_start(self, game: &mut Game){
        Vampires::ensure_youngest_vamp(self, game);
    }
    fn on_creation(self, game: &mut Game) {
        Vampires::ensure_youngest_vamp(self, game);
    }
    fn on_any_death(self, game: &mut Game){
        Vampires::ensure_youngest_vamp(self, game);
    }
    fn on_member_role_switch(self, _game: &mut Game, _actor: PlayerReference) {
    }
}
impl Vampires{
    fn ensure_youngest_vamp(mut self, game: &mut Game){
        // Remove dead
        self.ordered_vampires = self.ordered_vampires.iter().cloned().filter(|p|
            p.role(game) == Role::Vampire &&
            p.alive(game)
        ).collect();

        // Add new
        for player in PlayerReference::all_players(game){
            if 
                player.role(game) == Role::Vampire &&
                player.alive(game) &&
                !self.ordered_vampires.contains(&player)
            {
                self.ordered_vampires.push(player);
            }
        }

        game.teams.set_vampires(self);
    }
}



