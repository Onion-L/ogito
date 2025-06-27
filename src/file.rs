use std::{fs, path::PathBuf};

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
