use clap::{ArgAction, arg, command};
use regit::regit;

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

    let src = matches.get_one::<String>("src").unwrap();
    // get the value of the src argument
    match regit(&src.to_string(), &dir) {
        Ok(_) => println!("Successfully cloned the repository"),
        Err(e) => panic!("Error: {}", e),
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
