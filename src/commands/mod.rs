mod add;
mod cli;
mod complete;

use crate::db::{delete_task_db, get_all_tasks_db, get_task_db};
pub use add::handler_add;
use anyhow::Result;
pub use cli::Cli;
pub use complete::handler_complete_task;
use sqlx::SqlitePool;

pub async fn handler_delete_task(pool: &SqlitePool, id: u16) -> Result<()> {
    delete_task_db(pool, id).await?;
    println!("Task with ID {} deleted successfully", id);
    Ok(())
}

pub async fn handler_get_all_tasks(pool: &SqlitePool) -> Result<()> {
    let tasks = get_all_tasks_db(pool).await?;

    if tasks.is_empty() {
        println!("No tasks found");
        return Ok(());
    }

    println!(
        "{:<5} {:<20} {:<25} {:<25}",
        "ID", "Name", "Start Time", "End Time"
    );
    println!("{}", "-".repeat(80));

    for task in tasks {
        println!(
            "{:<5} {:<20} {:<25} {:<25}",
            task.id,
            task.name,
            task.start_time,
            task.end_time.unwrap_or_else(|| "In progress".to_string())
        );
    }

    Ok(())
}

pub async fn handle_get(pool: &SqlitePool, id: u16) -> Result<()> {
    match get_task_db(pool, id).await? {
        Some(task) => {
            println!("Task details:");
            println!("{:<10} {}", "ID:", task.id);
            println!("{:<10} {}", "Name:", task.name);
            println!("{:<10} {}", "Start:", task.start_time);
            println!("{:<10} {}", "End:", task.end_time.unwrap_or("In progress".into()));
        }
        None => println!("Task with ID {} not found", id),
    }
    Ok(())
}