use regex::Regex;

pub fn is_github_url(input: &str) -> bool {
    let re = Regex::new(r"^(?:https://)?github\.com/([^/]+)/([^/]+?)(?:\.git)?$").unwrap();
    re.is_match(input)
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
    let re = Regex::new(r"https?://(?:www\.)?([^.]+)\.com").unwrap();
    re.captures(url)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}
