use std::fs;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    file: Option<String>,
    branches: Option<HashMap<String, BranchConfig>>
}

#[derive(Deserialize)]
enum BranchMode {
    Publish,
    Save
}

#[derive(Deserialize)]
struct BranchConfig {
    experience_id: Option<u64>,
    place_id: Option<u64>,
    mode: Option<BranchMode>
}

fn main() {
    let data = fs::read_to_string("rocat.toml").expect("Unable to read file.");
    let config: Config = toml::from_str(&data).expect("Unable to parse file.");

    if config.file.is_some() {
        println!("file {}", config.file.unwrap());
    }

    if config.branches.is_some() {
        println!("branches");

        for (key, value) in config.branches.unwrap() {
            println!("{:?}", value.experience_id.unwrap());
            println!("{:?}", value.place_id.unwrap());
            if value.mode.is_some() {
                match value.mode.unwrap() {
                    BranchMode::Publish => println!("publish"),
                    BranchMode::Save => println!("save"),
                    _ => println!("none")
                }
            }
        }
    }
}
