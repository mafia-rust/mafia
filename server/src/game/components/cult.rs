use crate::game::{chat::{ChatGroup, ChatMessageVariant}, phase::PhaseType, player::PlayerReference, role::{apostle::Apostle, disciple::Disciple, zealot::Zealot, Role, RoleState}, role_list::RoleSet, Game};

use super::insider_group::InsiderGroupID;

impl Game {
    pub fn cult(&self)->&Cult{
        &self.cult
    }
    pub fn set_cult(&mut self, cult: Cult){
        self.cult = cult;
    }
}
#[derive(Default, Debug, Clone)]
pub struct Cult {
    pub ordered_cultists: Vec<PlayerReference>,
    pub next_ability: CultAbility,
    pub ability_used_last_night: Option<CultAbility>,
}
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum CultAbility{
    Kill,
    #[default] Convert,
}
impl Cult{
    pub fn on_phase_start(game: &mut Game, phase: PhaseType){
        Cult::set_ordered_cultists(game);
        
        match phase {
            PhaseType::Night => {
                if let Some(ability) = Cult::ability_used_last_night(game) {
                    match ability {
                        CultAbility::Kill => {
                            Cult::set_next_ability(game, CultAbility::Convert);
                        },
                        CultAbility::Convert => {
                            Cult::set_next_ability(game, CultAbility::Kill);
                        }
                    }
                    Cult::set_ability_used_last_night(game, None);
                }


                match Cult::next_ability(game) {
                    CultAbility::Kill => {
                        game.add_message_to_chat_group(ChatGroup::Cult, ChatMessageVariant::CultKillsNext);
                    },
                    CultAbility::Convert => {
                        game.add_message_to_chat_group(ChatGroup::Cult, ChatMessageVariant::CultConvertsNext);
                    }
                }
            },
            _ => {}
        }
    }
    pub fn on_game_start(game: &mut Game) {
        Cult::set_ordered_cultists(game);
    }
    pub fn on_any_death(game: &mut Game, _player: PlayerReference) {
        Cult::set_ordered_cultists(game);
    }
    pub fn on_role_switch(game: &mut Game, _old: Role, _new: Role) {
        Cult::set_ordered_cultists(game);
    }

    pub fn set_ordered_cultists(game: &mut Game){

        let mut cult = game.cult().clone();

        // Remove dead
        cult.ordered_cultists = cult.ordered_cultists.iter().cloned().filter(|p|
            RoleSet::Cult.get_roles().contains(&p.role(game)) &&
            InsiderGroupID::Cult.is_player_in_revealed_group(game, *p) &&
            p.alive(game)
        ).collect();

        // Add new
        for player in PlayerReference::all_players(game){
            if 
                RoleSet::Cult.get_roles().contains(&player.role(game)) &&
                InsiderGroupID::Cult.is_player_in_revealed_group(game, player) &&
                player.alive(game) &&
                !cult.ordered_cultists.contains(&player)
            {
                cult.ordered_cultists.push(player);
            }
        }

        for (i, player_ref) in cult.ordered_cultists.iter().enumerate(){
            let role = if i == 0 {
                RoleState::Apostle(Apostle)
            }else if i == cult.ordered_cultists.len() - 1 {
                RoleState::Zealot(Zealot)
            }else{
                RoleState::Disciple(Disciple)
            };
            
            if player_ref.role(game) == role.role() {continue}
            player_ref.set_role_and_win_condition_and_revealed_group(game, role);
        }

        game.set_cult(cult);
    }

    pub fn next_ability(game: &Game)->CultAbility{
        game.cult().next_ability.clone()
    }
    pub fn set_next_ability(game: &mut Game, ability: CultAbility){
        let mut cult = game.cult().clone();
        
        cult.next_ability = ability;

        game.set_cult(cult);
    }
    pub fn ability_used_last_night(game: &Game)->Option<CultAbility>{
        game.cult().ability_used_last_night.clone()
    }
    pub fn set_ability_used_last_night(game: &mut Game, ability: Option<CultAbility>){
        let mut cult = game.cult().clone();
        
        cult.ability_used_last_night = ability;

        game.set_cult(cult);
    }
}