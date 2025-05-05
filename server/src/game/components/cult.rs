use crate::game::{chat::{ChatGroup, ChatMessageVariant}, phase::PhaseType, player::PlayerReference, role::Role, role_list::RoleSet, Game};

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
        
        if phase == PhaseType::Night {
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
        }
    }
    pub fn on_any_death(game: &mut Game, _player: PlayerReference) {
        Cult::set_ordered_cultists(game);
    }
    pub fn on_role_switch(game: &mut Game, _old: Role, _new: Role) {
        Cult::set_ordered_cultists(game);
    }

    fn is_living_cultist(game: &Game, player: PlayerReference) -> bool {
        player.alive(game) &&
        RoleSet::Cult.get_roles().contains(&player.role(game)) &&
        InsiderGroupID::Cult.contains_player(game, player)
    }
    pub fn on_game_start(game: &mut Game) {
        let mut apostle = None;
        let mut zealot = None;
        let mut disciples = Vec::new();
        for player in PlayerReference::all_players(game) {
            match player.role(game) {
                Role::Apostle => {
                    assert!(apostle.is_none());
                    apostle = Some(player)
                },
                Role::Zealot => {
                    assert!(zealot.is_none());
                    zealot = Some(player)
                },
                Role::Disciple => disciples.push(player),
                _=>()
            }
        }
        let mut cult = disciples;
        apostle.inspect(|&a|cult.insert(0, a));
        zealot.inspect(|&z|cult.push(z));
        game.cult.ordered_cultists = cult;
        Cult::set_ordered_cultists(game);
    }

    pub fn set_ordered_cultists(game: &mut Game){
        
        let mut cult = game.cult().clone();

        // Remove dead & converted
        cult.ordered_cultists.retain(|&p|Self::is_living_cultist(game, p));

        // Add new
        for player in PlayerReference::all_players(game){
            if Self::is_living_cultist(game, player) && !cult.ordered_cultists.contains(&player) {
                cult.ordered_cultists.push(player);
            }
        }

        let zealot = cult.ordered_cultists.len().saturating_sub(1);
        for (i, player_ref) in cult.ordered_cultists.iter().enumerate(){
            let role =
                if i == 0 {
                    Role::Apostle
                } else if i == zealot {
                    Role::Zealot
                } else {
                    Role::Disciple
                };
            
            if player_ref.role(game) == role {continue}
            player_ref.set_role(game, role.new_state(game));
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