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
            SubCommand::with_name("save")
                .about("Saves a project file to a Roblox place")
                .arg(
                    Arg::with_name("FILE")
                        .required(true)
                        .index(1)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("EXPERIENCE_ID")
                        .required(true)
                        .index(2)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("PLACE_ID")
                        .required(true)
                        .index(3)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("publish")
                .about("Publishes a project file to a Roblox place")
                .arg(
                    Arg::with_name("FILE")
                        .required(true)
                        .index(1)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("EXPERIENCE_ID")
                        .required(true)
                        .index(2)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("PLACE_ID")
                        .required(true)
                        .index(3)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("deploy")
                .about("Saves a project file to a Roblox place")
                .arg(
                    Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("FILE")
                        .help("Sets a custom deploy config file")
                        .default_value("rocat.yml")
                        .takes_value(true),
                ),
        )
}

pub fn run_with(args: Vec<String>) -> Result<(), String> {
    let app = get_app();
    let matches = app.get_matches_from(args);
    match matches.subcommand() {
        ("save", Some(save_matches)) => commands::save::run(
            save_matches.value_of("FILE").unwrap(),
            save_matches.value_of("EXPERIENCE_ID").unwrap(),
            save_matches.value_of("PLACE_ID").unwrap(),
        ),
        ("publish", Some(publish_matches)) => commands::publish::run(
            publish_matches.value_of("FILE").unwrap(),
            publish_matches.value_of("EXPERIENCE_ID").unwrap(),
            publish_matches.value_of("PLACE_ID").unwrap(),
        ),
        ("deploy", Some(deploy_matches)) => {
            commands::deploy::run(deploy_matches.value_of("config").unwrap())
        }
        _ => Err("Unreachable code reached!".to_string()),
    }
}

pub fn run() -> Result<(), String> {
    run_with(env::args().collect())
}
