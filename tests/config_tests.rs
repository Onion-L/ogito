use ogito::fetch::config::Config;
use ogito::models::mode::Mode;

#[test]
fn test_config_from_all_fields() {
    let repo = String::from("repo");
    let dir = String::from("dir");
    let mode = Mode::Git;
    let config = Config::from(Some(&repo), Some(&dir), mode.clone(), true);
    assert_eq!(config.repo, Some(&repo));
    assert_eq!(config.dir, Some(&dir));
    assert_eq!(config.mode, mode);
    assert!(config.force);
}

#[test]
fn test_config_from_defaults() {
    let config = Config::from(None, None, Mode::Unknown, false);
    assert!(config.repo.is_none());
    assert!(config.dir.is_none());
    assert_eq!(config.mode, Mode::Unknown);
    assert!(!config.force);
}
