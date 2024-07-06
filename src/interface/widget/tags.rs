use std::collections::BTreeSet;

use log::debug;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{Paragraph, Widget},
};
use tui_markup::{compile, generator::RatatuiTextGenerator};

use crate::engine::core::tag::{Static, Tag, TagKey, Tags};

static RENDERABLE_TAGS: Static<BTreeSet<TagKey>> = Static::new(|| {
    BTreeSet::from_iter(vec![
        "player:resource:health".into(),
        "player:resource:stamina".into(),
        "player:resource:mana".into(),
        "player:resource:faith".into(),
    ])
});

impl Widget for &Tags {
    fn render(self, area: Rect, buf: &mut Buffer) {
        debug!(target: "Render/Tags", "{:?}", self);

        let renderable = RENDERABLE_TAGS
            .clone()
            .into_iter()
            .filter_map(|key| self.get(&key).cloned().map(|value| Tag { key, value }))
            .map(|tag| format!("{}", tag))
            .collect::<Vec<_>>();

        let mut scene = Text::default();
        renderable
            .iter()
            .map(|tag| {
                compile::<RatatuiTextGenerator>(tag.as_str())
                    .expect("Failed to compile tui text markup")
            })
            .for_each(|scene_line| scene.extend(scene_line));

        Widget::render(Paragraph::new(scene), area, buf);
    }
}
