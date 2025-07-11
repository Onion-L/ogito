use ogito::regex::{extract_path, is_github_url};

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
    assert_eq!(result, Some(("owner", "repo.git")));

    let invalid = "https://gitlab.com/owner/repo";
    assert!(extract_path(invalid).is_some());
}
