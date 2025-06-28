use flate2::read::GzDecoder;
use indicatif::ProgressBar;
use std::fs::{File, create_dir_all};
use std::path::Path;
use std::{fs, io::Write, path::PathBuf};
use tar::Archive;

pub fn print_file(path: &str, indent: usize) -> Result<(), std::io::Error> {
    let entries: Vec<PathBuf> = fs::read_dir(path)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();

    for entry in &entries {
        if entry.is_dir() {
            let dir_name = entry
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();

            println!("{:indent$}📁 {}", "", dir_name, indent = indent);
            print_file(&entry.to_string_lossy(), indent + 2)?;
        }
    }

    for entry in &entries {
        if entry.is_file() {
            let file_name = entry
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();

            println!("{:indent$}📄 {}", "", file_name, indent = indent);
        }
    }

    Ok(())
}

pub fn download_file(url: &str, dir: &str, pb: &ProgressBar) -> Result<PathBuf, String> {
    pb.set_message("🚀 Downloading...");

    // path of temp directory in OS
    let temp_dir = std::env::temp_dir();
    let file_name = format!(
        "{}_{}.tar.gz",
        dir,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    let temp_file_path = temp_dir.join(file_name);

    let response = reqwest::blocking::get(url).map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("Dowload Error: {}", response.status()));
    }

    let bytes = response.bytes().map_err(|e| e.to_string())?;
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
        let path_str = path.to_string_lossy();

        if path_str.matches('/').count() == 0 {
            continue;
        }

        let components: Vec<&str> = path_str.split("/").collect();
        let new_path = components[1..].join("/");
        let target_path = Path::new(dir).join(new_path);

        if let Some(parent) = target_path.parent() {
            create_dir_all(parent)?;
        }

        entry.unpack(&target_path)?;
    }
    Ok(())
}
