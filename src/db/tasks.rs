use anyhow::Result;
use chrono::Utc;
use sqlx::SqlitePool;
use crate::db::Task;

pub async fn create_task_db(pool: &SqlitePool, name: &str) -> Result<()> {
    let end_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    sqlx::query("INSERT INTO tasks (name, start_time) VALUES (?, ?)")
        .bind(name)
        .bind(end_time)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn complete_task_db(pool: &SqlitePool, id: i64) -> Result<()> {
    let end_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    sqlx::query("UPDATE tasks SET end_time = ? WHERE id = ?")
        .bind(end_time)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_task_db(pool: &SqlitePool, id: u16) -> Result<()> {
    sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_all_tasks_db(pool:&SqlitePool)-> Result<Vec<Task>> {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(pool)
        .await?;
    Ok(tasks)
}

pub async fn get_task_db(pool: &SqlitePool, id: u16) -> Result<Option<Task>> {
    let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
        .bind(id as i64)  // Конвертируем u16 в i64 для SQLite
        .fetch_optional(pool)
        .await?;
    Ok(task)
}
