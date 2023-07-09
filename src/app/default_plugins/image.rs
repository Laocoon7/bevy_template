use bevy::{app::PluginGroupBuilder, prelude::*};

pub(super) fn set_image_plugin<T: PluginGroup>(default_plugins: T) -> PluginGroupBuilder {
    default_plugins.set(ImagePlugin::default_nearest())
}
