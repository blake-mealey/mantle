use std::fs;
use std::path::PathBuf;

pub fn prepare(cargo_manifest_dir: &str) -> PathBuf {
    let mut working_dir = PathBuf::new();
    working_dir.push(&cargo_manifest_dir);
    working_dir.push("..");
    working_dir.push("integration_tmp");
    fs::create_dir_all(&working_dir).unwrap();
    working_dir
}

pub fn cleanup(working_dir: &PathBuf) {
    fs::remove_dir_all(working_dir).unwrap();
}
