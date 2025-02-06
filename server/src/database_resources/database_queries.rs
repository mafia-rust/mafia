use sqlx::Pool;
use crate::models::player::Player;
use crate::game::Game;
use dotenv::dotenv;
use std::sync::OnceLock;
use std::env;

//create a variable to hold the connection pool

static POOL: OnceLock<Pool<sqlx::Postgres>> = OnceLock::new();
pub async fn initialize_pool() -> &'static Pool<sqlx::Postgres> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //print the database url
    println!("Database URL: {}", database_url);
    let pool = Pool::<sqlx::Postgres>::connect(&database_url).await.expect("Failed to create pool");
    POOL.set(pool).expect("Failed to set global pool");
    POOL.get().unwrap()
}

pub async fn get_player_by_id(pool: &Pool<sqlx::Postgres>, player_id: i32) -> Result<Player, sqlx::Error> {
    sqlx::query!("SELECT * FROM players WHERE player_id = $1", player_id)
        .fetch_one(pool)
        .await
}

pub async fn create_game(pool: &Pool<sqlx::Postgres>) -> Result<i32, sqlx::Error> {
    let row = sqlx::query!(
        "INSERT INTO public.games (start_time, winning_team_id, is_valid) VALUES (now(), NULL, false) RETURNING game_id"
    )
    .fetch_one(pool)
    .await?;

    Ok(row.game_id)
}

pub async fn create_players(pool: &Pool<sqlx::Postgres>, players: Vec<Player>) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;

    for player in players {
        sqlx::query!(
            "INSERT INTO public.players (player_id, player_name, team_id) VALUES ($1, $2, $3)",
            player.player_id,
            player.player_name,
            player.team_id
        )
        .execute(&mut transaction)
        .await?;
    }

    transaction.commit().await?;
    Ok(())
}

pub async fn add_players_to_game(pool: &Pool<sqlx::Postgres>, game_id: i32, players: Vec<(i32, i32)>) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;

    for (player_id, team_id) in players {
        sqlx::query!(
            "INSERT INTO public.game_players (game_id, player_id, team_id) VALUES ($1, $2, $3)",
            game_id,
            player_id,
            team_id
        )
        .execute(&mut transaction)
        .await?;
    }

    transaction.commit().await?;
    Ok(())
}


pub async fn on_game_start(game: &Game) -> Result<(), sqlx::Error> {
    // Update the game to mark it as valid
    sqlx::query!(
        "SELECT public.games",
        game.game_id
    )
    .execute(POOL)
    .await?;

    Ok(())
}