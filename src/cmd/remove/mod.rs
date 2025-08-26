use crate::cmd::clear::dir::{compute_dir_stats, list_dir_entries};
use crate::file::cache::get_cache_root;
use crate::manifest::{Manifest, ManifestFile};
use clap::ArgMatches;
use color_eyre::Result;
use dialoguer::Confirm;
use indicatif::{HumanBytes, ProgressBar, ProgressStyle};
use std::fs;
use std::path::PathBuf;

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let force = matches.get_flag("force");
    let dry_run = matches.get_flag("dry-run");
    let quiet = matches.get_flag("quiet");
    let all = matches.get_flag("all");

    let templates_dir = get_cache_root().join("templates");

    if !templates_dir.exists() {
        if !quiet {
            println!("Template directory does not exist. Nothing to remove.");
        }
        return Ok(());
    }

    if all {
        handle_remove_all(&templates_dir, dry_run, force, quiet).await
    } else {
        let template_names: Vec<String> = matches
            .get_many::<String>("TEMPLATES")
            .unwrap_or_default()
            .map(String::from)
            .collect();

        if template_names.is_empty() {
            println!("No template names provided. Use --all to remove all templates.");
            return Ok(());
        }

        handle_remove_specific(&templates_dir, template_names, dry_run, force, quiet).await
    }
}

async fn handle_remove_all(
    templates_dir: &PathBuf,
    dry_run: bool,
    force: bool,
    quiet: bool,
) -> Result<()> {
    let (file_count, total_bytes) = compute_dir_stats(templates_dir)?;
    let config_path = get_cache_root().join("template.toml");
    let mut config = ManifestFile::load(&config_path)?;

    if file_count == 0 && total_bytes == 0 {
        if !quiet {
            println!("No templates found. Nothing to remove.");
        }
        return Ok(());
    }

    if !quiet {
        println!(
            "Preparing to remove all templates in: {}",
            templates_dir.display()
        );
        println!(
            "Total size: {} ({} files)",
            HumanBytes(total_bytes),
            file_count
        );
    }

    if dry_run {
        println!(
            "
(dry-run) The following top-level templates would be removed:"
        );
        for entry in list_dir_entries(templates_dir)? {
            println!("  - {}", entry.file_name().unwrap().to_string_lossy());
        }
        println!(
            "
(dry-run) Operation finished. No files were changed."
        );
        return Ok(());
    }

    if !force {
        let confirmed = Confirm::new()
            .with_prompt("Are you sure you want to remove all templates?")
            .default(false)
            .interact()?;
        if !confirmed {
            println!("🛑 Operation cancelled.");
            return Ok(());
        }
    }

    fs::remove_dir_all(templates_dir)?;
    Manifest::clear(&mut config.content);

    if !quiet {
        println!("🗑️ All templates removed successfully.");
    }

    Ok(())
}

async fn handle_remove_specific(
    templates_dir: &PathBuf,
    template_names: Vec<String>,
    dry_run: bool,
    force: bool,
    quiet: bool,
) -> Result<()> {
    let mut targets = Vec::new();
    let mut total_bytes = 0;
    let mut total_files = 0;
    let config_path = get_cache_root().join("template.toml");
    let mut config = ManifestFile::load(&config_path)?;

    for name in &template_names {
        let actual_name = Manifest::find(&config.content, name);
        if let Some(name) = actual_name {
            let path = templates_dir.join(name);
            if path.exists() && path.is_dir() {
                let (file_count, bytes) = compute_dir_stats(&path)?;
                total_files += file_count;
                total_bytes += bytes;
                targets.push((name.clone(), path));
            }
        }
    }

    if targets.is_empty() {
        if !quiet {
            println!("No valid templates found to remove.");
        }
        return Ok(());
    }

    if !quiet {
        println!("Preparing to remove the following templates:");
        for (name, _) in &targets {
            println!("  - {}", name);
        }
        println!(
            "Total size: {} ({} files)",
            HumanBytes(total_bytes),
            total_files
        );
    }

    if dry_run {
        println!(
            "
(dry-run) Operation finished. No files were changed."
        );
        return Ok(());
    }

    if !force {
        let confirmed = Confirm::new()
            .with_prompt("Are you sure?")
            .default(false)
            .interact()?;
        if !confirmed {
            println!("🛑 Operation cancelled.");
            return Ok(());
        }
    }

    let spinner = if !quiet {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::with_template("{spinner} Removing templates...")
                .unwrap()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
        );
        pb.enable_steady_tick(std::time::Duration::from_millis(80));
        Some(pb)
    } else {
        None
    };

    for (name, path) in &targets {
        fs::remove_dir_all(path)?;
        config.remove_template(name);

        if let Some(pb) = &spinner {
            pb.set_message(format!("Removed '{}'", name));
        }
    }

    if let Some(pb) = spinner {
        pb.finish_and_clear();
    }

    if !quiet {
        println!("🗑️ {} template(s) removed successfully.", targets.len());
    }

    config.save()?;

    Ok(())
}
