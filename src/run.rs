use crate::commands::{handle_get, handler_add, handler_complete_task, handler_delete_task, handler_get_all_tasks, Cli};
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
        handler_delete_task(&pool, id).await?;
    } else if let Some(id) = cli.get_complete() {
        handler_complete_task(&pool, id).await?;
    } else if cli.get_list() {
        handler_get_all_tasks(&pool).await?;
    } else if let Some(id) = cli.get_get() {
        handle_get(&pool,id).await?;
    } else {
        println!("No valid command provided. Use --help for more information.");
    }
    Ok(())
}
