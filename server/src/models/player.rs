use chrono::NaiveDateTime;

use crate::game::role::Role;

#[derive(sqlx::FromRow)]
pub struct Player {
    pub player_id: i32,
    pub username: String,
    pub player_number: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub role: String,
    pub team: String,

}



impl Player {
    pub fn new(username: String, role: String, player_number: i32, team: String) -> Self {
        Self {
            //generate unique id
            player_id: Some(chrono::Utc::now().timestamp_micros() as i32 % 1000000000).unwrap(),  // lol
            username,
            player_number: None,
            created_at: Some(chrono::Utc::now().naive_utc()),
            role,
            team: "None".to_string(),
        }
    }

    pub fn update_player_number(&mut self, new_player_number: Option<i32>) {
        self.player_number = new_player_number;
    }
    pub fn set_player_id(&mut self, new_player_id: i32) {
        self.player_id = new_player_id;
    }
    pub fn get_player_id(&self) -> i32 {
        self.player_id
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_created_at(&self) -> Option<&NaiveDateTime> {
        self.created_at.as_ref()
    }
    pub fn get_player_number(&self) -> Option<i32> {
        self.player_number
    }



}