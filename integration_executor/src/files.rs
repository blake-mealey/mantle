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
    context.set_file_extension(file, Some(ext));

    let (width, height) = get_image_dimensions(context, file);
    let version = context.get_file_version(file).unwrap();
    let image = images::create(width, height, &format!("{}", version + 1));

    let path = context.file_path(file);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    image.save(path).unwrap();
}

pub fn update(context: &mut SpecContext, file: &str) {
    match PathBuf::from(file).extension().and_then(|s| s.to_str()) {
        Some("image") => update_image(context, file),
        Some("audio") => unimplemented!("update audio file"),
        Some("place") => unimplemented!("update place file"),
        _ => println!("create other file"),
    };
}

fn update_image(context: &mut SpecContext, file: &str) {
    let (width, height) = get_image_dimensions(context, file);
    let version = context.increment_file_version(file);
    let image = images::create(width, height, &format!("{}", version + 1));

    let path = context.file_path(file);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    image.save(path).unwrap();
}

fn get_image_dimensions(context: &SpecContext, file: &str) -> (u32, u32) {
    let ext = context.get_file_extension(file).unwrap().unwrap();
    let (mut width, mut height) = (200, 200);
    let path = context.file_path(file);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    if file_name.ends_with(&format!(".thumbnail.{}", ext)) {
        width = 1920;
        height = 1080;
    } else if file_name.ends_with(&format!(".icon.{}", ext)) {
        width = 512;
        height = 512;
    }
    (width, height)
}

pub fn delete(context: &mut SpecContext, file: &str) {
    let path = context.file_path(file);
    fs::remove_file(path).unwrap();
    context.delete_file(file);
}
