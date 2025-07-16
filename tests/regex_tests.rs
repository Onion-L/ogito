use ogito::regex::{extract_host, extract_path, is_github_url};

#[test]
fn test_is_github_url() {
    assert!(is_github_url("https://github.com/owner/repo"));
    assert!(is_github_url("github.com/owner/repo"));
    assert!(is_github_url("https://github.com/owner/repo.git"));
    assert!(!is_github_url("https://gitlab.com/owner/repo"));
    assert!(!is_github_url("random string"));
}

#[test]
fn test_extract_path() {
    let url = "https://github.com/owner/repo";
    let result = extract_path(url);
    assert_eq!(result, Some(("owner", "repo")));

    let url_git = "https://github.com/owner/repo.git";
    let result = extract_path(url_git);
    assert_eq!(result, Some(("owner", "repo")));

    let invalid = "https://gitlab.com/owner/repo";
    assert!(extract_path(invalid).is_some());
}

// ... existing code ...
#[test]
fn test_extract_host() {
    let github_url = "https://github.com/owner/repo";
    let www_url = "https://www.example.com/path";
    let http_url = "http://gitlab.com/user/project";
    let non_com_url = "https://example.org";
    let invalid_url = "invalid-url";
    assert_eq!(extract_host(github_url), Some("github".to_string()));
    assert_eq!(extract_host(www_url), Some("example".to_string()));
    assert_eq!(extract_host(http_url), Some("gitlab".to_string()));
    assert_eq!(extract_host(invalid_url), None);
    assert_eq!(extract_host(non_com_url), None);
}
