use crate::game::{chat::{ChatGroup, ChatMessageVariant}, phase::PhaseType, player::PlayerReference, role::{apostle::Apostle, disciple::Disciple, zealot::Zealot, RoleState}, role_list::Faction, Game};


impl Game {
    pub fn cult(&self)->&Cult{
        &self.cult
    }
    pub fn set_cult(&mut self, cult: Cult){
        self.cult = cult;
    }
}
#[derive(Default, Clone)]
pub struct Cult {
    pub ordered_cultists: Vec<PlayerReference>,
    pub sacrifices_required: Option<u8>
}
impl Cult{
    pub fn on_phase_start(self, game: &mut Game, phase: PhaseType){
        Cult::set_ordered_cultists(self.clone(), game);
        
        if phase == PhaseType::Night {
            if self.can_convert_tonight(game){
                game.add_message_to_chat_group(ChatGroup::Cult, ChatMessageVariant::ApostleCanConvertTonight);
            }else{
                game.add_message_to_chat_group(ChatGroup::Cult, ChatMessageVariant::ApostleCantConvertTonight);
            }
        }
    }
    pub fn on_game_start(self, game: &mut Game) {
        Cult::set_ordered_cultists(self, game);
    }
    pub fn on_any_death(mut self, game: &mut Game){
        self.sacrifices_required = self.sacrifices_required.map(|s| s.saturating_sub(1));
        if let Some(s) = self.sacrifices_required{
            game.add_message_to_chat_group(ChatGroup::Cult, ChatMessageVariant::CultSacrificesRequired { required: s });
        }
        game.set_cult(self.clone());

        Cult::set_ordered_cultists(self.clone(), game);
    }
    pub fn on_role_switch(self, game: &mut Game, actor: PlayerReference) {
        if actor.role(game).faction() == Faction::Cult {
            Cult::set_ordered_cultists(self.clone(), game);
        }

        for a in Cult::get_members(game) {
            for b in Cult::get_members(game) {
                a.insert_role_label(game, b, b.role(game));
                b.insert_role_label(game, a, a.role(game));
            }
        }
    }
    pub fn get_members(game: &Game)->Vec<PlayerReference>{
        PlayerReference::all_players(game).filter(
            |p| p.role(game).faction() == Faction::Cult
        ).collect()
    }


    pub const SACRIFICES_NEEDED: u8 = 2;

    pub fn set_ordered_cultists(mut self, game: &mut Game){
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

        game.set_cult(self);
    }
    pub fn can_convert_tonight(&self, game: &Game)->bool {
        if self.ordered_cultists.len() >= 4 {return false}

        match self.sacrifices_required{
            None => game.day_number() != 1,
            Some(blood) => {
                blood == 0
            }
        }
    }
}