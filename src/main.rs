extern crate cookie;
extern crate log;

mod cli;
mod commands;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("off")).init();

    let exit_code = cli::run().await;
    std::process::exit(exit_code);
}
