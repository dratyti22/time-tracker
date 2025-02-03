use crate::commands::{handler_add, Cli};
use crate::db::{create_bd, create_bd_tasks, DB_URL};
use anyhow::Result;
use sqlx::sqlite::SqlitePool;

pub async fn run(cli: Cli) -> Result<()> {
    create_bd().await?;
    let pool = SqlitePool::connect(DB_URL).await?;
    create_bd_tasks(&pool).await?;

    if let Some(task_name) = cli.get_add() {
        if !task_name.is_empty() {
            handler_add(&pool, task_name).await?;
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
