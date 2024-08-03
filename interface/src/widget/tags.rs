use std::collections::BTreeMap;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols::border,
    widgets::{Block, Borders, Paragraph, Widget},
};
use tui_markup::{compile, generator::RatatuiTextGenerator};

use pipedream_engine::{
    core::tags::{Static, TagKey, TagValue, Tags, TAG_STYLES},
    game::{entity::Ent, target::Tgt},
    log::debug,
};

use crate::Renderable;

pub static IMAGE_TAGS: Static<BTreeMap<Tgt, TagKey>> = Static::new(|| {
    BTreeMap::from_iter(
        vec![Tgt::Player, Tgt::Enemy]
            .into_iter()
            .map(|tgt| (tgt, TagKey(format!("{}:image:", tgt)))),
    )
});

#[derive(Debug)]
pub struct TgtEntTags<'a> {
    pub tgt: Tgt,
    pub ent: Ent,
    pub tags: &'a Tags,
}

impl<'a> TgtEntTags<'a> {
    fn display_stats(&self) -> Vec<String> {
        let stats = self
            .tags
            .iter()
            .filter(|(key, _value)| key.0.starts_with(&self.tgt.ent(self.ent).0))
            .map(display)
            .collect::<Vec<_>>();
        if stats.is_empty() {
            vec!["None".to_string()]
        } else {
            stats
        }
    }

    pub fn size_hint(&self) -> u16 {
        self.display_stats().len() as u16 + 2
    }
}

impl<'a> Renderable for TgtEntTags<'a> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        debug!(target:"Interface/Tags/Render", "{:?} at {:?}", self, area);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.ent.pretty())
            .border_set(border::ROUNDED);

        let stats = self.display_stats(); // Must live long enough
        let stats = stats
            .iter()
            .flat_map(|display_str| {
                let compiled = compile::<RatatuiTextGenerator>(display_str.as_str())
                    .expect("Failed to compile tui text markup");
                compiled.lines
            })
            .collect::<Vec<_>>();

        Widget::render(Paragraph::new(stats), block.inner(area), buf);
        block.render(area, buf);
    }
}

fn display((key, value): (&TagKey, &TagValue)) -> String {
    let style = TAG_STYLES.get(key.trailing_key()).copied().unwrap_or("x");
    format!("<{} {} {}>", style, value, key.trailing_key())
}
