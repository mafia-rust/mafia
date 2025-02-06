use crate::{database_resources::database_queries, game::{player::PlayerReference, Game}, models::player::Player};

pub struct GameStatistics {
    stuff_to_save: bool,
    other_stuff: i8,
    all_players: Vec<PlayerReference>,
}

impl GameStatistics {
    pub fn on_game_start(game: &Game) {
        let all_players = Game::get_all_players(game).expect("Failed to get all players");
        tokio::spawn(async move {
            let mut game_statistics = GameStatistics {
                stuff_to_save: true,
                other_stuff: 0,
                all_players,
            };
            database_queries::on_game_start(game_statistics).await.unwrap();
        });
    }

    pub fn on_game_end(game: &Game) {
        tokio::spawn(async move {
            let game_statistics = GameStatistics {
                stuff_to_save: true,
                other_stuff: 0,
                all_players: vec![],
            };
            database_queries::on_game_end(game_statistics).await.unwrap();
        });
    }

    pub fn all_players(&self) -> Vec<Player> { self.all_players.iter().map(|p| p.player.clone()).collect()

    }

    
}