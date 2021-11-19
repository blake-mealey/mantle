mod cli;
mod commands;
mod config;
mod logger;
mod project;
mod roblox_api;
mod roblox_auth;
mod safe_resource_manager;
mod safe_resources;
mod state;
mod util;

#[tokio::main]
async fn main() {
    let exit_code = cli::run().await;
    std::process::exit(exit_code);
}
