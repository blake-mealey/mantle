mod context;
mod files;
mod images;

use pretty_assertions::assert_eq;
use regex::Regex;
use serde::Deserialize;
use serde_yaml::{self, Value};
use std::{collections::HashSet, fs, path::PathBuf};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SpecHeader {
    description: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
enum ExpectStatus {
    Success,
    Failure,
}
impl Default for ExpectStatus {
    fn default() -> Self {
        ExpectStatus::Success
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Expectations {
    #[serde(default)]
    status: ExpectStatus,
    #[serde(default = "Vec::new")]
    created_assets: Vec<String>,
    #[serde(default = "Vec::new")]
    updated_assets: Vec<String>,
    #[serde(default = "Vec::new")]
    deleted_assets: Vec<String>,
}
impl Default for Expectations {
    fn default() -> Self {
        Self {
            status: ExpectStatus::Success,
            created_assets: Vec::new(),
            updated_assets: Vec::new(),
            deleted_assets: Vec::new(),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpecStep {
    config: Option<Value>,
    command: String,
    create_files: Option<Vec<String>>,
    update_files: Option<Vec<String>>,
    delete_files: Option<Vec<String>>,
    #[serde(default)]
    expect: Expectations,
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

    let mut steps: Vec<SpecStep> = docs
        .iter()
        .map(|step| serde_yaml::from_value(step.to_owned()).unwrap())
        .collect();

    let mut context = context::prepare(&cargo_manifest_dir);

    println!("Executing spec: {}", spec_path.display());
    println!("\t{}", header.description);

    for (i, step) in steps.iter_mut().enumerate() {
        println!("\nStep {}", i);

        if let Some(config) = &step.config {
            println!("\tUpdating config");
            files::update_config(&mut context, config);
        }

        if let Some(create_files) = &step.create_files {
            println!("\tCreating files: {:?}", create_files);
            for file in create_files {
                files::create(&mut context, file);
            }
        }

        if let Some(update_files) = &step.update_files {
            println!("\tUpdating files: {:?}", update_files);
            for file in update_files {
                files::update(&mut context, file);
            }
        }

        if let Some(delete_files) = &step.delete_files {
            println!("\tDeleting files: {:?}", delete_files);
            for file in delete_files {
                files::delete(&mut context, file);
            }
        }

        println!("> mantle {}", step.command);
        let output = test_bin::get_test_bin("mantle")
            .args(step.command.split(" "))
            .arg(context.working_dir.to_str().unwrap())
            // .env("RUST_LOG", "trace,html5ever=error")
            .output()
            .unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();

        println!("{}", stdout);
        eprintln!("{}", String::from_utf8(output.stderr).unwrap());

        match step.expect.status {
            ExpectStatus::Success => {
                assert!(output.status.success(), "Status is not success");
            }
            ExpectStatus::Failure => {
                assert!(!output.status.success(), "Status is not failure");
            }
        }

        let actual_created_assets = get_asset_ids(&stdout, "Creating");
        step.expect.created_assets.sort();
        assert_eq!(
            step.expect.created_assets, actual_created_assets,
            "Mismatched created assets"
        );

        let actual_updated_assets = get_asset_ids(&stdout, "Updating");
        step.expect.updated_assets.sort();
        assert_eq!(
            step.expect.updated_assets, actual_updated_assets,
            "Mismatched updated assets"
        );

        let actual_deleted_assets = get_asset_ids(&stdout, "Deleting");
        step.expect.deleted_assets.sort();
        assert_eq!(
            step.expect.deleted_assets, actual_deleted_assets,
            "Mismatched deleted assets"
        );
    }

    // working_dir::cleanup(&context);
}

fn get_asset_ids(output: &str, operation: &str) -> Vec<String> {
    let re = Regex::new(format!("{}: (\\S+)", operation).as_str()).unwrap();
    let mut asset_ids = output
        .split("\n")
        .filter_map(|line| {
            if let Some(captures) = re.captures(line) {
                Some(captures.get(1).unwrap().as_str().to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    asset_ids.sort();
    asset_ids
}
