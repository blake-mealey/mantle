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
        .subcommand(SubCommand::with_name("outputs")
            .about("Prints the outputs from the project to the console or a file")
            .arg(
                Arg::with_name("PROJECT")
                    .index(1)
                    .help("The project to print outputs from: either the path to a directory containing a 'rocat.yml' file or the path to a configuration file. Defaults to the current directory.")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("output")
                    .long("output")
                    .short("o")
                    .help("A file path to print the outputs to")
                    .value_name("FILE")
                    .takes_value(true))
            .arg(
                Arg::with_name("format")
                    .long("format")
                    .short("f")
                    .help("The format to print the outputs in")
                    .value_name("FORMAT")
                    .takes_value(true)
                    .possible_values(&["json","yaml"])
                    .default_value("json"))
        )
}

pub async fn run_with(args: Vec<String>) -> i32 {
    let app = get_app();
    let matches = app.get_matches_from(args);
    match matches.subcommand() {
        ("deploy", Some(deploy_matches)) => {
            commands::deploy::run(deploy_matches.value_of("PROJECT")).await
        }
        ("outputs", Some(outputs_matches)) => {
            commands::outputs::run(
                outputs_matches.value_of("PROJECT"),
                outputs_matches.value_of("output"),
                outputs_matches.value_of("format").unwrap(),
            )
            .await
        }
        _ => unreachable!(),
    }
}

pub async fn run() -> i32 {
    run_with(env::args().collect()).await
}
