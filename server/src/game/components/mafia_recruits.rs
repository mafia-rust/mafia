use std::collections::HashSet;

use crate::game::{
    attack_power::AttackPower, chat::ChatMessageVariant, player::PlayerReference, resolution_state::ResolutionState, role::{
        Priority, Role
    }, role_list::Faction, tag::Tag, win_condition::WinCondition, Game
};

impl Game{
    pub fn mafia_recruits<'a>(&'a self)->&'a MafiaRecruits{
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

        if player.role(game).faction() == Faction::Mafia {return false;}
        if !recruiter_recruits.recruits.insert(player){return false;}

        game.set_recruiter_recruits(recruiter_recruits);
        player.set_win_condition(game, WinCondition::ResolutionStateReached { win_if_any: vec![ResolutionState::Mafia].into_iter().collect() });


        for mafia in MafiaRecruits::mafia_and_recruits(game){
            mafia.push_night_message(game, ChatMessageVariant::RecruiterPlayerIsNowRecruit{player: player.index()});
        }

        MafiaRecruits::give_tags_and_labels(game);
        true
    }

    pub fn kill_recruits(game: &mut Game){
        let marionettes = 
            game.mafia_recruits()
                .recruits
                .iter()
                .filter(|p|p.alive(game))
                .map(|p|p.clone())
                .collect::<Vec<_>>();

                MafiaRecruits::attack_players(game, marionettes, AttackPower::ProtectionPiercing);
    }
    fn attack_players(game: &mut Game, players: Vec<PlayerReference>, attack_power: AttackPower){
        
        let recruiters: HashSet<_> = PlayerReference::all_players(game)
            .filter(|p|p.role(game)==Role::Recruiter)
            .map(|p|p.clone())
            .collect();

        for player in players{
            player.try_night_kill(&recruiters, game, crate::game::grave::GraveKiller::Faction(Faction::Mafia), attack_power, false);
        }
    }

    pub fn give_tags_and_labels(game: &mut Game){
        let mafia_and_recruits = MafiaRecruits::mafia_and_recruits(game);

        for player_a in mafia_and_recruits.clone() {
            for player_b in mafia_and_recruits.clone() {

                player_a.insert_role_label(game, player_b);
                
                if 
                    player_a.player_has_tag(game, player_b, Tag::PuppeteerMarionette) == 0 &&
                    player_b.role(game).faction() != Faction::Mafia
                {
                    player_a.push_player_tag(game, player_b, Tag::PuppeteerMarionette);
                }
            }
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
            .filter(|p|p.role(game).faction()==Faction::Mafia)
            .map(|p|p.clone())
            .collect()
    }
    pub fn mafia_and_recruits(game: &Game)->HashSet<PlayerReference>{
        let mut mafia_and_recruits = MafiaRecruits::recruits(game);
        mafia_and_recruits.extend(MafiaRecruits::mafia_members(game));
        mafia_and_recruits
    }



    //event listeners

    pub fn on_game_start(game: &mut Game){
        MafiaRecruits::give_tags_and_labels(game);
    }
    pub fn on_night_priority(game: &mut Game, priority: Priority){
        if priority == Priority::Kill{
            MafiaRecruits::kill_recruits(game);
        }
    }
}