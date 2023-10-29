use std::{env, fmt::Display};

use clap::{crate_version, App, Arg};
use log::error;
use rbx_auth::WithRobloxAuth;
use reqwest::StatusCode;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let app = App::new("rbx_auth")
        .version(crate_version!())
        .about("Get the authenticated user from an authenticated Roblox Studio installation or an environment variable.")
         .arg(
            Arg::with_name("format")
                .long("format")
                .short("f")
                .help("The format of the output.")
                .value_name("FORMAT")
                .takes_value(true)
                .possible_values(&["table", "json"])
                .default_value("table"),
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

    if let Err(err) = run(matches.value_of("format")).await {
        error!("{}", err.to_string());
        std::process::exit(1);
    };
}

async fn run(format: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let auth = rbx_auth::RobloxAuth::new().await?;

    let client = reqwest::Client::builder()
        .user_agent("Roblox/WinInet")
        .roblox_auth(auth)
        .build()?;

    let res = client
        .get("https://users.roblox.com/v1/users/authenticated")
        .send()
        .await?;

    match res.status() {
        StatusCode::OK => {
            let model = res.json::<Value>().await?;

            match format {
                Some("table") => {
                    println!(
                        r#"id            {}
username      {}
display name  {}
"#,
                        &model.get("id").unwrap().as_u64().unwrap(),
                        &model.get("name").unwrap().as_str().unwrap(),
                        &model.get("displayName").unwrap().as_str().unwrap()
                    )
                }
                Some("json") => {
                    println!("{}", model);
                }
                _ => unreachable!(),
            };
        }
        _ => {
            return Err(Box::new(RobloxError::new()));
        }
    };

    Ok(())
}

#[derive(Debug)]
struct RobloxError {}

impl RobloxError {
    fn new() -> Self {
        RobloxError {}
    }
}

impl Display for RobloxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to retrieve authenticated user details.")
    }
}

impl std::error::Error for RobloxError {}
