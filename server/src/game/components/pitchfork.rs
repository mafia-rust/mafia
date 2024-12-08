use std::ops::Mul;

use crate::{
    game::{
        ability_input::*,
        attack_power::AttackPower, game_conclusion::GameConclusion,
        grave::GraveKiller, phase::PhaseType, player::PlayerReference,
        role::{Priority, Role}, role_list::RoleSet, Game
    },
    vec_map::VecMap, vec_set::{vec_set, VecSet}
};

#[derive(Clone)]
pub struct Pitchfork{
    pitchfork_owners: VecSet<PlayerReference>,

    pitchfork_uses_remaining: u8,

    angry_mobbed_player: Option<PlayerReference>,
}

impl Game{
    pub fn pitchfork(&self) -> &Pitchfork{
        &self.pitchfork
    }
    pub fn set_pitchfork(&mut self, pitchfork: Pitchfork){
        self.pitchfork = pitchfork;
    }
}

impl Default for Pitchfork{
    fn default() -> Self {
        Self {
            pitchfork_owners: Default::default(),
            pitchfork_uses_remaining: 3,
            angry_mobbed_player: Default::default()
        }
    }
}

impl Pitchfork{
    pub fn controller_parameters_map(game: &Game)->ControllerParametersMap{
        if
            !game.settings.enabled_roles.contains(&Role::Rabblerouser)
        {
            return ControllerParametersMap::default();
        }

        let mut out = ControllerParametersMap::default();
        
        for player in PlayerReference::all_players(game){
            out.combine_overwrite(
                ControllerParametersMap::new_controller_fast(
                    game,
                    ControllerID::pitchfork_vote(player),
                    AvailableAbilitySelection::new_one_player_option(
                        PlayerReference::all_players(game)
                            .into_iter()
                            .filter(|p|p.alive(game))
                            .map(|p|Some(p))
                            .chain(std::iter::once(None))
                            .collect()
                    ),
                    AbilitySelection::new_one_player_option(None),
                    game.day_number() == 1 ||
                        !player.alive(game) ||
                        !player.win_condition(game).is_loyalist_for(GameConclusion::Town),
                        Some(PhaseType::Obituary),
                        false,
                        vec_set![player]
                )
            );
        }
        
        out
    }
    

    pub fn before_phase_end(game: &mut Game, phase: PhaseType){
        match phase{
            PhaseType::Night => {
                Pitchfork::set_angry_mobbed_player(game, None);
                if Pitchfork::pitchfork_uses_remaining(game) > 0 {
                    if let Some(target) = Pitchfork::player_is_voted(game){
                        Pitchfork::set_angry_mobbed_player(game, Some(target));
                    }
                }
            },
            _ => {}
        }
    }
    pub fn on_night_priority(game: &mut Game, priority: Priority){
        if priority != Priority::Kill {return;}
        if game.day_number() <= 1 {return;}
        if Pitchfork::usable_pitchfork_owners(game).len() < 1 {return;}
        
        if let Some(target) = Pitchfork::angry_mobbed_player(game) {
            target.try_night_kill(
                &Pitchfork::usable_pitchfork_owners(game), 
                game, 
                GraveKiller::RoleSet(RoleSet::Town), 
                AttackPower::ProtectionPiercing, 
                false
            );
            Pitchfork::set_pitchfork_uses_remaining(game,
                Pitchfork::pitchfork_uses_remaining(game).saturating_sub(1)
            );
        }
    }

    pub fn usable_pitchfork_owners(game: &Game) -> VecSet<PlayerReference> {
        Pitchfork::pitchfork_owners(game).iter()
            .filter(|p|p.alive(game) && !p.night_blocked(game))
            .map(|p|*p).collect()
    }

    
    pub fn player_is_voted(game: &Game) -> Option<PlayerReference> {
        let mut votes: VecMap<PlayerReference, u8> = VecMap::new();


        for voter in PlayerReference::all_players(game){
            let Some(OnePlayerOptionSelection(Some(target))) = game.saved_controllers
                .get_controller_current_selection_player_option(ControllerID::pitchfork_vote(voter))
                else {continue};
            if 
                !voter.alive(game) || 
                !voter.win_condition(game).is_loyalist_for(GameConclusion::Town) ||
                !target.alive(game)
            {continue;}


            let count: u8 = if let Some(count) = votes.get(&target){
                *count + 1
            }else{
                1
            };
            if count >= Pitchfork::number_of_votes_needed(game) {return Some(target);}
            votes.insert(target, count);
        }
        None
    }

    pub fn number_of_votes_needed(game: &Game) -> u8 {
        let x = PlayerReference::all_players(game).filter(|p|
            p.alive(game) && p.win_condition(game).is_loyalist_for(GameConclusion::Town)
        ).count().mul(2).div_ceil(3) as u8;
        if x == 0 {1} else {x}
    }


    pub fn angry_mobbed_player(game: &Game) -> Option<PlayerReference>{
        game.pitchfork.angry_mobbed_player
    }
    pub fn set_angry_mobbed_player(game: &mut Game, player_ref: Option<PlayerReference>){
        let mut pitchfork = game.pitchfork().clone();
        pitchfork.angry_mobbed_player = player_ref;
        game.set_pitchfork(pitchfork);
    }
    pub fn pitchfork_owners(game: &Game) -> VecSet<PlayerReference>{
        game.pitchfork().pitchfork_owners.clone()
    }
    pub fn has_pitchfork(game: &Game, player_ref: PlayerReference) -> bool{
        game.pitchfork().pitchfork_owners.contains(&player_ref)
    }
    pub fn add_pitchfork(game: &mut Game, player_ref: PlayerReference){
        let mut pitchfork = game.pitchfork().clone();
        pitchfork.pitchfork_owners.insert(player_ref);
        game.set_pitchfork(pitchfork);
    }
    pub fn remove_pitchfork(game: &mut Game, player_ref: PlayerReference){
        let mut pitchfork = game.pitchfork().clone();
        pitchfork.pitchfork_owners.remove(&player_ref);
        game.set_pitchfork(pitchfork);
    }
    pub fn pitchfork_uses_remaining(game: &Game) -> u8{
        game.pitchfork().pitchfork_uses_remaining
    }
    pub fn set_pitchfork_uses_remaining(game: &mut Game, uses: u8){
        let mut pitchfork = game.pitchfork().clone();
        pitchfork.pitchfork_uses_remaining = uses;
        game.set_pitchfork(pitchfork);
    }
}