use std::path::Path;

use crate::{
    roblox_api::{DeployMode, RobloxApi},
    roblox_auth::RobloxAuth,
};

pub fn run(place_file: &str, experience_id: &str, place_id: &str) -> Result<(), String> {
    let parsed_experience_id = match experience_id.parse::<u64>() {
        Ok(v) => v,
        Err(e) => return Err(format!("Invalid EXPERIENCE_ID: {}\n\t{}", experience_id, e)),
    };

    let parsed_place_id = match place_id.parse::<u64>() {
        Ok(v) => v,
        Err(e) => return Err(format!("Invalid PLACE_ID: {}\n\t{}", place_id, e)),
    };

    println!("✅ Configuration:");
    println!("\tExperience ID: {}", experience_id);
    println!("\tPlace ID: {}", place_id);

    println!("🚀 Saving place");

    let mut roblox_api = RobloxApi::new(RobloxAuth::new());

    roblox_api.upload_place(
        Path::new(place_file),
        parsed_experience_id,
        parsed_place_id,
        DeployMode::Save,
    )?;

    Ok(())
}
