use rand::seq::SliceRandom;

use super::{chat::{ChatGroup, ChatMessageVariant}, phase::PhaseType, player::PlayerReference, role::{
        apostle::Apostle,
        godfather::Godfather,
        zealot::Zealot,
        disciple::Disciple,
        Role, RoleState
    }, role_list::Faction, Game
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Team{
    Mafia, Cult
}
impl Team{
    pub fn same_team(game: &Game, a: PlayerReference, b: PlayerReference)->bool{
        let Some(a) = a.role_state(game).team(game, a) else {return false};
        let Some(b) = b.role_state(game).team(game, b) else {return false};

        a == b
    }

    pub fn members(&self, game: &Game) -> Vec<PlayerReference> {
        PlayerReference::all_players(game)
            .filter(|p| p.team(game) == Some(*self))
            .collect()
    }
    
    pub fn team_state(&self, teams: &Teams)->TeamState{
        match self{
            Team::Mafia => TeamState::Mafia(teams.mafia().clone()),
            Team::Cult => TeamState::Cult(teams.cult().clone()),
        }
    }
}


pub enum TeamState{
    Mafia(Mafia), Cult(Cult)
}
impl TeamState{
    pub fn team(&self) -> Team{
        match self {
            TeamState::Mafia(t) => t.team(),
            TeamState::Cult(t) => t.team(),
        }
    }
    pub fn on_creation(self, game: &mut Game){
        match self {
            TeamState::Mafia(t) => t.on_creation(game),
            TeamState::Cult(t) => t.on_creation(game),
        }
    }
    pub fn on_phase_start(self, game: &mut Game, phase: PhaseType){
        match self {
            TeamState::Mafia(t) => t.on_phase_start(game, phase),
            TeamState::Cult(t) => t.on_phase_start(game, phase),
        }
    }
    pub fn on_any_death(self, game: &mut Game){
        match self {
            TeamState::Mafia(t) => t.on_any_death(game),
            TeamState::Cult(t) => t.on_any_death(game),
        }
    }
    pub fn on_member_role_switch(self, game: &mut Game, actor: PlayerReference){
        match self {
            TeamState::Mafia(t) => t.on_member_role_switch(game, actor),
            TeamState::Cult(t) => t.on_member_role_switch(game, actor),
        }
    }
}




pub trait TeamStateImpl : Clone{
    fn team(&self) -> Team;
    fn on_creation(self, game: &mut Game);
    fn on_phase_start(self, game: &mut Game, phase: PhaseType);
    fn on_any_death(self, game: &mut Game);
    fn on_member_role_switch(self, game: &mut Game, actor: PlayerReference);
}

#[derive(Default)]
pub struct Teams{
    mafia: Mafia,
    cult: Cult
}
impl Teams{
    pub fn on_team_creation(game: &mut Game){
        game.teams.mafia.clone().on_creation(game);
        game.teams.cult.clone().on_creation(game);
    }
    pub fn on_phase_start(game: &mut Game, phase: PhaseType){
        game.teams.mafia.clone().on_phase_start(game, phase);
        game.teams.cult.clone().on_phase_start(game, phase);
    }
    pub fn on_any_death(game: &mut Game){
        game.teams.mafia.clone().on_any_death(game);
        game.teams.cult.clone().on_any_death(game);
    }

    pub fn mafia(&self)->&Mafia{
        &self.mafia
    }
    pub fn cult(&self)->&Cult{
        &self.cult
    }

    pub fn set_mafia(&mut self, mafia: Mafia){
        self.mafia = mafia;
    }
    pub fn set_cult(&mut self, cult: Cult){
        self.cult = cult;
    }
}


#[derive(Default, Clone)]
pub struct Mafia;
impl TeamStateImpl for Mafia{
    fn team(&self) -> Team {
        Team::Mafia
    }
    fn on_phase_start(self, game: &mut Game, _phase: PhaseType){
        //This depends on role_state.on_phase_start being called before this
        Mafia::ensure_mafia_can_kill(game);
    }
    fn on_creation(self, game: &mut Game) {
        //This depends on role_state.on_any_death being called before this
        Mafia::ensure_mafia_can_kill(game);
    }
    fn on_any_death(self, game: &mut Game){
        //This depends on role_state.on_any_death being called before this
        Mafia::ensure_mafia_can_kill(game);
    }
    fn on_member_role_switch(self, game: &mut Game, _actor: PlayerReference) {
        Mafia::ensure_mafia_can_kill(game);
    }
}
impl Mafia{
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
            random_mafia.set_role(game, super::role::RoleState::Godfather(Godfather::default()));
        }
    }
}


#[derive(Default, Clone)]
pub struct Cult {
    pub ordered_cultists: Vec<PlayerReference>,
    pub sacrifices_needed: Option<u8>
}
impl TeamStateImpl for Cult{
    fn team(&self) -> Team {
        Team::Cult
    }
    fn on_phase_start(self, game: &mut Game, phase: PhaseType){
        Cult::set_ordered_cultists(self.clone(), game);
        
        if phase == PhaseType::Night {
            if self.can_convert_tonight(game){
                game.add_message_to_chat_group(ChatGroup::Cult, ChatMessageVariant::ApostleCanConvertTonight);
            }else{
                game.add_message_to_chat_group(ChatGroup::Cult, ChatMessageVariant::ApostleCantConvertTonight);
            }
        }
    }
    fn on_creation(self, game: &mut Game) {
        Cult::set_ordered_cultists(self, game);
    }
    fn on_any_death(mut self, game: &mut Game){
        self.sacrifices_needed = self.sacrifices_needed.map(|s| s.saturating_sub(1));
        if let Some(s) = self.sacrifices_needed{
            game.add_message_to_chat_group(ChatGroup::Cult, ChatMessageVariant::CultSacrificesRequired { required: s });
        }
        game.teams.set_cult(self.clone());

        Cult::set_ordered_cultists(self.clone(), game);
    }
    fn on_member_role_switch(self, game: &mut Game, _actor: PlayerReference) {
        Cult::set_ordered_cultists(self, game);
    }
}
impl Cult{

    pub const SACRIFICES_NEEDED: u8 = 2;

    fn set_ordered_cultists(mut self, game: &mut Game){
        // Remove dead
        self.ordered_cultists = self.ordered_cultists.iter().cloned().filter(|p|
            p.role(game).faction() == Faction::Cult &&
            p.alive(game)
        ).collect();

        // Add new
        for player in PlayerReference::all_players(game){
            if 
                player.role(game).faction() == Faction::Cult &&
                player.alive(game) &&
                !self.ordered_cultists.contains(&player)
            {
                self.ordered_cultists.push(player);
            }
        }

        for (i, player_ref) in self.ordered_cultists.iter().enumerate(){
            let role = if i == 0 {
                RoleState::Apostle(Apostle)
            }else if i == self.ordered_cultists.len() - 1 {
                RoleState::Zealot(Zealot)
            }else{
                RoleState::Disciple(Disciple)
            };
            
            if player_ref.role(game) == role.role() {continue}
            player_ref.set_role(game, role);
        }

        game.teams.set_cult(self);
    }
    pub fn can_convert_tonight(&self, game: &Game)->bool {
        if self.ordered_cultists.len() >= 4 {return false}

        match self.sacrifices_needed{
            None => game.day_number() != 1,
            Some(blood) => {
                blood == 0
            }
        }
    }
}



