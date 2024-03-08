use bevy::prelude::*;

pub struct ShapePlugin;

impl Plugin for ShapePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_prototype_lyon::prelude::ShapePlugin);
    }
}