extern crate clap;
use crate::commands;
use clap::{crate_version, App, AppSettings, Arg, SubCommand};
use std::env;

fn get_app() -> App<'static, 'static> {
    App::new("Rocat")
        .version(crate_version!())
        .about("Manages Roblox deployments")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("deploy")
                .about("Saves a project file to a Roblox place")
                .arg(
                    Arg::with_name("PROJECT")
                        .index(1)
                        .help("The project to deploy: either the path to a directory containing a 'rocat.yml' file or the path to a configuration file. Defaults to the current directory.")
                        .takes_value(true),
                ),
        )
}

pub fn run_with(args: Vec<String>) -> Result<(), String> {
    let app = get_app();
    let matches = app.get_matches_from(args);
    match matches.subcommand() {
        ("deploy", Some(deploy_matches)) => {
            commands::deploy::run(deploy_matches.value_of("PROJECT"))
        }
        _ => Err("Unreachable code reached!".to_string()),
    }
}

pub fn run() -> Result<(), String> {
    run_with(env::args().collect())
}
