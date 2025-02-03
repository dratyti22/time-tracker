use anyhow::Result;
use chrono::Utc;
use sqlx::SqlitePool;

pub async fn create_task_db(pool: &SqlitePool, name: &str) -> Result<()> {
    sqlx::query("INSERT INTO tasks (name, start_time) VALUES (?, ?)")
        .bind(name)
        .bind(Utc::now().to_rfc3339())
        .execute(pool)
        .await?;

    Ok(())
}
