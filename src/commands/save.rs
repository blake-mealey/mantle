use crate::roblox_api::{upload_place, DeployMode};

pub fn run(project_file: &str, experience_id: &str, place_id: &str) -> Result<(), String> {
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

    upload_place(
        project_file,
        parsed_experience_id,
        parsed_place_id,
        DeployMode::Save,
    )?;

    Ok(())
}
