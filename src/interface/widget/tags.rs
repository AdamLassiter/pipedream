use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::Text,
    widgets::{Paragraph, Widget},
};
use tui_markup::{compile, generator::RatatuiTextGenerator};

use crate::{
    engine::core::tag::{Static, Tag, TagKey, Tags},
    prefab::{tag_engine::Ent, tags::Tgt},
};

static RENDERABLE_TAGS: Static<Vec<TagKey>> = Static::new(|| {
    vec![
        format!("{}:{}:health", Tgt::Player, Ent::Resource)
            .as_str()
            .into(),
        format!("{}:{}:stamina", Tgt::Player, Ent::Resource)
            .as_str()
            .into(),
        format!("{}:{}:mana", Tgt::Player, Ent::Resource)
            .as_str()
            .into(),
        format!("{}:{}:faith", Tgt::Player, Ent::Resource)
            .as_str()
            .into(),
    ]
});

impl Widget for &Tags {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // debug!(target:"Render/Tags", "{:?}", self);

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

        Widget::render(Paragraph::new(scene).alignment(Alignment::Right), area, buf);
    }
}
