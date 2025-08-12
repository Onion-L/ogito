use clap::ArgMatches;
use color_eyre::{eyre::eyre, Result};
use dialoguer::Confirm;
use indicatif::{HumanBytes, HumanDuration, ProgressBar, ProgressStyle};
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::file::file::clear_directory;

fn compute_dir_stats(path: &Path) -> Result<(u64, u64)> {
    if !path.exists() {
        return Ok((0, 0));
    }

    let mut file_count: u64 = 0;
    let mut total_bytes: u64 = 0;

    let mut stack: Vec<PathBuf> = vec![path.to_path_buf()];
    while let Some(current) = stack.pop() {
        let read_dir_result = fs::read_dir(&current);
        let entries_iter = match read_dir_result {
            Ok(iter) => iter,
            Err(err) => {
                return Err(eyre!(
                    "Failed to read directory {}: {}",
                    current.display(),
                    err
                ))
            }
        };
        for entry_result in entries_iter {
            let entry = entry_result?;
            let entry_path = entry.path();
            let metadata = match entry.metadata() {
                Ok(md) => md,
                Err(err) => {
                    return Err(eyre!(
                        "Failed to get metadata for {}: {}",
                        entry_path.display(),
                        err
                    ))
                }
            };

            if metadata.is_dir() {
                stack.push(entry_path);
            } else if metadata.is_file() {
                file_count += 1;
                total_bytes = total_bytes.saturating_add(metadata.len());
            }
        }
    }

    Ok((file_count, total_bytes))
}

fn list_dir_entries(path: &Path) -> Result<Vec<PathBuf>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut items: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        items.push(entry.path());
    }
    Ok(items)
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
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
        for item in items {
            if verbose {
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
                    println!("  ↳ Failed: {}", err);
                }
            } else if let Err(err) = fs::remove_file(&item) {
                println!("  ↳ Failed: {}", err);
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
