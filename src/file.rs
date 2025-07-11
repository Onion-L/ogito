use color_eyre::Result;
use color_eyre::eyre::eyre;
use flate2::read::GzDecoder;
use indicatif::ProgressBar;
use std::ffi::OsString;
use std::fs::{File, create_dir_all};
use std::path::Path;
use std::{fs, io::Write, path::PathBuf};
use tar::Archive;

#[derive(Debug)]
pub struct Repo {
    pub directories: Vec<OsString>,
    pub files: Vec<OsString>,
    pub path: PathBuf,
}

impl Repo {
    pub fn new() -> Self {
        Self {
            directories: Vec::new(),
            files: Vec::new(),
            path: PathBuf::new(),
        }
    }
}

pub fn get_repo(path: &OsString) -> std::io::Result<Repo> {
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

pub fn download_file(url: &str, dir: &str, pb: &ProgressBar) -> Result<PathBuf> {
    pb.set_message("🚀 Downloading...");

    let temp_dir = std::env::temp_dir();
    let file_name = format!(
        "{}_{}.tar.gz",
        dir,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs()
    );
    let temp_file_path = temp_dir.join(file_name);

    let response = reqwest::blocking::get(url).map_err(|e| eyre!(e))?;
    if !response.status().is_success() {
        return Err(eyre!("Download Error: {}", response.status()));
    }

    let bytes = response.bytes().map_err(|e| eyre!(e))?;
    let mut file = File::create(&temp_file_path).expect("Failed to create temp file");
    Write::write_all(&mut file, &bytes).expect("Failed to write temp file");
    Ok(temp_file_path)
}

pub fn extract_archive(temp_file_path: &PathBuf, dir: &str) -> std::io::Result<()> {
    let tar_gz = File::open(temp_file_path).expect("Failed to open the temp file");
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

pub fn get_canonical_path(root: &PathBuf, current_path: &OsString) -> PathBuf {
    let path = root.join(current_path);
    std::fs::canonicalize(&path).unwrap_or_else(|_| path.clone())
}
