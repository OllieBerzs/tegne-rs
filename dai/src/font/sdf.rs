// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// generates SDF image from font character

use rusttype::Font;

use super::bitmap::Bitmap;
use super::diamond_iterator::DiamondIterator;
use crate::error::Result;

pub struct Sdf {
    pub sample_size: u32,
    pub sdf_size: u32,
    pub sdf_margin: u16,
}

impl Sdf {
    pub fn new(sample_size: u32, sdf_size: u32, sdf_margin: u16) -> Self {
        Self {
            sample_size,
            sdf_size,
            sdf_margin,
        }
    }

    pub fn generate(&self, font: &Font<'_>, c: char) -> Result<(Bitmap, u32)> {
        // ttf to png
        let sample_margin =
            (f32::from(self.sdf_margin) / self.sdf_size as f32) * self.sample_size as f32;
        let (sample_bitmap, advance) =
            Bitmap::rasterize(font, self.sample_size, sample_margin as u32, c)?;

        // png to sdf
        let bitmap_size = self.sdf_size + u32::from(self.sdf_margin) * 2;
        let mut bitmap = Bitmap::new(bitmap_size, bitmap_size);
        for x in 0..bitmap_size {
            for y in 0..bitmap_size {
                let value = self.distance_to_zone(&sample_bitmap, x, y);
                bitmap.put_pixel(x, y, value);
            }
        }

        Ok((bitmap, self.scale_to_sdf(advance)))
    }

    pub fn scale_to_sdf(&self, value: f32) -> u32 {
        let rescale = self.sdf_size as f32 / self.sample_size as f32;
        (value * rescale).round() as u32
    }

    fn distance_to_zone(&self, sample: &Bitmap, out_x: u32, out_y: u32) -> u8 {
        let threshold = 127;
        let bitmap_size = self.sdf_size + u32::from(self.sdf_margin) * 2;
        let sample_max =
            (f32::from(self.sdf_margin) / self.sdf_size as f32) * self.sample_size as f32;

        let mid_x = (out_x * sample.width()) / bitmap_size;
        let mid_y = (out_y * sample.height()) / bitmap_size;

        let is_inside = sample.get_pixel(mid_x, mid_y) > threshold;

        let mut closest_distance = sample_max;
        for (x, y) in DiamondIterator::new(mid_x as i32, mid_y as i32, sample_max as u16) {
            if x < 0 || y < 0 || x >= sample.width() as i32 || y >= sample.height() as i32 {
                continue;
            }

            let value = sample.get_pixel(x as u32, y as u32);
            if (value >= threshold) == is_inside {
                continue;
            }

            let dx = mid_x as i32 - x;
            let dy = mid_y as i32 - y;
            closest_distance = ((dx * dx + dy * dy) as f32).sqrt();
            break;
        }

        // outside = [0.0, 0.5], inside = [0.5, 1.0]
        let distance = if is_inside {
            0.5 + (closest_distance / 2.0) / sample_max
        } else {
            0.5 - (closest_distance / 2.0) / sample_max
        };

        (distance * 255.0) as u8
    }
}