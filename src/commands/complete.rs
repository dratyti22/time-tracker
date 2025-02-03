use crate::db::complete_task_db;
use anyhow::Result;
use sqlx::SqlitePool;

pub async fn handler_complete_task(pool: &SqlitePool, id: i64) -> Result<()> {
    complete_task_db(pool, id).await?;
    println!("Task with ID {} completed successfully", id);
    Ok(())
}
