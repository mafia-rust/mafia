use sqlx::{Pool, Postgres, Transaction};
use crate::game;
use crate::models::player::Player;
use crate::game::components::save_game_statistics::GameStatistics;
use crate::game::role::Role;
use std::sync::OnceLock;
use std::env;
use sqlx::Row;

// Declare a global static variable for the pool
static POOL: OnceLock<Pool<sqlx::Postgres>> = OnceLock::new();


pub async fn initialize_pool() -> Result<&'static Pool<sqlx::Postgres>, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", database_url);
    let pool = Pool::<sqlx::Postgres>::connect(&database_url).await?;
    POOL.set(pool).expect("Failed to set global pool");
    Ok(POOL.get().unwrap())
}


pub async fn on_game_start(game: GameStatistics) -> Result<(), sqlx::Error> {
    if let Err(e) = create_game(POOL.get().unwrap()).await {
        eprintln!("Failed to create game: {:?}", e);
        return Err(e);
    }

    let game_players: Vec<Player> = game.all_players();

    if let Err(e) = populate_game(POOL.get().unwrap(), game_players)
    .await
    {
        eprintln!("Failed to populate game: {:?}", e);
        return Err(e);
    }
    Ok(())
}

pub async fn on_game_end(game: GameStatistics) -> Result<(), sqlx::Error> {
    Ok(())
}


//Todo queries for basic winrate information:
//
//update game end timestamp.
//mark game as valid or invalid
//update game results
//update player



pub async fn create_game(pool: &Pool<sqlx::Postgres>) -> Result<i32, sqlx::Error> {
    let game_id = sqlx::query("INSERT INTO games DEFAULT VALUES RETURNING game_id")
        .fetch_one(pool)
        .await?
        .get::<i32, _>(0);
    Ok(game_id)
}

pub async fn populate_game(
    pool: &Pool<Postgres>,
    players: Vec<Player>,
) -> Result<i32, sqlx::Error> {
    if players.is_empty() {
        return Err(sqlx::Error::Protocol("No players provided".to_string()));
    }

    // Start a transaction
    let mut transaction = pool.begin().await?;

    // Insert a new game into the `games` table
    let game_id: i32 = sqlx::query!(
        r#"
        INSERT INTO games (start_time, is_valid)
        VALUES (CURRENT_TIMESTAMP, true)
        RETURNING game_id
        "#
    )
    .fetch_one(&mut *transaction)
    .await?
    .game_id;

    // Collect player IDs, roles, and teams
    let player_ids: Vec<i32> = players.iter().map(|player| player.player_id).collect();
    let roles: Vec<String> = players.iter().map(|player| player.role.clone()).collect();
    let teams: Vec<String> = players.iter().map(|player| player.team.clone()).collect();

    // Insert players into the `game_players` table
    sqlx::query!(
        r#"
        INSERT INTO game_players (game_id, player_id, role, team)
        SELECT $1, unnest($2::int[]), unnest($3::text[]), unnest($4::text[])
        "#,
        game_id,
        &player_ids,
        &roles,
        &teams
    )
    .execute(&mut *transaction)
    .await?;

    // Commit the transaction
    transaction.commit().await?;

    // Return the game_id for reference
    Ok(game_id)
}