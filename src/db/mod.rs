use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};

mod models;
mod tasks;
pub use tasks::{create_task_db, complete_task_db, delete_task_db};

pub async fn create_bd_tasks(pool: &SqlitePool) -> anyhow::Result<()> {
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
pub async fn create_bd() -> anyhow::Result<()> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        Sqlite::create_database(DB_URL).await?;
        println!("Database created successfully");
    } else {
        println!("Database already exists");
    }
    Ok(())
}
