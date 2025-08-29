pub mod dir;

use crate::file::clear_directory;
use clap::ArgMatches;
use color_eyre::Result;
use dialoguer::Confirm;
use dir::{compute_dir_stats, list_dir_entries};
use indicatif::{HumanBytes, HumanDuration, ProgressBar, ProgressStyle};
use std::fs::{self};
use std::time::Instant;

pub fn run(matches: &ArgMatches) -> Result<()> {
    let force = matches.get_flag("force");
    let dry_run = matches.get_flag("dry-run");
    let verbose = matches.get_flag("verbose");

    let cache_path = dirs::cache_dir()
        .expect("Failed to get cache directory")
        .join("ogito")
        .join("cache");

    if !cache_path.exists() {
        println!("📦 Cache directory not found: {}", cache_path.display());
        return Ok(());
    }

    let (file_count_before, total_bytes_before) = compute_dir_stats(&cache_path)?;
    if file_count_before == 0 && total_bytes_before == 0 {
        println!("📦 Cache is already empty: {}", cache_path.display());
        return Ok(());
    }

    println!(
        "Clearing cache: {} ({} files, {} bytes)",
        cache_path.display(),
        file_count_before,
        HumanBytes(total_bytes_before)
    );

    if dry_run {
        let items = list_dir_entries(&cache_path)?;
        if items.is_empty() {
            println!("(dry-run) No items to clear");
            return Ok(());
        }
        println!("(dry-run) Items to be removed:");
        if verbose {
            for item in items {
                println!("  - {}", item.display());
            }
        }
        println!(
            "(dry-run) Estimated space to be freed: {} ({} files)",
            HumanBytes(total_bytes_before),
            file_count_before
        );
        return Ok(());
    }

    if !force {
        let confirm = Confirm::new()
            .with_prompt("Are you sure you want to clear the cache?")
            .default(false)
            .interact()?;
        if !confirm {
            println!("🛑 Operation cancelled");
            return Ok(());
        }
    }

    let started = Instant::now();

    let spinner = if verbose {
        None
    } else {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::with_template("{spinner} Clearing cache...")
                .unwrap()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
        );
        pb.enable_steady_tick(std::time::Duration::from_millis(80));
        Some(pb)
    };

    if verbose {
        let items = list_dir_entries(&cache_path)?;
        for item in items {
            println!("Removing: {}", item.display());
            if item.is_dir() {
                if let Err(err) = fs::remove_dir_all(&item) {
                    println!("  ↳ Failed: {err}");
                }
            } else if let Err(err) = fs::remove_file(&item) {
                println!("  ↳ Failed: {err}");
            }
        }
    } else {
        clear_directory(&cache_path)?;
    }

    if let Some(pb) = spinner {
        pb.finish_and_clear();
    }

    let duration = HumanDuration(started.elapsed());
    let (_, total_bytes_after) = compute_dir_stats(&cache_path)?;
    let freed = total_bytes_before.saturating_sub(total_bytes_after);

    println!(
        "📦 Cache cleared, freed {} in {}",
        HumanBytes(freed),
        duration
    );

    Ok(())
}
