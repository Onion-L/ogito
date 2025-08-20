use clap::{arg, command, Arg, ArgAction, ArgMatches, Command};
use color_eyre::Result;

pub fn build() -> Command {
    let new_command = Command::new("new")
        .about("Create a new project from a template")
        .arg(arg!([source] "The template source (URL or local name)").required(true))
        .arg(arg!(-d --dir <DIRNAME> "The directory name of the new project"))
        .arg(
            arg!(-b --branch [BRANCH] "The branch to clone (if using a URL)")
                .require_equals(true)
                .num_args(0..=1)
                .default_missing_value("INTERACTIVE"),
        )
        .arg(arg!(-m --mode <MODE> "The mode of the operation (if using a URL)").default_value("git"))
        .arg(arg!(-f --force "Force the operation, overwriting existing files").action(ArgAction::SetTrue))
        .arg(
            Arg::new("keep-history")
                .short('H')
                .long("keep-history")
                .help("Keep the history of the repository (if using a URL)")
                .action(ArgAction::SetTrue),
        );

    let clear_command = Command::new("clear")
        .about("Clear the cache")
        .arg(arg!(-f --force "force the operation").action(ArgAction::SetTrue))
        .arg(
            Arg::new("dry-run")
                .short('n')
                .long("dry-run")
                .help("show what would be removed without deleting anything")
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(-v --verbose "show detailed output for each item removed")
                .action(ArgAction::SetTrue),
        );

    let add_command = Command::new("add")
        .about("Add a new template")
        .arg(arg!([url] "the link to the source file").required(true))
        .arg(arg!(-n --name <NAME> "the name of the template"))
        .arg(arg!(-d --description <DESCRIPTION> "the description of the template"))
        .arg(arg!(-a --alias <ALIAS> "the alias of the template"))
        .arg(arg!(-u --update "update the template").action(ArgAction::SetTrue))
        .arg(arg!(-f --force "force the operation").action(ArgAction::SetTrue));

    command!()
        .about("A simple git clone manager")
        .subcommand(new_command)
        .subcommand(clear_command)
        .subcommand(add_command)
        .subcommand_required(true)
        .arg_required_else_help(true)
}

pub async fn dispatch(matches: ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("new", m)) => crate::cmd::new::run(m).await?,
        Some(("clear", m)) => crate::cmd::clear::run(m).await?,
        Some(("add", m)) => crate::cmd::add::run(m).await?,
        _ => {}
    }
    Ok(())
}