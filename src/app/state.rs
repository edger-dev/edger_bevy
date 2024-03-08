use std::path::PathBuf;
use bevy::prelude::*;
use bevy::asset::{AssetPath, Asset};

#[derive(Resource, Default)]
pub struct AppState {
    pub window_width: f32,
    pub window_height: f32,
    pub window_scale_factor: f32,
    pub scale_factor_override: Option<f32>,

    #[cfg(feature = "assets")]
    pub assets: Vec<UntypedHandle>
}

impl AppState {
    pub fn convert_pos(&self, pos: Vec2) -> Vec2 {
        Vec2::new(
            pos.x - self.window_width / 2.0,
            self.window_height / 2.0 - pos.y,
        )
    }

    #[cfg(feature = "assets")]
    pub fn get_asset<A: Asset>(&self, path: PathBuf) -> Option<Handle<A>> {
        let handle_path = AssetPath::from(path);
        let mut handle = None;
        for asset in self.assets.iter() {
            if asset.path() == Some(&handle_path) {
                handle = Some(asset.clone().typed::<A>());
                break;
            }
        }
        handle
    }
}