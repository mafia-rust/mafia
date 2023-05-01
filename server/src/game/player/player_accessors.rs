use std::collections::HashMap;

use crate::{game::{role::{RoleData, Role}, Game}, network::packet::ToClientPacket};
use super::{Player, PlayerIndex};




impl Player{
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn index(&self) -> &PlayerIndex{
        &self.index
    }
    
    pub fn role(&self) -> Role {
        self.role_data.role()
    }
    pub fn role_data(&self) -> &RoleData{
        &self.role_data
    }
    
    pub fn set_role_data(&mut self, new_role_data: RoleData){
        self.role_data = new_role_data;
        self.send_packet(ToClientPacket::YourRole { role: self.role_data });
    }

    pub fn alive(&self)->&bool{
        &self.alive
    }
    pub fn set_alive(game: &mut Game, player_index: PlayerIndex, alive: bool){
        game.get_unchecked_mut_player(player_index).alive = alive;

        let mut alive_players = vec![];
        for player in game.players.iter(){
            alive_players.push(player.alive().clone());
        }
        game.send_packet_to_all(ToClientPacket::PlayerAlive { alive: alive_players });
    }

    pub fn will(&self)->&String{
        &self.will
    }
    pub fn set_will(&mut self, will: String){
        self.will = will;
        self.send_packet(ToClientPacket::YourWill { will: self.will.clone() });
    }
     
    pub fn role_labels(&self)->&HashMap<PlayerIndex, Role>{
        &self.role_labels
    }  
    pub fn insert_role_label(&mut self, key: PlayerIndex, value: Role){
        self.role_labels.insert(key, value);
        self.send_packet(ToClientPacket::YourRoleLabels { role_labels: self.role_labels.clone() });
    }

}



