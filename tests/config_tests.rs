use ogito::fetch::config::Config;
use ogito::models::{mode::Mode, site::Site};

#[test]
fn test_config_from_all_fields() {
    let repo = String::from("repo");
    let dir = String::from("dir");
    let site = Site::Github;
    let mode = Mode::Git;
    let config = Config::from(Some(&repo), Some(&dir), Some(site), Some(&mode), true);
    assert_eq!(config.repo, Some(&repo));
    assert_eq!(config.dir, Some(&dir));
    assert_eq!(config.site, Some(site));
    assert_eq!(config.mode, Some(&mode));
    assert!(config.force);
}

#[test]
fn test_config_from_defaults() {
    let config = Config::from(None, None, None, None, false);
    assert!(config.repo.is_none());
    assert!(config.dir.is_none());
    assert!(config.site.is_none());
    assert!(config.mode.is_none());
    assert!(!config.force);
}
