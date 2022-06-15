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
                .help("The format of the output. Either 'cookie' (default) or 'value'")
                .value_name("FORMAT")
                .takes_value(true)
                .validator(|value| match value.as_str() {
                    "cookie" => Ok(()),
                    "value" => Ok(()),
                    _ => Err(format!("Expected either 'cookie' or 'value'").into()),
                })
                .default_value("cookie"),
        );

    let args: Vec<String> = env::args().collect();
    let matches = app.get_matches_from(args);

    match matches.value_of("format") {
        Some("cookie") => match rbx_cookie::get() {
            Ok(cookie) => println!("{}", cookie),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        Some("value") => match rbx_cookie::get_value() {
            Ok(cookie) => println!("{}", cookie),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        _ => unreachable!(),
    };
}
