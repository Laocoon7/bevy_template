use bevy::prelude::*;

use self::{image::set_image_plugin, log::set_log_plugin, window::set_window_plugin};

mod image;
mod log;
mod window;

pub(super) fn add_default_plugins(app: &mut App) {
    let default_plugins = DefaultPlugins;
    let default_plugins = set_window_plugin(default_plugins);
    let default_plugins = set_image_plugin(default_plugins);
    let default_plugins = set_log_plugin(default_plugins);

    app.add_plugins(default_plugins);
}
