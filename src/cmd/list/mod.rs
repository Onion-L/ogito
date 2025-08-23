use crate::file::cache::get_cache_root;
use crate::manifest::ManifestFile;
use clap::ArgMatches;
use color_eyre::{eyre::eyre, Result};
use comfy_table::{Cell, ContentArrangement, Table};

pub async fn run(_matches: &ArgMatches) -> Result<()> {
    //TODO fuzzy search
    let cache_path = get_cache_root();
    let config_path = cache_path.join("template.toml");

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

    let mut table = Table::new();
    table.set_header(vec!["Name", "Description", "Alias"]);

    for (name, template) in templates {
        let description = match &template.description {
            Some(des) => des,
            None => &"None".to_string(),
        };
        let alias = match &template.alias {
            Some(alias) => alias,
            None => &"None".to_string(),
        };
        table.add_row(vec![
            Cell::new(name),
            Cell::new(description),
            Cell::new(alias),
        ]);
    }
    table.set_content_arrangement(ContentArrangement::Dynamic);
    println!("{table}");

    Ok(())
}
