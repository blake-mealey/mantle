mod cli;
mod commands;
mod roblox_api;

fn main() {
    std::process::exit(match cli::run() {
        Ok(v) => {
            println!("{}", v);
            0
        }
        Err(e) => {
            println!("{}", e);
            1
        }
    });
}
