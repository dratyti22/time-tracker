use crate::commands::Cli;
use anyhow::Result;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePool;
use sqlx::Sqlite;

async fn create_bd_tasks(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            start_time TEXT NOT NULL,
            end_time TEXT
        )",
    )
    .execute(pool)
    .await?;
    Ok(())
}
pub const DB_URL: &str = "sqlite://time_tracker.db";
async fn create_bd()->Result<()>{
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        Sqlite::create_database(DB_URL).await?;
        println!("Database created successfully");
    } else {
        println!("Database already exists");
    }
    Ok(())
}

pub async fn run(cli: Cli) -> Result<()> {
    create_bd().await?;
    let pool = SqlitePool::connect(DB_URL).await?;
    create_bd_tasks(&pool).await?;
    if let Some(task_name) = cli.get_add() {
        if !task_name.is_empty() {
            println!("Adding task: {:?}", cli.get_add());
        }
    } else if let Some(id) = cli.get_delete() {
        println!("Deleting task with ID: {}", id);
    } else if let Some(id) = cli.get_complete() {
        println!("Completing task with ID: {}", id);
    } else if cli.get_list() {
        println!("Listing all tasks");
    } else if let Some(id) = cli.get_status() {
        println!("Setting status for task with ID: {}", id);
    } else {
        println!("No valid command provided. Use --help for more information.");
    }
    Ok(())
}
