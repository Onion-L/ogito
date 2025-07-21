use ogito::fetch::config::Config;
use ogito::models::Mode;

#[test]
fn test_config_from_all_fields() {
    let dir = String::from("dir");
    let mode = Mode::Git;
    let config = Config::from(&dir, mode.clone(), true);
    assert_eq!(config.dir, &dir);
    assert_eq!(config.mode, mode);
    assert!(config.force);
}

#[test]
fn test_config_from_defaults() {
    let dir = String::from("dir");
    let config = Config::from(&dir, Mode::Unknown, false);
    assert_eq!(config.dir, &dir);
    assert_eq!(config.mode, Mode::Unknown);
    assert!(!config.force);
}
