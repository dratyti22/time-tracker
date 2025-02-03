mod commands;
mod run;

use anyhow::Result;
use clap::Parser;
use commands::Cli;
use run::run;


#[tokio::main]
async fn main() -> Result<()> {
    let cli =Cli::parse();
    run(cli).await?;
    Ok(())
}
