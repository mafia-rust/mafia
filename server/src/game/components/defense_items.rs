use crate::{game::{attack_power::DefensePower, player::PlayerReference, Game}, vec_set::VecSet};

pub struct DefenseItems{
    players: Box<[PlayerDefenseItems]>
}
impl DefenseItems{
    pub fn new(player_count: u8)->Self{
        Self{
            players: (0..player_count)
                .map(|_|PlayerDefenseItems::new())
                .collect()
        }
    }

    fn defense_items(game: &Game)->&Self{
        &game.defense_items
    }
    fn defense_items_mut(game: &mut Game)->&mut Self{
        &mut game.defense_items
    }
    
    pub fn add_defense_item(
        game: &mut Game,
        player: PlayerReference,
        power: DefensePower,
        informed_players: VecSet<PlayerReference>
    ){
        //oh i cant be serious
        Self::defense_items_mut(game)
            .players.get_mut(player.index() as usize).expect("Player reference is valid")
            .add_defense_item(power, informed_players);
    }
}

struct PlayerDefenseItems{
    items: Vec<PlayerDefenseItem>
}
impl PlayerDefenseItems{
    fn new()->Self{
        Self { items: Vec::new() }
    }
    fn add_defense_item(&mut self, power: DefensePower, informed_players: VecSet<PlayerReference>){
        self.items.push(PlayerDefenseItem { power, informed_players })
    }
}
struct PlayerDefenseItem{
    power: DefensePower,
    informed_players: VecSet<PlayerReference>
}
