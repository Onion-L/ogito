use std::path::PathBuf;

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
    pub cache_hash_path: PathBuf,
    pub archive_path: PathBuf,
}

impl CacheConfig {
    pub fn new(cache_metadata: &CacheMetadata) -> Self {
        let root_path = dirs::cache_dir()
            .expect("Failed to get cache directory")
            .join(".ogito");

        let cache_path = root_path.join("cache");

        let cache_hash_path = cache_path
            .join(&cache_metadata.owner)
            .join(&cache_metadata.repo)
            .join(&cache_metadata.hash[..2])
            .join(&cache_metadata.hash[2..]);

        let archive_path = cache_hash_path.join("archive.tar.gz");

        Self {
            cache_hash_path,
            archive_path,
        }
    }
}
