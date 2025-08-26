use crate::{file::cache::get_cache_root, manifest::ManifestFile, progress::create_spinner};
use clap::ArgMatches;
use color_eyre::{eyre::eyre, Result};
use console::style;

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let cache_path = get_cache_root();
    let config_path = cache_path.join("template.toml");

    if !config_path.exists() {
        return Err(eyre!(
            "No templates configured. Use 'ogito add' to add one first."
        ));
    }

    let config = ManifestFile::load(&config_path)?;
    let templates = &config.content.templates;

    if templates.is_empty() {
        return Err(eyre!("No templates found to update."));
    }

    let force = matches.get_flag("force");
    let dry_run = matches.get_flag("dry-run");
    let quiet = matches.get_flag("quiet");

    let templates_to_update: Vec<String> = if matches.get_flag("all") {
        templates.keys().cloned().collect()
    } else if let Some(template_names) = matches.get_many::<String>("TEMPLATES") {
        template_names.cloned().collect()
    } else {
        return Err(eyre!(
            "No templates specified. Use --all to update all templates or specify template names."
        ));
    };

    if dry_run {
        if !quiet {
            println!(
                "{} Would update {} template(s):",
                style("ℹ️").yellow(),
                templates_to_update.len()
            );
            for template_name in &templates_to_update {
                println!("  - {}", template_name);
            }
        }
        return Ok(());
    }

    let mut updated_count = 0;
    let mut failed_count = 0;

    for template_name in templates_to_update {
        if let Some(actual_name) = config.content.find(&template_name) {
            if let Some(template) = templates.get(actual_name) {
                match update_template(actual_name, template, force, quiet).await {
                    Ok(_) => {
                        updated_count += 1;
                        if !quiet {
                            if actual_name == &template_name {
                                println!(
                                    "{} Updated template: {}",
                                    style("✅").green(),
                                    template_name
                                );
                            } else {
                                println!(
                                    "{} Updated template: {} (via alias: {})",
                                    style("✅").green(),
                                    actual_name,
                                    template_name
                                );
                            }
                        }
                    }
                    Err(e) => {
                        failed_count += 1;
                        if !quiet {
                            if actual_name == &template_name {
                                eprintln!(
                                    "{} Failed to update template '{}': {}",
                                    style("❌").red(),
                                    template_name,
                                    e
                                );
                            } else {
                                eprintln!(
                                    "{} Failed to update template '{}' (via alias: '{}'): {}",
                                    style("❌").red(),
                                    actual_name,
                                    template_name,
                                    e
                                );
                            }
                        }
                    }
                }
            } else {
                failed_count += 1;
                if !quiet {
                    eprintln!(
                        "{} Template not found: {}",
                        style("❌").red(),
                        template_name
                    );
                }
            }
        } else {
            failed_count += 1;
            if !quiet {
                eprintln!(
                    "{} Template not found: {}",
                    style("❌").red(),
                    template_name
                );
            }
        }
    }

    if !quiet {
        println!(
            "{} Update completed: {} succeeded, {} failed",
            style("✨").cyan(),
            updated_count,
            failed_count
        );
    }

    Ok(())
}

async fn update_template(
    template_name: &str,
    template: &crate::manifest::Template,
    force: bool,
    quiet: bool,
) -> Result<()> {
    use crate::regex::is_valid_url;

    if !is_valid_url(&template.url)? {
        return Err(eyre!(
            "Invalid URL for template '{}': {}",
            template_name,
            template.url
        ));
    }

    let templates_dir = get_cache_root().join("templates");
    let destination = templates_dir.join(template_name);

    if !destination.exists() {
        return Err(eyre!(
            "Template directory does not exist: {}",
            template_name
        ));
    }

    if !quiet {
        let spinner = create_spinner(&format!("Updating template: {}", template_name));
        spinner.set_message("Fetching latest changes...");
    }

    let dest_string = destination
        .to_str()
        .ok_or_else(|| eyre!("Failed to convert destination path to string"))?
        .to_string();

    let clone_config =
        crate::fetch::config::Config::from(&dest_string, crate::mode::Mode::Git, force, true, None);

    // Use force_clone to remove existing directory and re-clone
    crate::clone::force_clone(&template.url, &dest_string, &clone_config).await?;

    if !quiet {
        println!(
            "{} Template updated successfully: {}",
            style("✅").green(),
            template_name
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::manifest::{Manifest, Template};
    use std::collections::BTreeMap;

    #[test]
    fn test_find_template_by_alias() {
        let mut templates = BTreeMap::new();
        templates.insert(
            "test-template".to_string(),
            Template {
                description: Some("Test template".to_string()),
                alias: Some("test".to_string()),
                url: "https://example.com/test.git".to_string(),
            },
        );

        let manifest = Manifest { templates };

        // Test finding by exact name
        assert_eq!(
            manifest.find(&"test-template".to_string()),
            Some(&"test-template".to_string())
        );

        // Test finding by alias
        assert_eq!(
            manifest.find(&"test".to_string()),
            Some(&"test-template".to_string())
        );

        // Test not found
        assert_eq!(manifest.find(&"nonexistent".to_string()), None);
    }
}
