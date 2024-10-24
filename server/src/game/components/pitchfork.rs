use std::{collections::{HashMap, HashSet}, ops::Mul};

use crate::{game::{
    attack_power::AttackPower, grave::GraveKiller, phase::PhaseType, player::PlayerReference, game_conclusion::GameConclusion, role::Priority, role_list::RoleSet, Game
}, packet::ToClientPacket};

#[derive(Clone)]
pub struct Pitchfork{
    pitchfork_owners: HashSet<PlayerReference>,

    pitchfork_uses_remaining: u8,

    angry_mob_vote: HashMap<PlayerReference, PlayerReference>,
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
            angry_mob_vote: Default::default(),
            angry_mobbed_player: Default::default()
        }
    }
}

impl Pitchfork{
    pub fn on_phase_start(game: &mut Game, phase: PhaseType){
        match phase{
            PhaseType::Night => {
                Pitchfork::set_angry_mobbed_player(game, None);
                if Pitchfork::pitchfork_uses_remaining(game) > 0 {
                    if let Some(target) = Pitchfork::player_is_voted(game){
                        Pitchfork::set_angry_mobbed_player(game, Some(target));
                    }
                }
                Pitchfork::clear_votes_for_angry_mob(game);
            },
            _ => {}
        }
    }
    pub fn on_night_priority(game: &mut Game, priority: Priority){
        if priority != Priority::Kill {return;}
        if game.day_number() <= 1 {return;}
        if !Pitchfork::pitchfork_owners(game).iter().any(|p|p.alive(game)) {return;}
        
        if let Some(target) = Pitchfork::angry_mobbed_player(game) {
            target.try_night_kill(
                &Pitchfork::pitchfork_owners(game).iter().filter(|p|p.alive(game)).map(|p|*p).collect(), 
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

    
    pub fn player_is_voted(game: &Game) -> Option<PlayerReference> {
        let pitchfork = game.pitchfork().clone();
        let mut votes: HashMap<PlayerReference, u8> = HashMap::new();

        for (voter, target) in pitchfork.angry_mob_vote.iter(){
            if 
                !voter.alive(game) || 
                !target.alive(game) || 
                !voter.win_condition(game).requires_only_this_resolution_state(GameConclusion::Town) 
            {continue;}

            let count = votes.entry(*target).or_insert(0);
            *count += 1;
            if *count >= Pitchfork::number_of_votes_needed(game) {return Some(*target);}
        }
        None
    }

    pub fn player_votes_for_angry_mob_action(game: &mut Game, player: PlayerReference, target: Option<PlayerReference>) {
        Pitchfork::set_vote_for_angry_mob(game, player, 
            if let Some(target_player) = target {
                if player.alive(game) && target_player.alive(game){
                    target
                }else{
                    None
                }
            }else{
                None
            }
        );
    }
    pub fn set_vote_for_angry_mob(game: &mut Game, player_ref: PlayerReference, target_ref: Option<PlayerReference>){
        let mut pitchfork = game.pitchfork().clone();

        if let Some(target_ref) = target_ref{
            pitchfork.angry_mob_vote.insert(player_ref, target_ref);
        }else{
            pitchfork.angry_mob_vote.remove(&player_ref);
        }

        player_ref.send_packet(game, ToClientPacket::YourPitchforkVote { player: target_ref });
        

        game.set_pitchfork(pitchfork);
    }
    pub fn clear_votes_for_angry_mob(game: &mut Game){
        let mut pitchfork = game.pitchfork().clone();
        pitchfork.angry_mob_vote.clear();

        for player in PlayerReference::all_players(game){
            player.send_packet(game, ToClientPacket::YourPitchforkVote { player: None });
        }

        game.set_pitchfork(pitchfork);
    }
    pub fn number_of_votes_needed(game: &Game) -> u8 {
        let x = PlayerReference::all_players(game).filter(|p|
            p.alive(game) && p.win_condition(game).requires_only_this_resolution_state(GameConclusion::Town)
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
    pub fn pitchfork_owners(game: &Game) -> HashSet<PlayerReference>{
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