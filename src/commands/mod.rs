mod add;
mod cli;
mod complete;

use crate::db::delete_task_db;
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
