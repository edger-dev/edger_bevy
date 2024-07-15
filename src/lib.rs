pub use bevy;

// https://github.com/bevyengine/bevy/issues/3659
pub mod bevy_prelude {
    pub use bevy::prelude::*;
    pub use bevy::ecs as bevy_ecs;
    pub use bevy::reflect as bevy_reflect;
}

pub use unic_langid;

#[cfg(feature = "assets")]
pub mod assets;

pub mod app;

pub use edger_bevy_util;

#[cfg(feature = "shape")]
pub use edger_bevy_shape;

#[cfg(feature = "shape")]
pub use edger_bevy_shape::bevy_prototype_lyon;

#[cfg(feature = "view")]
pub use edger_bevy_view;

#[cfg(feature = "view")]
pub mod view;

#[cfg(feature = "egui")]
pub use edger_bevy_egui;

#[cfg(feature = "egui")]
pub use edger_bevy_egui::bevy_egui;

#[cfg(feature = "egui")]
pub use edger_bevy_egui::egui;

#[cfg(feature = "assets")]
pub use bevy_asset_loader;

pub mod prelude {
    pub use crate::app::events::{WindowResizedEvent, MouseClickedEvent, MouseDraggedEvent};
    pub use crate::app::state::AppState;
    pub use crate::app::run_2d_app;

    #[cfg(feature = "assets")]
    pub use crate::assets::{PreloadAssets, AssetsPlugin, AssetsStates, init_preload_assets, insert_preload_assets, add_assets_loaded_systems, register_file_asset};

    #[doc(hidden)]
    pub use edger_bevy_util::prelude::*;

    #[cfg(feature = "shape")]
    #[doc(hidden)]
    pub use edger_bevy_shape::prelude::*;

    #[cfg(feature = "view")]
    #[doc(hidden)]
    pub use edger_bevy_view::prelude::*;

    #[cfg(feature = "view")]
    #[doc(hidden)]
    pub use crate::view::root_layout::DoRootLayoutEvent;

    #[cfg(feature = "egui")]
    #[doc(hidden)]
    pub use edger_bevy_egui::prelude::*;
}
