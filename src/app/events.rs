use bevy::prelude::*;

use super::state::AppState;

#[derive(Event, Clone, Debug)]
pub struct WindowResizedEvent{
    pub last_width: f32,
    pub last_height: f32,
}

impl WindowResizedEvent {
    pub fn new(state: &AppState) -> Self {
        Self { last_width: state.window_width, last_height: state.window_height, }
    }
}

#[derive(Event, Clone, Debug)]
pub struct MouseClickedEvent {
    pub cursor_position: Vec2,
}

#[derive(Event, Clone, Debug)]
pub struct MouseDraggedEvent {
    pub cursor_position: Vec2,
    pub delta: Vec2,
}

pub fn add_app_events(app: &mut App) {
    app.add_event::<WindowResizedEvent>();
    app.add_event::<MouseClickedEvent>();
    app.add_event::<MouseDraggedEvent>();
}
