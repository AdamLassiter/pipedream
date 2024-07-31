use std::collections::BTreeMap;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};
use tui_markup::{compile, generator::RatatuiTextGenerator};

use pipedream_engine::{
    combat::{entity::Ent, target::Tgt},
    core::tags::{Static, Tag, TagKey, TagValue, Tags},
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

impl Renderable for Vec<Tag> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let this: Tags = Tags::new(
            self.clone()
                .into_iter()
                .map(|tag| (tag.key, tag.value))
                .collect(),
            vec![],
            vec![],
        );

        debug!(target:"Interface/Tags/Render", "{:?} at {:?}", self, area);

        let renderable = RENDERABLE_TAGS
            .clone()
            .into_iter()
            .map(|key| {
                this.get(&key)
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

        Widget::render(Paragraph::new(scene), area, buf);
    }
}
