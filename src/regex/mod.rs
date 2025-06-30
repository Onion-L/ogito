use regex::Regex;

pub fn is_github_url(input: &str) -> bool {
    let re = Regex::new(r"^(?:https://)?github\.com/([^/]+)/([^/]+?)(?:\.git)?$").unwrap();
    re.is_match(input)
}

pub fn extract_path(url: &str) -> Option<(&str, &str)> {
    let re = Regex::new(r"https?://[^/]+/(.*)").ok()?;
    let path = re.captures(url)?.get(1).map(|m| m.as_str())?;
    let (owner, repo) = (path.split("/").next()?, path.split("/").last()?);
    Some((owner, repo))
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
