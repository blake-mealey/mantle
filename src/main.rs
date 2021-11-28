extern crate cookie;

mod cli;
mod commands;
mod config;
mod logger;
mod project;
mod resource_graph;
mod resource_reconciler;
mod roblox_api;
mod roblox_auth;
mod roblox_resource_manager;
mod state;
mod util;

#[tokio::main]
async fn main() {
    let exit_code = cli::run().await;
    std::process::exit(exit_code);
}
