use chrono;

#[derive(Debug, sqlx::FromRow)]
pub struct Player {
    pub player_id: i32,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
}