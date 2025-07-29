use color_eyre::{Result, eyre::eyre};
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
