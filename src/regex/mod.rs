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
