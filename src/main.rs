mod cli;
mod commands;
mod resource_manager;
mod resources;
mod roblox_api;
mod roblox_auth;
mod state;

fn main() {
    let result = cli::run();

    if let Err(e) = &result {
        println!("\n❌ {}", e);
    }

    std::process::exit(match &result {
        Ok(()) => 0,
        Err(_) => 1,
    });
}
