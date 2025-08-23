use crate::file::cache::get_cache_root;
use crate::manifest::ManifestFile;
use clap::ArgMatches;
use color_eyre::{eyre::eyre, Result};

pub async fn run(matches: &ArgMatches) -> Result<()> {
    // let cache_path = get_cache_root();
    let config_path = get_cache_root().join("template.toml");

    if !config_path.exists() {
        return Err(eyre!(
            "No templates configured. Use 'ogito add' to add one."
        ));
    }

    let config = ManifestFile::load(&config_path)?;
    let templates = &config.content.templates;

    if templates.is_empty() {
        println!("No templates found.");
        return Ok(());
    }

    for (name, template) in templates {
        println!("{}", name);
        if let Some(description) = &template.description {
            println!("  - {}", description);
        }
        if let Some(alias) = &template.alias {
            println!("  Alias: {}", alias);
        }
    }

    Ok(())
}
