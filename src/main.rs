mod cli;
mod clone;
mod cmd;
mod fetch;
mod file;
mod git;
mod manifest;
mod mode;
mod progress;
mod regex;

use color_eyre::Result;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    color_eyre::install()?;
    let matches = cli::build().get_matches();
    cli::dispatch(matches).await?;
    Ok(())
}
