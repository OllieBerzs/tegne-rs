// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// imports ttf font for use in draw-it

use rusttype::Font;
use serde::Serialize;
use std::collections::HashMap;

use super::bitmap::Bitmap;
use super::sdf::Sdf;
use crate::error::ErrorKind;
use crate::error::ErrorType;
use crate::error::Result;

#[derive(Serialize)]
struct FontFile {
    bitmap_fonts: Vec<BitmapFont>,
    sdf_font: SdfFont,
}

#[derive(Serialize)]
struct BitmapFont {
    bitmap_size: u32,
    font_size: u32,
    char_metrics: HashMap<char, CharMetrics>,
    bitmap: Vec<u8>,
}

#[derive(Serialize)]
struct SdfFont {
    bitmap_size: u32,
    font_size: u32,
    margin: u32,
    char_metrics: HashMap<char, CharMetrics>,
    bitmap: Vec<u8>,
}

#[derive(Serialize)]
struct CharMetrics {
    pub x: u32,
    pub y: u32,
    pub advance: u32,
}

pub struct FontOptions<'sizes> {
    pub sdf_sample: u32,
    pub sdf_size: u32,
    pub sdf_margin: u16,
    pub bitmap_sizes: &'sizes [u32],
}

pub fn import_font(data: &[u8], options: FontOptions<'_>) -> Result<Vec<u8>> {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,?!:-_+=@#(){}[]/";
    let tile_count = (chars.len() as f32).sqrt().ceil() as u32;

    let font = Font::try_from_bytes(data).ok_or(ErrorType::Internal(ErrorKind::InvalidFont))?;

    // create sdf font
    let sdf = Sdf::new(options.sdf_sample, options.sdf_size, options.sdf_margin);
    let sdf_tile_size = sdf.sdf_size + u32::from(sdf.sdf_margin) * 2;
    let sdf_bitmap_size = tile_count * sdf_tile_size;

    let mut sdf_font = SdfFont {
        bitmap_size: sdf_bitmap_size,
        font_size: sdf.sdf_size,
        margin: u32::from(sdf.sdf_margin),
        char_metrics: HashMap::new(),
        bitmap: vec![],
    };

    let mut sdf_bitmap = Bitmap::new(sdf_bitmap_size, sdf_bitmap_size);

    for (i, c) in chars.chars().enumerate() {
        let (bitmap, advance) = sdf.generate(&font, c)?;

        let x = (i as u32 % tile_count) * sdf_tile_size;
        let y = (i as u32 / tile_count) * sdf_tile_size;

        sdf_font
            .char_metrics
            .insert(c, CharMetrics { x, y, advance });

        sdf_bitmap.copy_from(&bitmap, x, y);
    }

    sdf_font.bitmap = sdf_bitmap.into_buffer();

    // create bitmap fonts
    let mut bitmap_fonts = Vec::with_capacity(options.bitmap_sizes.len());
    for font_size in options.bitmap_sizes {
        let bitmap_size = tile_count * font_size;
        let mut bitmap = Bitmap::new(bitmap_size, bitmap_size);
        let mut char_metrics = HashMap::new();

        for (i, c) in chars.chars().enumerate() {
            // ttf to png
            let (char_bitmap, advance) = Bitmap::rasterize(&font, *font_size, 0, c)?;

            let x = (i as u32 % tile_count) * *font_size;
            let y = (i as u32 / tile_count) * *font_size;

            char_metrics.insert(
                c,
                CharMetrics {
                    x,
                    y,
                    advance: advance as u32,
                },
            );

            bitmap.copy_from(&char_bitmap, x, y);
        }

        bitmap_fonts.push(BitmapFont {
            font_size: *font_size,
            bitmap_size,
            char_metrics,
            bitmap: bitmap.into_buffer(),
        });
    }

    // write fonts to file
    let data = FontFile {
        bitmap_fonts,
        sdf_font,
    };

    let binary = bincode::serialize(&data)?;
    Ok(binary)
}