use crate::{game::{attack_power::DefensePower, chat::ChatMessageVariant, event::on_midnight::{MidnightVariables, OnMidnight, OnMidnightPriority}, player::PlayerReference, Game}, vec_set::VecSet};

pub struct FragileVests{
    players: Box<[PlayerVests]>
}
impl FragileVests{
    pub fn new(player_count: u8)->Self{
        Self{
            players: (0..player_count)
                .map(|_|PlayerVests::new())
                .collect()
        }
    }
    fn get(game: &Game)->&Self{
        &game.defense_items
    }
    fn get_mut(game: &mut Game)->&mut Self{
        &mut game.defense_items
    }
    
    pub fn add_defense_item(
        game: &mut Game,
        player: PlayerReference,
        power: DefensePower,
        informed_players: VecSet<PlayerReference>
    ){
        //oh i cant be serious
        Self::get_mut(game)
            .players.get_mut(player.index() as usize)
            .expect("Player reference is valid")
            .add_defense_item(power, informed_players);
    }
    pub fn add_defense_item_midnight(
        game: &mut Game,
        midnight_variables: &mut MidnightVariables, 
        player: PlayerReference,
        power: DefensePower,
        informed_players: VecSet<PlayerReference>
    ){
        
        player.increase_defense_to(game, midnight_variables, power);

        //oh i cant be serious
        Self::get_mut(game)
            .players.get_mut(player.index() as usize)
            .expect("Player reference is valid")
            .add_defense_item(power, informed_players);
    }
    pub fn break_defense_items(
        game: &mut Game,
        player: PlayerReference,
        midnight_variables: &mut MidnightVariables
    ){
        //oh i cant be serious
        Self::get_mut(game)
            .players.get_mut(player.index() as usize)
            .expect("Player reference is valid")
            .break_defense_items(midnight_variables, player);
    }
    pub fn get_defense_from_items(
        game: &Game,
        player: PlayerReference,
    )->DefensePower{
        Self::get(game)
            .players.get(player.index() as usize)
            .expect("Player reference is valid")
            .max_defense()
    }

    pub fn on_midnight(game: &mut Game, _event: &OnMidnight, midnight_variables: &mut MidnightVariables, priority: OnMidnightPriority){
        match priority {
            OnMidnightPriority::Heal => {
                for player in PlayerReference::all_players(game){
                    player.increase_defense_to(game, midnight_variables, Self::get_defense_from_items(game, player));
                }
            }
            OnMidnightPriority::Investigative => {
                for player in PlayerReference::all_players(game){
                    if player.night_attacked(midnight_variables){
                        Self::break_defense_items(game, player, midnight_variables);
                    }
                }
            }
            _ => ()
        }
    }
}

struct PlayerVests{
    items: Vec<PlayerVest>
}
impl PlayerVests{
    fn new()->Self{
        Self { items: Vec::new() }
    }
    fn add_defense_item(&mut self, power: DefensePower, informed_players: VecSet<PlayerReference>){
        self.items.push(PlayerVest { power, informed_players })
    }
    fn break_defense_items(&mut self, midnight_variables: &mut MidnightVariables, player: PlayerReference){
        for item in self.items.iter_mut() {
            for infomed_player in item.informed_players.iter() {
                infomed_player.push_night_message(midnight_variables, ChatMessageVariant::FragileVestBreak{defense: item.power, player_with_vest: player});
            }
        }
        self.items = Vec::new();
    }
    fn max_defense(&self)->DefensePower{
        self.items.iter().fold(
            DefensePower::None, 
            |fold, item|if item.power.is_stronger(fold){item.power}else{fold}
        )
    }
}
struct PlayerVest{
    power: DefensePower,
    informed_players: VecSet<PlayerReference>
}
