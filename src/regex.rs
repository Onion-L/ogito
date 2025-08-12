use color_eyre::{eyre::eyre, Result};
use regex::Regex;

pub fn is_valid_url(input: &str) -> Result<bool> {
    let re = Regex::new(r"^(?:https://)?(github|gitlab)\.com/([^/]+)/([^/]+?)(?:\.git)?$")
        .map_err(|e| eyre!("Failed to create regex: {}", e))?;
    Ok(re.is_match(input))
}

pub fn extract_path(url: &str) -> Option<(&str, &str)> {
    let re = Regex::new(r"https?://[^/]+/(.*)").ok()?;
    let path = re.captures(url)?.get(1).map(|m| m.as_str())?;
    let (owner, repo) = (
        path.split("/").next()?,
        path.split("/").last()?.split(".").next()?,
    );
    Some((owner, repo))
}

pub fn extract_host(url: &str) -> Option<String> {
    let re = Regex::new(r"https?://(?:www\.)?([^.]+)\.com").ok()?;
    re.captures(url)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}

#[cfg(test)]
mod tests {
    use super::{extract_host, extract_path, is_valid_url};

    #[test]
    fn test_is_github_url() {
        assert!(is_valid_url("https://github.com/owner/repo").unwrap());
        assert!(is_valid_url("github.com/owner/repo").unwrap());
        assert!(is_valid_url("https://github.com/owner/repo.git").unwrap());
        assert!(is_valid_url("https://gitlab.com/owner/repo").unwrap());
        assert!(!is_valid_url("random string").unwrap());
        assert!(!is_valid_url("https://example.com/owner/repo").unwrap());
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
}
