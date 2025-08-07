use crate::file::cache::{CacheConfig, CacheMetadata, Repo};
use color_eyre::Result;
use color_eyre::eyre::eyre;
use flate2::read::GzDecoder;
use std::ffi::OsString;
use std::fs::{File, create_dir_all};
use std::path::Path;
use std::{fs, io::Write, path::PathBuf};
use tar::Archive;

pub fn get_repo(path: &OsString) -> Result<Repo> {
    let path = std::env::current_dir()?.join(path);
    let mut repo = Repo::new();
    repo.path = path.clone();

    let entries: Vec<PathBuf> = fs::read_dir(path)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();

    for entry in entries.iter() {
        if let Some(name) = entry.file_name().map(OsString::from) {
            if entry.is_dir() {
                repo.directories.push(name);
            } else {
                repo.files.push(name);
            }
        }
    }

    Ok(repo)
}

pub async fn download_file(url: &str, cache_metadata: &CacheMetadata) -> Result<PathBuf> {
    let cache = CacheConfig::new(cache_metadata);
    std::fs::create_dir_all(&cache.cache_dir)?;

    std::fs::create_dir_all(&cache.cache_hash_path)?;

    let archive_path = cache.archive_path;

    if !archive_path.exists() {
        let response = reqwest::get(url).await?;
        if !response.status().is_success() {
            return Err(eyre!("Download Error: {}", response.status()));
        }

        let bytes = response.bytes().await?;
        let mut file = File::create(&archive_path)?;
        Write::write_all(&mut file, &bytes)?;
        return Ok(archive_path);
    }

    Ok(archive_path)
}

pub fn extract_archive(temp_file_path: &PathBuf, dir: &str) -> std::io::Result<()> {
    let tar_gz = File::open(temp_file_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    for entry_result in archive.entries()? {
        let mut entry = entry_result?;
        let path = entry.path()?.to_owned();
        let mut components = path.components();
        if components.next().is_none() {
            continue;
        }
        let new_path: PathBuf = components.collect();
        if new_path.as_os_str().is_empty() {
            continue;
        }
        let target_path = Path::new(dir).join(new_path);
        if let Some(parent) = target_path.parent() {
            create_dir_all(parent)?;
        }

        entry.unpack(&target_path)?;
    }

    Ok(())
}

pub fn get_canonical_path(root: &PathBuf, current_path: &OsString) -> Result<PathBuf> {
    let path = root.join(current_path);
    std::fs::canonicalize(&path)
        .map_err(|e| eyre!("Failed to canonicalize path: {}", e))
        .or_else(|_| Ok(path.clone()))
}
