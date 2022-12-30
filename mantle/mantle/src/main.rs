use dotenv::dotenv;
use log::info;

mod cli;
mod commands;

#[tokio::main]
async fn main() {
    let dotenv_path = dotenv().ok();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("off")).init();

    if let Some(path) = dotenv_path {
        info!("Loaded variables from dotenv file: {}", path.display());
    }

    let exit_code = cli::run().await;
    std::process::exit(exit_code);
}
