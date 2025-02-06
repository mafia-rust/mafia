use sqlx::Pool;
use crate::models::player::Player;
use std::sync::OnceLock;
use std::env;
use crate::sqlx::Row;
use crate::log;

// Declare a global static variable for the pool
static POOL: OnceLock<Pool<sqlx::Postgres>> = OnceLock::new();

pub async fn initialize_pool() -> &'static Pool<sqlx::Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", database_url);
    let pool = Pool::<sqlx::Postgres>::connect(&database_url).await.expect("Failed to create pool");
    POOL.set(pool).expect("Failed to set global pool");
    POOL.get().unwrap()
}

pub async fn get_player_by_id(pool: &Pool<sqlx::Postgres>, player_id: i32) -> Result<Player, sqlx::Error> {
    sqlx::query_as!(Player, "SELECT * FROM players WHERE player_id = $1", player_id)
        .fetch_one(pool)
        .await
}

pub async fn on_game_start() -> Result<(), sqlx::Error> {
    create_game(POOL.get().unwrap()).await?;
    Ok(())
}

pub async fn create_game(pool: &Pool<sqlx::Postgres>) -> Result<i32, sqlx::Error> {
    let game_id = sqlx::query("INSERT INTO games DEFAULT VALUES RETURNING game_id")
        .fetch_one(pool).await?.get::<i32, _>(0);
    log!(info "Database"; "Game ID: {}", game_id);
    Ok(game_id)
}