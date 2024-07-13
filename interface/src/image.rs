use std::path::Path;

use image::io::Reader;
use image::{DynamicImage, GenericImageView, Pixel};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};

#[derive(Default)]
pub struct AsciiOptions {
    pub width: u32,
    pub height: u32,
    pub gamma: f32,
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
                let mut subpixels = [Color::Black, Color::Black];
                let mut total_channels = [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]];

                let start_x = (x as f32 * width_ratio) as u32;
                let start_y = (y as f32 * height_ratio) as u32;
                let count = width_ratio * height_ratio / 2.0;

                for dy in 0..height_ratio as u32 {
                    for dx in 0..width_ratio as u32 {
                        let pixel = self.image.get_pixel(start_x + dx, start_y + dy);
                        let subpixel = if dy as f32 >= height_ratio / 2.0 {
                            1
                        } else {
                            0
                        };
                        let channels = pixel.channels();
                        for (channel, rgba) in channels.iter().enumerate().take(4) {
                            total_channels[subpixel][channel] += *rgba as f32;
                        }
                    }
                }

                for subpixel in 0..2 {
                    let [total_r, total_g, total_b, _total_a] = total_channels[subpixel];
                    let avg_r = (total_r * gamma / count) as u8;
                    let avg_g = (total_g * gamma / count) as u8;
                    let avg_b = (total_b * gamma / count) as u8;
                    subpixels[subpixel] = Color::Rgb(avg_r, avg_g, avg_b);
                }

                let [top_subpixel, btm_subpixel] = subpixels;
                let threshold = 1.0;
                let color;
                let character;

                match (
                    total_channels[0][3] / count > threshold,
                    total_channels[1][3] / count > threshold,
                ) {
                    (true, false) => {
                        character = '\u{2580}'.to_string();
                        color = Style::new().fg(top_subpixel);
                    }
                    (false, true) => {
                        character = '\u{2584}'.to_string();
                        color = Style::new().fg(btm_subpixel);
                    }
                    (true, true) => {
                        character = '\u{2580}'.to_string();
                        color = Style::new().fg(top_subpixel).bg(btm_subpixel);
                    }
                    (false, false) => {
                        character = ' '.to_string();
                        color = Style::new();
                    }
                }

                line.push(Span::from(character).style(color));
            }
            lines.push(Line::from(line));
        }

        Text::from(lines)
    }
}
