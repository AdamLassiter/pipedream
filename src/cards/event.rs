use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum NodeInteractionType {
    Hover,
    LeftClick,
    LeftDrop,
    LeftDrag,
    RightClick,
}

#[derive(Event, Debug)]
pub struct NodeInteraction {
    pub entity: Entity,
    pub interaction_type: NodeInteractionType,
}

#[derive(Default)]
pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NodeInteraction>();
    }
}
