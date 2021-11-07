mod cli;
mod commands;
mod config;
mod resource_manager;
mod resources;
mod roblox_api;
mod roblox_auth;
mod state;

#[tokio::main]
async fn main() {
    let result = cli::run().await;

    if let Err(e) = &result {
        println!("\n❌ {}", e);
    }

    std::process::exit(match &result {
        Ok(()) => 0,
        Err(_) => 1,
    });
}
