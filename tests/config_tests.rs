use ogito::fetch::config::Config;
use ogito::models::Mode;

#[test]
fn test_config_from_all_fields() {
    let dir = String::from("dir");
    let mode = Mode::Git;
    let branch = String::from("branch");
    let config = Config::from(&dir, mode.clone(), true, false, false, Some(&branch));
    assert_eq!(config.dir, &dir);
    assert_eq!(config.mode, mode);
    assert!(config.cache);
    assert!(!config.force);
    assert!(!config.keep_history);
}
