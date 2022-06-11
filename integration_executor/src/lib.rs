mod context;
mod files;
mod images;

use serde::Deserialize;
use serde_yaml;
use std::{fs, path::PathBuf};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SpecHeader {
    description: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpecState {
    config: Option<rbx_mantle::config::Config>,
    command: String,
    create_files: Option<Vec<String>>,
    update_files: Option<Vec<String>>,
    delete_files: Option<Vec<String>>,
}

pub fn execute_spec(spec: &str) {
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut spec_path = PathBuf::new();
    spec_path.push(&cargo_manifest_dir);
    spec_path.push("..");
    spec_path.push(spec);

    let data = fs::read_to_string(spec_path.clone()).unwrap();

    let mut docs = serde_yaml::Deserializer::from_str(&data)
        .into_iter()
        .map(|document| serde_yaml::Value::deserialize(document).unwrap())
        .collect::<Vec<_>>();

    let header: SpecHeader = serde_yaml::from_value(docs.remove(0)).unwrap();

    let states: Vec<SpecState> = docs
        .iter()
        .map(|state| serde_yaml::from_value(state.to_owned()).unwrap())
        .collect();

    let mut context = context::prepare(&cargo_manifest_dir);

    println!("Executing spec: {}", spec_path.display());
    println!("\t{}", header.description);

    for (i, state) in states.iter().enumerate() {
        println!("\nState {}", i);

        if let Some(_config) = &state.config {
            println!("\tUpdating config");
        }

        if let Some(create_files) = &state.create_files {
            println!("\tCreating files: {:?}", create_files);
            for file in create_files {
                files::create(&mut context, file);
            }
        }

        if let Some(update_files) = &state.update_files {
            println!("\tUpdating files: {:?}", update_files);
            for file in update_files {
                files::update(&mut context, file);
            }
        }

        if let Some(delete_files) = &state.delete_files {
            println!("\tDeleting files: {:?}", delete_files);
            for file in delete_files {
                files::delete(&mut context, file);
            }
        }

        println!("> mantle {}", state.command);
    }

    // working_dir::cleanup(&context);
}
