use std::ops::Mul;
use std::path::Path;

use image::io::Reader;
use image::{DynamicImage, GenericImageView, Pixel};
use ratatui::style::Color;
use ratatui::text::{Line, Span, Text};

pub struct AsciiOptions {
    width: u32,
    height: u32,
    gamma: f32,
}

impl AsciiOptions {
    pub fn new(width: u32, height: u32, gamma: f32) -> Self {
        Self {
            width,
            height,
            gamma,
        }
    }
}

impl Default for AsciiOptions {
    fn default() -> Self {
        Self::new(80, 80, 1.0)
    }
}

pub trait ToAsciiArt {
    fn to_ascii_art(&self, options: Option<AsciiOptions>) -> Text;
}

pub struct ImageConverter {
    image: DynamicImage,
}

impl ImageConverter {
    pub fn new(image: DynamicImage) -> Self {
        Self { image }
    }
}

impl<P> From<P> for ImageConverter
where
    P: AsRef<Path>,
{
    fn from(value: P) -> Self {
        let open_file = Reader::open(value).unwrap();
        let image = open_file.decode().unwrap();
        Self::new(image)
    }
}

impl ToAsciiArt for ImageConverter {
    fn to_ascii_art(&self, options: Option<AsciiOptions>) -> Text {
        let options = options.unwrap_or_default();

        let target_width = options.width;
        let target_height = options.height;
        let gamma = options.gamma;

        let width_ratio = self.image.width() as f32 / target_width as f32;
        let height_ratio = self.image.height() as f32 / target_height as f32;

        let mut lines = vec![];
        for y in 0..target_height {
            let mut line = vec![];
            for x in 0..target_width {
                let start_x = (x as f32 * width_ratio) as u32;
                let start_y = (y as f32 * height_ratio) as u32;

                let mut total_r = 0;
                let mut total_g = 0;
                let mut total_b = 0;
                let mut total_a = 0;

                for dy in 0..height_ratio as u32 {
                    for dx in 0..width_ratio as u32 {
                        let pixel = self.image.get_pixel(start_x + dx, start_y + dy);
                        let channels = pixel.channels();
                        total_r += channels[0] as u32;
                        total_g += channels[1] as u32;
                        total_b += channels[2] as u32;
                        total_a += channels[3] as u32;
                    }
                }

                let count = (width_ratio * height_ratio) as u32;
                let avg_r = (total_r / count) as u8;
                let avg_g = (total_g / count) as u8;
                let avg_b = (total_b / count) as u8;
                let avg_a = (total_a / count) as u8;

                let base_luminance =
                    (0.2126 * avg_r as f32 + 0.7152 * avg_g as f32 + 0.0722 * avg_b as f32) as u8;
                let luminance = ((base_luminance as f32 / 255.0).powf(gamma) * 255.0)
                    .mul(255.0 - avg_a as f32) as u8;

                let color = Color::Rgb(avg_r, avg_g, avg_b);

                let character = match luminance {
                    0..=31 => '#',
                    32..=63 => '@',
                    64..=95 => '8',
                    96..=127 => '&',
                    128..=159 => 'o',
                    160..=191 => ':',
                    192..=223 => '*',
                    224..=250 => '.',
                    251..=255 => ' ',
                };

                line.push(Span::from(character.to_string()).style(color));
            }
            lines.push(Line::from(line));
        }

        Text::from(lines)
    }
}
