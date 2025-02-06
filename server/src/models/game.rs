use chrono;

#[derive(Debug, sqlx::FromRow)]
pub struct Game {
    pub game_id: i32,
    pub start_time: chrono::NaiveDateTime,
    pub end_time: Option<chrono::NaiveDateTime>,
    pub winning_team_id: Option<i32>,
}