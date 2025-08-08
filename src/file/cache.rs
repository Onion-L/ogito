use std::{ffi::OsString, path::PathBuf};

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

pub struct CacheMetadata {
    pub owner: String,
    pub repo: String,
    pub hash: String,
}

impl CacheMetadata {
    pub fn new(owner: &str, repo: &str, hash: &str) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            hash: hash.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub cache_dir: PathBuf,
    pub cache_hash_path: PathBuf,
    pub archive_path: PathBuf,
}

impl CacheConfig {
    pub fn new(cache_metadata: &CacheMetadata) -> Self {
        let cache_dir = dirs::cache_dir()
            .expect("Failed to get cache directory")
            .join(".ogito")
            .join("cache");

        let cache_hash_path = cache_dir
            .join(&cache_metadata.owner)
            .join(&cache_metadata.repo)
            .join(&cache_metadata.hash[..2])
            .join(&cache_metadata.hash[2..]);

        let archive_path = cache_hash_path.join("archive.tar.gz");

        Self {
            cache_dir,
            cache_hash_path,
            archive_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Repo;
    use std::path::PathBuf;

    #[test]
    fn test_repo_new() {
        let repo = Repo::new();
        assert!(repo.directories.is_empty());
        assert!(repo.files.is_empty());
        assert_eq!(repo.path, PathBuf::new());
    }
}
