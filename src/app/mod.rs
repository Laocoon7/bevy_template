use bevy::prelude::*;

use self::{default_plugins::add_default_plugins, icon::set_window_icon};

mod default_plugins;
mod icon;

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        add_default_plugins(app);
        app.add_system(set_window_icon.on_startup());
    }
}
