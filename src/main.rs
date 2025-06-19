use clap::{ArgAction, arg, command};
use futures::executor::block_on;
use regex::Regex;
use std::path::Path;
use tokio::fs;
use tokio::process::Command;

fn main() {
    let matches = command!()
        .about("A simple git clone manager")
        .arg(arg!([src] "the link to the source file"))
        .arg(arg!(-r --repo <REPO> "the repository name, e.g. 'user/repo'").required(false))
        .arg(arg!(-d --dir <DIRNAME> "the directory name").required(true))
        .arg(
            arg!( -s --site <SITE> "Sets the site or use Github by default")
                .default_value("github"),
        )
        .arg(arg!(-f --force "force the operation").action(ArgAction::SetTrue))
        .get_matches();

    let dir = matches
        .get_one::<String>("dir")
        .expect("Directory name is required");

    // get the value of the src argument
    if let Some(src) = matches.get_one::<String>("src") {
        if is_github_url(&src.to_string()) {
            block_on(_run_git_clone(&src.to_string(), &dir));
        } else {
            panic!("The source is not a Github URL");
        }
    }

    // get the value of the repo argument
    if let Some(repo) = matches.get_one::<String>("repo") {
        println!("Value for repo: {}", repo.to_string());
    }

    // get the value of the site argument
    if let Some(site) = matches.get_one::<String>("site") {
        println!("Value for site: {}", site);
    }
}

fn is_github_url(input: &str) -> bool {
    let re = Regex::new(r"^(?:https://)?github\.com/([^/]+)/([^/]+?)(?:\.git)?$").unwrap();
    re.is_match(input)
}

async fn _run_git_clone(url: &str, dir: &str) {
    println!("Cloning {} into {}", url, dir);
    let clone_status = Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(dir)
        .status()
        .await
        .unwrap();

    if !clone_status.success() {
        panic!("Failed to execute git clone");
    }

    let git_dir = Path::new(dir).join(".git");
    match fs::remove_dir_all(git_dir).await {
        Ok(_) => println!("Removed .git directory"),
        Err(e) => panic!("Failed to remove .git directory: {}", e),
    }
}
