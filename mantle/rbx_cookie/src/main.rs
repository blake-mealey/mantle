use std::env;

use clap::{crate_version, App, Arg};
use log::error;

fn main() {
    let app = App::new("rbx_cookie")
        .version(crate_version!())
        .about("Get the Roblox auth cookie from an authenticated Roblox Studio installation or an environment variable.")
        .arg(
            Arg::with_name("format")
                .long("format")
                .short("f")
                .help("The format of the output. Either 'value' (default) or 'cookie'")
                .value_name("FORMAT")
                .takes_value(true)
                .validator(|value| match value.as_str() {
                    "value" => Ok(()),
                    "cookie" => Ok(()),
                    _ => Err("Expected either 'value' or 'cookie'".to_owned()),
                })
                .default_value("value"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("Increase the verbosity of output")
        );

    let args: Vec<String> = env::args().collect();
    let matches = app.get_matches_from(args);

    let log_level = match matches.is_present("verbose") {
        true => "trace",
        false => "info",
    };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    let result = match matches.value_of("format") {
        Some("cookie") => rbx_cookie::get(),
        Some("value") => rbx_cookie::get_value(),
        _ => unreachable!(),
    };

    match result {
        Some(output) => print!("{}", output),
        None => {
            error!("Unable to find ROBLOSECURITY cookie. Login to Roblox Studio or set the ROBLOSECURITY environment variable.");
            std::process::exit(1);
        }
    };
}
