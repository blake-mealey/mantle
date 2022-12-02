use crate::commands;
use clap::{crate_version, App, AppSettings, Arg, SubCommand};
use std::env;

fn get_app() -> App<'static, 'static> {
    App::new("Mantle")
        .version(crate_version!())
        .about("Infra-as-code and deployment tool for Roblox")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("deploy")
                .about("Updates a Mantle environment with a project's latest configuration.")
                .arg(
                    Arg::with_name("PROJECT")
                        .index(1)
                        .help("The Mantle project: either the path to a directory containing a 'mantle.yml' file or the path to a configuration file. Defaults to the current directory.")
                        .takes_value(true))
                .arg(
                    Arg::with_name("environment")
                        .long("environment")
                        .short("e")
                        .help("The label of the environment to deploy to. If not specified, attempts to match the current git branch to each environment's `branches` property.")
                        .value_name("ENVIRONMENT")
                        .takes_value(true))
                .arg(
                    Arg::with_name("allow_purchases")
                        .long("allow-purchases")
                        .help("Gives Mantle permission to make purchases with Robux."))
        )
        .subcommand(
            SubCommand::with_name("destroy")
                .about("Destroys a Mantle environment.")
                .arg(
                    Arg::with_name("PROJECT")
                        .index(1)
                        .help("The Mantle project: either the path to a directory containing a 'mantle.yml' file or the path to a configuration file. Defaults to the current directory.")
                        .takes_value(true))
                .arg(
                    Arg::with_name("environment")
                        .long("environment")
                        .short("e")
                        .help("The label of the environment to destroy. If not specified, attempts to match the current git branch to each environment's `branches` property.")
                        .value_name("ENVIRONMENT")
                        .takes_value(true))
        )
        .subcommand(
            SubCommand::with_name("outputs")
                .about("Prints a Mantle environment's outputs to the console or a file in a machine-readable format.")
                .arg(
                    Arg::with_name("PROJECT")
                        .index(1)
                        .help("The Mantle project: either the path to a directory containing a 'mantle.yml' file or the path to a configuration file. Defaults to the current directory.")
                        .takes_value(true))
                .arg(
                    Arg::with_name("environment")
                        .long("environment")
                        .short("e")
                        .help("The label of the environment to print the outputs of. If not specified, attempts to match the current git branch to each environment's `branches` property.")
                        .value_name("ENVIRONMENT")
                        .takes_value(true))
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
        .subcommand(
            SubCommand::with_name("import")
                .about("Imports an existing target into a Mantle environment.")
                .arg(
                    Arg::with_name("PROJECT")
                        .index(1)
                        .help("The Mantle project: either the path to a directory containing a 'mantle.yml' file or the path to a configuration file. Defaults to the current directory.")
                        .takes_value(true))
                .arg(
                    Arg::with_name("environment")
                        .long("environment")
                        .short("e")
                        .help("The label of the environment to print the outputs of. If not specified, attempts to match the current git branch to each environment's `branches` property.")
                        .value_name("ENVIRONMENT")
                        .takes_value(true))
                .arg(
                    Arg::with_name("target_id")
                        .long("target-id")
                        .help("The ID of the target to import.")
                        .value_name("ID")
                        .takes_value(true)
                        .required(true))
        )
        .subcommand(
            SubCommand::with_name("state")
                .about("Manage state files.")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("download")
                        .about("Download the remote state file for a project. Remote state must be configured for the Mantle project.")
                        .arg(
                            Arg::with_name("PROJECT")
                                .index(1)
                                .help("The Mantle project: either the path to a directory containing a 'mantle.yml' file or the path to a configuration file. Defaults to the current directory.")
                                .takes_value(true))
                        .arg(
                            Arg::with_name("key")
                                .long("key")
                                .help("A key to prefix the name of the state file (e.g. `--key custom` will result in `custom.mantle-state.yml`).")
                                .value_name("KEY")
                                .takes_value(true))
                )
                .subcommand(
                    SubCommand::with_name("upload")
                        .about("Upload a state file to a remote provider for a project. Remote state must be configured for the Mantle project.")
                        .arg(
                            Arg::with_name("PROJECT")
                                .index(1)
                                .help("The Mantle project: either the path to a directory containing a 'mantle.yml' file or the path to a configuration file. Defaults to the current directory.")
                                .takes_value(true))
                        .arg(
                            Arg::with_name("key")
                                .long("key")
                                .help("The prefix of the name of the state file (e.g. `--key custom` will load from `custom.mantle-state.yml`).")
                                .value_name("KEY")
                                .takes_value(true))
                )
        )
}

pub async fn run_with(args: Vec<String>) -> i32 {
    let app = get_app();
    let matches = app.get_matches_from(args);
    match matches.subcommand() {
        ("deploy", Some(deploy_matches)) => {
            commands::deploy::run(
                deploy_matches.value_of("PROJECT"),
                deploy_matches.value_of("environment"),
                deploy_matches.is_present("allow_purchases"),
            )
            .await
        }
        ("destroy", Some(destroy_matches)) => {
            commands::destroy::run(
                destroy_matches.value_of("PROJECT"),
                destroy_matches.value_of("environment"),
            )
            .await
        }
        ("outputs", Some(outputs_matches)) => {
            commands::outputs::run(
                outputs_matches.value_of("PROJECT"),
                outputs_matches.value_of("environment"),
                outputs_matches.value_of("output"),
                outputs_matches.value_of("format").unwrap(),
            )
            .await
        }
        ("import", Some(import_matches)) => {
            commands::import::run(
                import_matches.value_of("PROJECT"),
                import_matches.value_of("environment"),
                import_matches.value_of("target_id").unwrap(),
            )
            .await
        }
        ("state", Some(state_matches)) => match state_matches.subcommand() {
            ("download", Some(download_matches)) => {
                commands::download::run(
                    download_matches.value_of("PROJECT"),
                    download_matches.value_of("key"),
                )
                .await
            }
            ("upload", Some(upload_matches)) => {
                commands::upload::run(
                    upload_matches.value_of("PROJECT"),
                    upload_matches.value_of("key"),
                )
                .await
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

pub async fn run() -> i32 {
    run_with(env::args().collect()).await
}
