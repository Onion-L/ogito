use crate::file::{cache::get_cache_root, copy::create_template, path::sanitize_dir};
use crate::manifest::Manifest;
use crate::progress::create_spinner;
use clap::ArgMatches;
use color_eyre::eyre::Ok;
use color_eyre::{eyre::eyre, Result};
use console::{style, Emoji};
use indicatif::HumanDuration;
use std::{fs, time::Instant};

static FINISH: Emoji<'_, '_> = Emoji("🚀", "🚀");
static FIRE: Emoji<'_, '_> = Emoji("🔥", "🔥");

pub async fn local_template(matches: &ArgMatches, template_name: &String) -> Result<()> {
    let started = Instant::now();
    let pb = create_spinner("🔍 Looking for template...");
    
    let cache_path = get_cache_root();
    let config_path = cache_path.join("template.toml");
    let template_path = cache_path.join("templates");

    if !config_path.exists() {
        pb.finish_and_clear();
        return Err(eyre!(
            "No templates configured. Use 'ogito add' to add one."
        ));
    }

    let toml_content = fs::read_to_string(&config_path)?;
    let toml_config: Manifest = toml::from_str(&toml_content)?;
    let path_name = toml_config
        .find(template_name)
        .ok_or_else(|| eyre!("Template '{}' not found", template_name))?;
    let source = template_path.join(path_name);
    
    pb.set_message("📁 Preparing destination directory...");
    let dir_str = match matches.get_one::<String>("dir") {
        Some(dir) => dir,
        None => &template_name.to_string(),
    };
    let dest_path = sanitize_dir(dir_str)?;

    pb.set_message("📋 Copying template files...");
    create_template(source, dest_path)?;
    
    pb.finish_and_clear();

    println!("{} Done in {}", FINISH, HumanDuration(started.elapsed()));
    println!(
        "{} {}",
        FIRE,
        style("The template is prepared and ready to use!")
            .green()
            .bold()
    );
    Ok(())
}
