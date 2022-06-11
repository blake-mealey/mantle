use image::{Rgb, RgbImage};
use imageproc::{
    drawing::{draw_filled_rect_mut, draw_text_mut, text_size},
    rect::Rect,
};
use rusttype::{Font, Scale};

pub fn create(width: u32, height: u32, center_text: &str) -> RgbImage {
    let mut image = RgbImage::new(width, height);

    let font = Vec::from(include_bytes!("Roboto-Regular.ttf") as &[u8]);
    let font = Font::try_from_vec(font).expect("font");

    // White background
    draw_filled_rect_mut(
        &mut image,
        Rect::at(0, 0).of_size(width, height),
        Rgb([255u8, 255u8, 255u8]),
    );

    let width_scale_factor = width as f32 / 200.0;
    let height_scale_factor = height as f32 / 200.0;
    let scale_factor = if width_scale_factor < height_scale_factor {
        width_scale_factor
    } else {
        height_scale_factor
    };

    // Bottom watermark
    let text = "Mantle Integration Test";
    let scale = Scale::uniform(18.0 * scale_factor);
    let (w, h) = text_size(scale, &font, text);
    draw_text_mut(
        &mut image,
        Rgb([50u8, 50u8, 50u8]),
        width as i32 / 2 - w / 2,
        height as i32 - h - 5,
        scale,
        &font,
        text,
    );

    // Center text
    let text = center_text;
    let scale = Scale::uniform(128.0 * scale_factor);
    let (w, h) = text_size(scale, &font, text);
    draw_text_mut(
        &mut image,
        Rgb([0u8, 0u8, 0u8]),
        width as i32 / 2 - w / 2,
        height as i32 / 2 - h / 2 - 10,
        scale,
        &font,
        text,
    );

    image
}
