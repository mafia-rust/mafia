use crate::game::{chat::{ChatGroup, ChatMessageVariant}, phase::PhaseType, player::PlayerReference, role::{apostle::Apostle, disciple::Disciple, zealot::Zealot, Role, RoleState}, role_list::Faction, Game};


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
    pub fn on_phase_start(game: &mut Game, phase: PhaseType){
        Cult::set_ordered_cultists(game);
        
        if phase == PhaseType::Night {
            if Cult::can_convert_tonight(game){
                game.add_message_to_chat_group(ChatGroup::Cult, ChatMessageVariant::ApostleCanConvertTonight);
            }else{
                game.add_message_to_chat_group(ChatGroup::Cult, ChatMessageVariant::ApostleCantConvertTonight);
            }
        }
    }
    pub fn on_game_start(game: &mut Game) {
        Cult::set_ordered_cultists(game);
    }
    pub fn on_any_death(game: &mut Game){
        let mut cult = game.cult().clone();

        cult.sacrifices_required = cult.sacrifices_required.map(|s| s.saturating_sub(1));
        if let Some(required) = cult.sacrifices_required{
            game.add_message_to_chat_group(ChatGroup::Cult, ChatMessageVariant::CultSacrificesRequired { required });
        }
        game.set_cult(cult);

        Cult::set_ordered_cultists(game);
    }
    pub fn on_role_switch(game: &mut Game, old: Role, new: Role) {
        if old.faction() == Faction::Cult || new.faction() == Faction::Cult {
            Cult::set_ordered_cultists(game);
        }

        for a in Cult::get_members(game) {
            for b in Cult::get_members(game) {
                a.insert_role_label(game, b);
            }
        }
    }
    
    
    pub fn get_members(game: &Game)->Vec<PlayerReference>{
        PlayerReference::all_players(game).filter(
            |p| p.role(game).faction() == Faction::Cult
        ).collect()
    }


    pub const SACRIFICES_NEEDED: u8 = 2;

    pub fn set_ordered_cultists(game: &mut Game){

        let mut cult = game.cult().clone();

        // Remove dead
        cult.ordered_cultists = cult.ordered_cultists.iter().cloned().filter(|p|
            p.role(game).faction() == Faction::Cult &&
            p.alive(game)
        ).collect();

        // Add new
        for player in PlayerReference::all_players(game){
            if 
                player.role(game).faction() == Faction::Cult &&
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
            player_ref.set_role(game, role);
        }

        game.set_cult(cult);
    }
    pub fn can_convert_tonight(game: &Game)->bool {
        let cult = game.cult();
        if cult.ordered_cultists.len() >= 4 {return false}

        match cult.sacrifices_required {
            None | Some(0) => true,
            _ => false
        }
    }
}