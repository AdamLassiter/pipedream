use std::collections::BTreeMap;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};
use tui_markup::{compile, generator::RatatuiTextGenerator};

use pipedream_engine::{
    combat::{entity::Ent, target::Tgt},
    core::tags::{Static, Tag, TagKey, TAG_STYLES},
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
    pub tags: &'a Vec<Tag>,
}

impl <'a> Renderable for TgtEntTags<'a> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        debug!(target:"Interface/Tags/Render", "{:?} at {:?}", self, area);

        // Must live long enough
        let display_strs = self
            .tags
            .iter()
            .filter(|tag| {
                tag.key
                    .0
                    .starts_with(format!("{}:{}", self.tgt, self.ent).as_str())
            })
            .map(display)
            .collect::<Vec<_>>();

        let scene = display_strs
            .iter()
            .flat_map(|display_str| {
                let compiled = compile::<RatatuiTextGenerator>(display_str.as_str())
                    .expect("Failed to compile tui text markup");
                compiled.lines
            })
            .collect::<Vec<_>>();

        Widget::render(Paragraph::new(scene), area, buf);
    }
}

fn display(tag: &Tag) -> String {
    let style = TAG_STYLES
        .get(tag.key.trailing_key())
        .copied()
        .unwrap_or("x");
    format!("<{} {}={}>", style, tag.key.trailing_key(), tag.value)
}
