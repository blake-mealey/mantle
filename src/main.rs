extern crate cookie;

mod cli;
mod commands;
mod lib;

#[tokio::main]
async fn main() {
    let exit_code = cli::run().await;
    std::process::exit(exit_code);
}
