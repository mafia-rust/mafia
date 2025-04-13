use std::collections::HashSet;

use crate::{game::{
    attack_power::AttackPower, chat::ChatMessageVariant, event::{on_add_insider::OnAddInsider, on_midnight::{OnMidnight, OnMidnightPriority}, on_remove_insider::OnRemoveInsider},
    game_conclusion::GameConclusion, player::PlayerReference, role::Role, role_list::RoleSet, win_condition::WinCondition, Game, InsiderGroupID
}, vec_set::VecSet};

use super::tags::Tags;

impl Game{
    pub fn mafia_recruits(&self)->&MafiaRecruits{
        &self.mafia_recruits
    }
    pub fn set_recruiter_recruits(&mut self, mafia_recruits: MafiaRecruits){
        self.mafia_recruits = mafia_recruits;
    }
}

#[derive(Default, Clone)]
pub struct MafiaRecruits{
    recruits: HashSet<PlayerReference>,
}
impl MafiaRecruits{
    pub fn recruit(game: &mut Game, player: PlayerReference)->bool{
        let mut recruiter_recruits = game.mafia_recruits().clone();

        if InsiderGroupID::Mafia.is_player_in_revealed_group(game, player) {return false;}
        if !recruiter_recruits.recruits.insert(player){return false;}
        Tags::add_tag(game, super::tags::TagSetID::SyndicateRecruit, player);

        game.set_recruiter_recruits(recruiter_recruits);
        InsiderGroupID::Mafia.add_player_to_revealed_group(game, player);
        player.set_win_condition(game, WinCondition::GameConclusionReached { win_if_any: vec![GameConclusion::Mafia].into_iter().collect() });


        for mafia in MafiaRecruits::mafia_and_recruits(game){
            mafia.push_night_message(game, ChatMessageVariant::RecruiterPlayerIsNowRecruit{player: player.index()});
        }

        true
    }

    pub fn kill_recruits(game: &mut Game){
        let marionettes = 
            game.mafia_recruits()
                .recruits
                .iter()
                .filter(|p|p.alive(game))
                .copied()
                .collect::<Vec<_>>();

                MafiaRecruits::attack_players(game, marionettes, AttackPower::ProtectionPiercing);
    }
    fn attack_players(game: &mut Game, players: Vec<PlayerReference>, attack_power: AttackPower){
        
        let recruiters: VecSet<_> = PlayerReference::all_players(game)
            .filter(|p|p.role(game)==Role::Recruiter)
            .collect();

        for player in players{
            player.try_night_kill(&recruiters, game, crate::game::grave::GraveKiller::RoleSet(RoleSet::Mafia), attack_power, false);
        }
    }

    pub fn is_recruited(game: &Game, player: PlayerReference)->bool{
        game.mafia_recruits().recruits.contains(&player)
    }
    pub fn recruits(game: &Game)->HashSet<PlayerReference>{
        PlayerReference::all_players(game)
            .filter(|p|
                game.mafia_recruits().recruits.contains(p)
            )
            .collect()
    }
    pub fn mafia_members(game: &Game)->HashSet<PlayerReference>{
        PlayerReference::all_players(game)
            .filter(|p|InsiderGroupID::Mafia.is_player_in_revealed_group(game, *p))
            .collect()
    }
    pub fn mafia_and_recruits(game: &Game)->HashSet<PlayerReference>{
        let mut mafia_and_recruits = MafiaRecruits::recruits(game);
        mafia_and_recruits.extend(MafiaRecruits::mafia_members(game));
        mafia_and_recruits
    }



    //event listeners
    pub fn on_add_insider(game: &mut Game, _event: &OnAddInsider, _fold: &mut (), _priority: ()){
        Tags::set_viewers(game, super::tags::TagSetID::SyndicateRecruit, &InsiderGroupID::Mafia.players(game).clone());
    }
    pub fn on_remove_insider(game: &mut Game, _event: &OnRemoveInsider, _fold: &mut (), _priority: ()){
        Tags::set_viewers(game, super::tags::TagSetID::SyndicateRecruit, &InsiderGroupID::Mafia.players(game).clone());
    }
    pub fn on_midnight(game: &mut Game, _event: &OnMidnight, _fold: &mut (), priority: OnMidnightPriority){
        if priority == OnMidnightPriority::Kill{
            MafiaRecruits::kill_recruits(game);
        }
    }
}