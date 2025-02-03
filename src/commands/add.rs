use crate::db::create_task_db;
use anyhow::Result;
use sqlx::SqlitePool;

pub async fn handler_add(pool: &SqlitePool, name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("Task name cannot be empty"));
    }
    create_task_db(pool, name).await?;
    println!("Task '{}' added successfully", name);
    Ok(())
}
