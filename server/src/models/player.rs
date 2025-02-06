use chrono::NaiveDateTime;

#[derive(sqlx::FromRow)]
pub struct Player {
    pub player_id: i32,
    pub username: String,
    pub player_number: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    // ...other fields...
}