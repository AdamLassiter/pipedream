#![allow(clippy::needless_range_loop)]

use std::fmt::Debug;
use std::iter::zip;
use std::path::Path;

use image::io::Reader;
use image::{DynamicImage, GenericImageView, Pixel};
use pipedream_engine::log::debug;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};

#[derive(Debug)]
pub struct AsciiOptions {
    pub height: Option<u16>,
    pub width: Option<u16>,
    pub gamma: f32,
}

impl Default for AsciiOptions {
    fn default() -> Self {
        Self {
            height: Default::default(),
            width: Default::default(),
            gamma: 1.0,
        }
    }
}

pub trait ToAsciiArt {
    fn to_ascii_art(&self, options: Option<AsciiOptions>) -> Text;
}

pub struct ImageConverter {
    pub image: DynamicImage,
    threshold: f32,
}

impl ImageConverter {
    pub fn new(image: DynamicImage) -> Self {
        Self {
            image,
            threshold: 256. / 3.,
        }
    }

    fn sum_channels(
        &self,
        width_ratio: f32,
        height_ratio: f32,
        start_x: u32,
        start_y: u32,
    ) -> ([[[f32; 4]; 2]; 2], [[i32; 2]; 2]) {
        let mut total_channels = [[[0.0; 4]; 2]; 2];
        let mut count = [[0; 2]; 2];

        for (sub_x, mut dx_range) in [
            (0, 0..(width_ratio / 2.) as u32),
            (1, (width_ratio / 2.) as u32..width_ratio as u32),
        ] {
            if dx_range.is_empty() {
                dx_range = 0..1;
            }
            for (sub_y, mut dy_range) in [
                (0, 0..(height_ratio / 2.) as u32),
                (1, (height_ratio / 2.) as u32..height_ratio as u32),
            ] {
                if dy_range.is_empty() {
                    dy_range = 0..1;
                }
                for dx in dx_range.clone() {
                    for dy in dy_range.clone() {
                        let pixel = self.image.get_pixel(start_x + dx, start_y + dy);
                        let channels = pixel.channels();
                        for chan in 0..4 {
                            total_channels[sub_x][sub_y][chan] += channels[chan] as f32;
                        }
                        count[sub_x][sub_y] += 1;
                    }
                }
            }
        }

        // Downscale
        (total_channels, count)
    }

    fn mean_channels(
        &self,
        channels: [[[f32; 4]; 2]; 2],
        count: [[i32; 2]; 2],
        gamma: f32,
    ) -> [[[u8; 4]; 2]; 2] {
        let mut mean_channels = [[[0_u8; 4]; 2]; 2];
        for sub_x in 0..2 {
            for sub_y in 0..2 {
                for channel in 0..4 {
                    let total = channels[sub_x][sub_y][channel];
                    let count = count[sub_x][sub_y] as f32;
                    let mean = total / count;
                    mean_channels[sub_x][sub_y][channel] = (mean * gamma) as u8;
                }
            }
        }
        mean_channels
    }

    fn two_means_cluster_iter(
        &self,
        clusters: [Vec<[u8; 4]>; 2],
    ) -> ([[f32; 4]; 2], [Vec<[u8; 4]>; 2]) {
        let mut new_centres = [[0.0; 4]; 2];
        for x in 0..clusters.len() {
            let cluster = &clusters[x];
            for y in 0..cluster.len() {
                let point = cluster[y];
                for chan in 0..4 {
                    new_centres[x][chan] += point[chan] as f32 / cluster.len() as f32;
                }
            }
        }

        let mut new_clusters: [Vec<[u8; 4]>; 2] = [Vec::new(), Vec::new()];
        for x in 0..clusters.len() {
            let cluster = &clusters[x];
            for y in 0..cluster.len() {
                let point = cluster[y];
                let mut distances = [0.0; 2];
                for centre_idx in 0..2 {
                    let centre = new_centres[centre_idx];
                    let dist: f32 = zip(point, centre)
                        .map(|(p, p0)| (p as f32 - p0).powi(2))
                        .sum();
                    distances[centre_idx] = dist;
                }

                if distances[0] < distances[1] {
                    new_clusters[0].push(point);
                } else {
                    new_clusters[1].push(point);
                }
            }
        }

        (new_centres, new_clusters)
    }

    fn two_means_cluster(&self, points: Vec<[u8; 4]>) -> ([[f32; 4]; 2], [Vec<[u8; 4]>; 2]) {
        let (l, r) = points.split_at(1);
        let mut init_clusters = [l.to_vec(), r.to_vec()];
        let mut next_centres;
        let mut next_clusters;

        while {
            (next_centres, next_clusters) = self.two_means_cluster_iter(init_clusters.clone());

            if next_clusters == init_clusters {
                false
            } else {
                init_clusters = next_clusters.clone();
                true
            }
        } {}

        (next_centres, next_clusters)
    }

    fn subpixel_render(
        &self,
        channels: [[[u8; 4]; 2]; 2],
        centres: [[f32; 4]; 2],
        clusters: [Vec<[u8; 4]>; 2],
    ) -> (char, Style) {
        let colors = centres.map(|[r, g, b, a]| {
            Some(Color::Rgb(r as u8, g as u8, b as u8)).filter(|_| a > self.threshold)
        });

        let (primary_idx, secondary_idx) = if colors[0].is_none() { (1, 0) } else { (0, 1) };

        let mut bitmask = 0u8;
        for sub_x in 0..2 {
            for sub_y in 0..2 {
                if clusters[primary_idx].contains(&channels[sub_x][sub_y])
                    && centres[primary_idx][3] > self.threshold
                {
                    bitmask += 1 << (sub_x + 2 * sub_y);
                }
            }
        }

        let characters = [
            ' ', '▘', '▝', '▀', '▖', '▌', '▞', '▛', '▗', '▚', '▐', '▜', '▄', '▙', '▟', '█',
        ];
        let character = characters[bitmask as usize];

        let mut style = Style::new();
        if let Some(fg_color) = colors[primary_idx] {
            style = style.fg(fg_color);
        }
        if let Some(bg_color) = colors[secondary_idx] {
            style = style.bg(bg_color);
        }

        (character, style)
    }
}

impl<P> From<P> for ImageConverter
where
    P: AsRef<Path> + Debug + Clone,
{
    fn from(value: P) -> Self {
        let open_file = Reader::open(value.clone())
            .unwrap_or_else(|_| panic!("No such file or directory {:?}", value));
        let image = open_file
            .decode()
            .unwrap_or_else(|_| panic!("No such file or directory {:?}", value));
        Self::new(image)
    }
}

impl ToAsciiArt for ImageConverter {
    fn to_ascii_art(&self, options: Option<AsciiOptions>) -> Text {
        let options = options.unwrap_or_default();
        debug!(target:"Interface/AsciiArt/ToAsciiArt", "{:?}", options);

        let AsciiOptions {
            height,
            width,
            gamma,
        } = options;

        let (width_ratio, height_ratio) = match (height, width) {
            (Some(_), Some(_)) | (None, None) => panic!("Expected just one dimension for image"),
            (Some(height), None) => {
                let height_ratio = self.image.height() as f32 / height as f32;
                (height_ratio / 2., height_ratio)
            }
            (None, Some(width)) => {
                let width_ratio = self.image.width() as f32 / width as f32;
                (width_ratio, width_ratio * 2.)
            }
        };

        let width = (self.image.width() as f32 / width_ratio) as u16;
        let height = (self.image.height() as f32 / height_ratio) as u16;

        debug!(target:"Interface/AsciiArt/Ratios", "{:?}", (width_ratio, height_ratio));

        let mut lines = vec![];
        for y in 0..height {
            let mut line = vec![];
            let mut log_line = vec![];
            for x in 0..width {
                let start_x = (x as f32 * width_ratio) as u32;
                let start_y = (y as f32 * height_ratio) as u32;

                let (total_channels, count) =
                    self.sum_channels(width_ratio, height_ratio, start_x, start_y);
                let mean_channels = self.mean_channels(total_channels, count, gamma);
                let (centres, clusters) =
                    self.two_means_cluster(mean_channels.as_flattened().to_vec());
                let (character, style) = self.subpixel_render(mean_channels, centres, clusters);
                line.push(Span::from(character.to_string()).style(style));
                log_line.push(character.to_string());
            }
            debug!(target:"Interface/AsciiArt/Image", "{}",  log_line.join(""));

            lines.push(Line::from(line));
        }

        Text::from(lines)
    }
}
