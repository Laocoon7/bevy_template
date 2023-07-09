use app::AppPlugin;
use bevy::prelude::*;
use game::GamePlugin;

mod app;

mod config;
pub use self::config::*;

#[cfg(feature = "dev")]
mod debug;
#[cfg(not(feature = "dev"))]
use bevy_egui::EguiPlugin as DebugPlugin;

#[cfg(feature = "dev")]
use crate::debug::DebugPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugin(AppPlugin).add_plugin(DebugPlugin).add_plugin(GamePlugin);

    app.run();
}
