#![allow(clippy::type_complexity)]

use bevy::{
    asset::io::{file::FileAssetReader, AssetSource},
    prelude::*,
    window::WindowMode,
};
use bevy_data::{
    constants::{WINDOW_DECORATIONS, WINDOW_RESIZEABLE},
    prelude::*,
    BevyDataPlugin,
};
use bevy_utils::{prelude::*, BevyUtilsPlugin};
use game::GamePlugin;

use crate::{
    splash::SplashPlugin,
    systems::{quit_game, set_window_icon},
};

pub(crate) mod splash;
pub(crate) mod systems;

/// Program entry point
fn main() {
    let mut app = App::new();

    // Asset Sources must be setup before DefaultPlugins
    setup_asset_sources(&mut app);
    setup_default_plugins(&mut app);
    setup_additional_plugins(&mut app);
    setup_systems(&mut app);

    app.run();
}

/// TODO: Add other asset sources here as needed
fn setup_asset_sources(app: &mut App) {
    // Create url "[BASE_DIR]://path/to/asset"
    // linking to "./path/to/asset"
    // This removes the assets folder from being added automatically
    app.register_asset_source(
        BASE_DIR,
        AssetSource::build().with_reader(move || Box::new(FileAssetReader::new("../"))),
    );
}

fn setup_default_plugins(app: &mut App) {
    let settings = Settings::read();
    let graphics_settings = settings.graphics();

    // Add DefaultPlugins
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: graphics_settings.window_resolution(),
                    title: APP_NAME.to_string(),
                    resizable: WINDOW_RESIZEABLE,
                    decorations: WINDOW_DECORATIONS,
                    mode: WindowMode::Windowed,
                    position: WindowPosition::At(IVec2::ZERO),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()),
    );
}

fn setup_additional_plugins(app: &mut App) {
    app.add_plugins((BevyDataPlugin, BevyUtilsPlugin, SplashPlugin, GamePlugin));
}

fn setup_systems(app: &mut App) {
    app.add_systems(Startup, set_window_icon);

    app.add_systems(OnEnter(AppState::Quit), quit_game);
}
