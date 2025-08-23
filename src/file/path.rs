use color_eyre::{eyre::eyre, Result};
use std::path::{Component, Path, PathBuf};

// A path is considered safe if it is within the current working directory.
// Will return `Err` if the path is invalid or outside the current working directory.
pub fn sanitize_dir(path: &str) -> Result<PathBuf> {
    let target_path = Path::new(path);
    let current_dir = std::env::current_dir()?;

    // If the path is absolute, it must be a sub-path of the current directory.
    if target_path.is_absolute() {
        return if target_path.starts_with(&current_dir) {
            Ok(target_path.to_path_buf())
        } else {
            Err(eyre!(
                "forbidden: absolute path '{}' is outside the current working directory",
                target_path.display()
            ))
        };
    }

    // For relative paths, check for traversal components.
    for component in target_path.components() {
        if let Component::ParentDir = component {
            return Err(eyre!(
                "forbidden: path '{}' contains traversal components ('..')",
                path
            ));
        }
    }

    let resolved_path = current_dir.join(target_path);
    Ok(resolved_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_sanitize_dir_valid_relative() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();

        assert!(sanitize_dir("test").is_ok());
        assert!(sanitize_dir("a/b/c").is_ok());
        assert!(sanitize_dir("./a/b").is_ok());
    }

    #[test]
    fn test_sanitize_dir_invalid_traversal() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();

        assert!(sanitize_dir("../test").is_err());
        assert!(sanitize_dir("../../test").is_err());
        assert!(sanitize_dir("a/../../b").is_err());
    }

    #[test]
    fn test_sanitize_dir_absolute() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();

        let valid_abs = dir.path().join("valid");
        assert!(sanitize_dir(valid_abs.to_str().unwrap()).is_ok());

        let outside_path = tempdir().unwrap();
        assert!(sanitize_dir(outside_path.path().to_str().unwrap()).is_err());
    }
}
