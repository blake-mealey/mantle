use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use rand::prelude::ThreadRng;

#[derive(Debug)]
pub struct FileMeta {
    pub extension: Option<String>,
    pub version: u32,
}

#[derive(Debug)]
pub struct SpecContext {
    pub working_dir: PathBuf,
    pub file_meta: HashMap<String, FileMeta>,
    pub rng: ThreadRng,
}

impl SpecContext {
    pub fn file_path(&self, file: &str) -> PathBuf {
        let mut file_path = PathBuf::new();
        file_path.push(&self.working_dir);
        file_path.push(file);
        if let Some(Some(ext)) = self.get_file_extension(file) {
            file_path.set_extension(ext);
        }
        file_path
    }

    pub fn set_file_extension(&mut self, file: &str, extension: Option<&str>) {
        if let Some(meta) = self.file_meta.get_mut(file) {
            meta.extension = extension.map(|x| x.to_owned());
        } else {
            self.file_meta.insert(
                file.to_string(),
                FileMeta {
                    extension: extension.map(|x| x.to_owned()),
                    version: 0,
                },
            );
        }
    }

    pub fn get_file_extension(&self, file: &str) -> Option<Option<String>> {
        self.file_meta.get(file).map(|meta| meta.extension.clone())
    }

    pub fn set_file_version(&mut self, file: &str, version: u32) {
        if let Some(meta) = self.file_meta.get_mut(file) {
            meta.version = version;
        } else {
            self.file_meta.insert(
                file.to_string(),
                FileMeta {
                    extension: None,
                    version,
                },
            );
        }
    }

    pub fn get_file_version(&self, file: &str) -> Option<u32> {
        self.file_meta.get(file).map(|meta| meta.version)
    }

    pub fn increment_file_version(&mut self, file: &str) -> u32 {
        if let Some(meta) = self.file_meta.get_mut(file) {
            meta.version += 1;
            meta.version
        } else {
            self.file_meta.insert(
                file.to_string(),
                FileMeta {
                    extension: None,
                    version: 0,
                },
            );
            0
        }
    }

    pub fn delete_file(&mut self, file: &str) {
        self.file_meta.remove(file);
    }
}

pub fn prepare(cargo_manifest_dir: &str) -> SpecContext {
    let mut working_dir = PathBuf::new();
    working_dir.push(&cargo_manifest_dir);
    working_dir.push("..");
    working_dir.push("integration_tmp");
    fs::create_dir_all(&working_dir).unwrap();

    SpecContext {
        working_dir,
        file_meta: HashMap::new(),
        rng: rand::thread_rng(),
    }
}

pub fn cleanup(spec_context: &SpecContext) {
    fs::remove_dir_all(&spec_context.working_dir).unwrap();
}
