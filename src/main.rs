mod cli;
mod commands;
mod roblox_api;

fn main() {
    std::process::exit(match cli::run() {
        Ok(()) => 0,
        Err(e) => {
            println!("\n❌ {}", e);
            1
        }
    });
}
