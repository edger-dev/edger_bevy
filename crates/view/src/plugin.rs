use crate::prelude::*;
use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "shape")]
        ColorBackground::setup(app);
    }
}