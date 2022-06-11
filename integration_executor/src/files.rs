use rand::seq::SliceRandom;
use std::fs;
use std::path::PathBuf;

use crate::context::SpecContext;
use crate::images;

pub fn create(context: &mut SpecContext, file: &str) {
    match PathBuf::from(file).extension().and_then(|s| s.to_str()) {
        Some("image") => create_image(context, file),
        Some("audio") => unimplemented!("create audio file"),
        Some("place") => unimplemented!("create place file"),
        _ => println!("create other file"),
    };
}

fn create_image(context: &mut SpecContext, file: &str) {
    let ext = ["bmp", "gif", "jpeg", "jpg", "png", "tga"]
        .choose(&mut context.rng)
        .unwrap();
    println!("create image file: {}", ext);

    context.set_file_extension(file, Some(ext));
    let version = context.get_file_version(file).unwrap();
    let image = images::create(512, 512, &format!("{}", version + 1));

    let path = context.file_path(file);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    image.save(path).unwrap();
}

pub fn update(context: &mut SpecContext, file: &str) {
    match PathBuf::from(file).extension().and_then(|s| s.to_str()) {
        Some("image") => updateImage(context, file),
        Some("audio") => unimplemented!("update audio file"),
        Some("place") => unimplemented!("update place file"),
        _ => println!("create other file"),
    };
}

fn updateImage(context: &mut SpecContext, file: &str) {
    let version = context.increment_file_version(file);
    let image = images::create(512, 512, &format!("{}", version + 1));

    let path = context.file_path(file);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    image.save(path).unwrap();
}

pub fn delete(context: &mut SpecContext, file: &str) {
    let path = context.file_path(file);
    fs::remove_file(path).unwrap();
    context.delete_file(file);
}
