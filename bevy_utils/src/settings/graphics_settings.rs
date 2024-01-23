use bevy::window::WindowResolution;
use bevy_data::constants::{DEFAULT_SCREEN_HEIGHT_F32, DEFAULT_SCREEN_WIDTH_F32};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphicsSettings {
    resolution: WindowResolution,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            resolution: WindowResolution::new(DEFAULT_SCREEN_WIDTH_F32, DEFAULT_SCREEN_HEIGHT_F32),
        }
    }
}

impl GraphicsSettings {
    pub fn window_resolution(&self) -> WindowResolution { self.resolution.clone() }

    pub fn set_window_resolution(&mut self, resolution: WindowResolution) { self.resolution = resolution; }
}
