use color_eyre::{eyre::eyre, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub fn compute_dir_stats(path: &Path) -> Result<(u64, u64)> {
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

pub fn list_dir_entries(path: &Path) -> Result<Vec<PathBuf>> {
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
