mod cli;
mod clone;
mod cmd;
mod fetch;
mod file;
mod git;
mod mode;
mod progress;
mod regex;

use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli::build().get_matches();
    cli::dispatch(matches).await?;
    Ok(())
}
