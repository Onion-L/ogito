use ogito::models::mode::Mode;

#[test]
fn mode_to_str() {
    assert_eq!(Mode::Git.to_str(), "git");
    assert_eq!(Mode::Tar.to_str(), "tar");
}

#[test]
fn mode_from_str_valid() {
    assert_eq!(Mode::from_str("git"), Ok(Mode::Git));
    assert_eq!(Mode::from_str("tar"), Ok(Mode::Tar));
}

#[test]
fn mode_from_str_invalid() {
    let err = Mode::from_str("zip");
    assert!(err.is_err());
    assert_eq!(err.unwrap_err(), "Invalid mode: zip");
}
