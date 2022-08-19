use rbx_auth::RobloxAuth;

#[tokio::main]
async fn main() {
    match RobloxAuth::new().await {
        Ok(auth) => {
            println!("{:?}", auth);
            println!("\nSuccessfully authenticated!");
        }
        Err(e) => {
            println!("Failed to authenticate: {}", e);
        }
    }
}
