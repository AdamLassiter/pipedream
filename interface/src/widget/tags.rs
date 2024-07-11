use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::{Paragraph, Widget},
};
use tui_markup::{compile, generator::RatatuiTextGenerator};

use pipedream_engine::{
    combat::{entity::Ent, target::Tgt},
    core::tags::{Static, Tag, TagKey, TagValue, Tags},
};

use crate::Renderable;

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

impl Renderable for Tags {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        // debug!(target:"Render/Tags", "{:?}", self);

        let renderable = RENDERABLE_TAGS
            .clone()
            .into_iter()
            .map(|key| {
                self.get(&key)
                    .cloned()
                    .map(|value| Tag {
                        key: key.clone(),
                        value,
                    })
                    .unwrap_or(Tag {
                        key,
                        value: TagValue::Number(0.into()),
                    })
            })
            .map(|tag| format!("{}", tag))
            .collect::<Vec<_>>();

        let scene = renderable
            .iter()
            .map(|tag| {
                compile::<RatatuiTextGenerator>(tag.as_str())
                    .expect("Failed to compile tui text markup")
            })
            .flat_map(|text| text.lines)
            .collect::<Vec<_>>();

        Widget::render(Paragraph::new(scene).alignment(Alignment::Right), area, buf);
    }
}
