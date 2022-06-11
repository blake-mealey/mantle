use std::fs;
use std::path::PathBuf;

pub fn create(working_dir: &PathBuf, file: &str) {
    let mut file_path = PathBuf::new();
    file_path.push(working_dir);
    file_path.push(file);

    match file_path.extension().and_then(|s| s.to_str()) {
        Some("image") => create_image(&file_path),
        Some("audio") => unimplemented!("create audio file"),
        Some("place") => unimplemented!("create place file"),
        _ => println!("create other file"),
    };
}

fn create_image(path: &PathBuf) {
    println!("create image");
}

pub fn modify(working_dir: &PathBuf, file: &str) {
    let mut file_path = PathBuf::new();
    file_path.push(working_dir);
    file_path.push(file);

    match file_path.extension().and_then(|s| s.to_str()) {
        Some("image") | Some("bmp") | Some("gif") | Some("jpeg") | Some("jpg") | Some("png")
        | Some("tga") => modify_image(&file_path),
        Some("audio") | Some("ogg") | Some("mp3") => modify_image(&file_path),
        _ => println!("create other file"),
    };
}

fn modify_image(path: &PathBuf) {
    println!("create image");
}

pub fn delete(working_dir: &PathBuf, file: &str) {
    let mut file_path = PathBuf::new();
    file_path.push(working_dir);
    file_path.push(file);

    fs::remove_file(file_path).unwrap();
}
