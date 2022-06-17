use std::env;

use clap::{crate_version, App, Arg};

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let app = App::new("rbx_cookie")
        .version(crate_version!())
        .about("Get the Roblox auth cookie from an authenticated Roblox Studio installation or an environment variable.")
        .arg(
            Arg::with_name("format")
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
        );

    let args: Vec<String> = env::args().collect();
    let matches = app.get_matches_from(args);

    match matches.value_of("format") {
        Some("cookie") => match rbx_cookie::get() {
            Ok(cookie) => print!("{}", cookie),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        Some("value") => match rbx_cookie::get_value() {
            Ok(cookie) => print!("{}", cookie),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        _ => unreachable!(),
    };
}
