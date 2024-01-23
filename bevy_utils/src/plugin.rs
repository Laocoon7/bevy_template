use bevy::prelude::*;

use crate::asset_loaders::AtlasLoader;

pub struct BevyUtilsPlugin;

impl Plugin for BevyUtilsPlugin {
    fn build(&self, app: &mut App) { app.register_asset_loader(AtlasLoader); }
}
