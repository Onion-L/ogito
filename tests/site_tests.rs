use ogito::models::site::Site;

#[test]
fn test_to_str() {
    assert_eq!(Site::Github.to_str(), "github");
    assert_eq!(Site::Gitlab.to_str(), "gitlab");
}

#[test]
fn test_from_str_option() {
    assert_eq!(Site::from_str("github"), Some(Site::Github));
    assert_eq!(Site::from_str("gitlab"), Some(Site::Gitlab));
    assert_eq!(Site::from_str("gitee"), None);
}
