use std::path::PathBuf;

use log::debug;
use ratatui::{prelude::*, widgets::Paragraph};

use crate::{
    ascii_art::{AsciiOptions, ImageConverter, ToAsciiArt},
    Renderable,
};
use pipedream_domain::image::Image;

impl Renderable for Image {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        debug!(target:"Interface/Image/Render", "{:?} at {:?}", self, area);
        let image = ImageConverter::from(&PathBuf::from(self.path.clone()));

        let area_aspect = (area.width as f32 / 2.) / area.height as f32;
        let image_aspect = image.image.width() as f32 / image.image.height() as f32;
        let width_major = image_aspect > area_aspect;

        let options = if width_major {
            AsciiOptions {
                width: Some(area.width),
                ..Default::default()
            }
        } else {
            AsciiOptions {
                height: Some(area.height),
                ..Default::default()
            }
        };
        let ascii_text = image.to_ascii_art(Some(options));

        Paragraph::new(ascii_text)
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}
