use ogito::file::{Repo, get_canonical_path};
use std::ffi::OsString;
use std::fs::File;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_repo_new() {
    let repo = Repo::new();
    assert!(repo.directories.is_empty());
    assert!(repo.files.is_empty());
    assert_eq!(repo.path, PathBuf::new());
}

#[test]
fn test_get_canonical_path_success() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();

    let test_file = temp_path.join("test.txt");
    File::create(&test_file).unwrap();

    let canonical = get_canonical_path(&temp_path, &OsString::from("test.txt")).unwrap();

    assert!(canonical.is_absolute());
    assert!(canonical.ends_with("test.txt"));
}

#[test]
fn test_get_canonical_path_nonexistent() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    let result = get_canonical_path(&temp_path, &OsString::from("nonexistent.txt")).unwrap();
    assert!(result.ends_with("nonexistent.txt"));
}
