use load_image::ImageData;

use super::color::Color;

pub trait ImageT {
    fn get_pixel_data(&self, x: usize, y: usize) -> Color;
}

impl ImageT for load_image::Image {
    fn get_pixel_data(&self, x: usize, y: usize) -> Color {
        match &self.bitmap {
            ImageData::RGB8(img) => {
                let pixel = img[x + self.width * y];
                Color::new(
                    pixel.r as f64 / u8::MAX as f64,
                    pixel.g as f64 / u8::MAX as f64,
                    pixel.b as f64 / u8::MAX as f64,
                )
            }
            ImageData::RGBA8(img) => {
                let pixel = img[x + self.width * y];
                Color::new(
                    pixel.r as f64 / u8::MAX as f64,
                    pixel.g as f64 / u8::MAX as f64,
                    pixel.b as f64 / u8::MAX as f64,
                )
            }
            ImageData::RGB16(img) => {
                let pixel = img[x + self.width * y];
                Color::new(
                    pixel.r as f64 / u16::MAX as f64,
                    pixel.g as f64 / u16::MAX as f64,
                    pixel.b as f64 / u16::MAX as f64,
                )
            }
            ImageData::RGBA16(img) => {
                let pixel = img[x + self.width * y];
                Color::new(
                    pixel.r as f64 / u16::MAX as f64,
                    pixel.g as f64 / u16::MAX as f64,
                    pixel.b as f64 / u16::MAX as f64,
                )
            }
            ImageData::GRAY8(img) => {
                let pixel = img[x + self.width * y];
                Color::new(
                    pixel.0 as f64 / u8::MAX as f64,
                    pixel.0 as f64 / u8::MAX as f64,
                    pixel.0 as f64 / u8::MAX as f64,
                )
            }
            ImageData::GRAY16(img) => {
                let pixel = img[x + self.width * y];
                Color::new(
                    pixel.0 as f64 / u16::MAX as f64,
                    pixel.0 as f64 / u16::MAX as f64,
                    pixel.0 as f64 / u16::MAX as f64,
                )
            }
            ImageData::GRAYA8(img) => {
                let pixel = img[x + self.width * y];
                Color::new(
                    pixel.0 as f64 / u8::MAX as f64,
                    pixel.0 as f64 / u8::MAX as f64,
                    pixel.0 as f64 / u8::MAX as f64,
                )
            }
            ImageData::GRAYA16(img) => {
                let pixel = img[x + self.width * y];
                Color::new(
                    pixel.0 as f64 / u16::MAX as f64,
                    pixel.0 as f64 / u16::MAX as f64,
                    pixel.0 as f64 / u16::MAX as f64,
                )
            }
        }
    }
}

#[macro_export]
macro_rules! load_image {
    ($filename:expr) => {
        load_image::load_path($filename).expect(&format!("Failed to load image: '{}'!", $filename))
    };
}
