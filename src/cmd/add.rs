use clap::ArgMatches;
use color_eyre::{
    eyre::{eyre, OptionExt},
    Result,
};

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let url = matches
        .get_one::<String>("URL")
        .ok_or_eyre("URL is required")?;
    let name = matches.get_one::<String>("name");
    let description = matches.get_one::<String>("description");
    let alias = matches.get_one::<String>("alias");
    let force = matches.get_flag("force");

    Ok(())
}
