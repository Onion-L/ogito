use ogito::models::mode::Mode;

#[test]
fn mode_from_string() {
    let git_str = String::from("git");
    let tar_str = String::from("tar");
    let invalid_str = String::from("zip");

    assert_eq!(Mode::from(&git_str), Mode::Git);
    assert_eq!(Mode::from(&tar_str), Mode::Tar);
    assert_eq!(Mode::from(&invalid_str), Mode::Uninitialized);
}

#[test]
fn mode_into() {
    let git_str = String::from("git");
    let tar_str = String::from("tar");
    let invalid_str = String::from("zip");

    let git_mode: Mode = (&git_str).into();
    let tar_mode: Mode = (&tar_str).into();
    let invalid_mode: Mode = (&invalid_str).into();

    assert_eq!(git_mode, Mode::Git);
    assert_eq!(tar_mode, Mode::Tar);
    assert_eq!(invalid_mode, Mode::Uninitialized);
}
