use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use bevy::prelude::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{files::Files, settings::GraphicsSettings, traits::SaveLoadToml};

mod graphics_settings;
pub use self::graphics_settings::*;

lazy_static! {
    static ref SETTINGS: RwLock<Settings> = RwLock::new(Settings::load());
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    graphics: GraphicsSettings,

    // Keep track of mutable accesses
    // We will never load from serde as dirty
    #[serde(skip, default = "never_dirty")]
    dirty: bool,
}

fn never_dirty() -> bool { false }

impl Default for Settings {
    fn default() -> Self {
        Self {
            // If we are built not from serde, we are dirty
            dirty: true,
            graphics: GraphicsSettings::default(),
        }
    }
}

impl Settings {
    fn load() -> Self {
        let mut settings: Self = SaveLoadToml::load_toml(Files::settings()).unwrap_or_default();
        // save if we created from default
        settings.save();
        settings
    }

    /// Save these settings to file
    pub fn save(&mut self) {
        if self.dirty {
            if let Err(e) = self.save_toml(Files::settings()) {
                error!("{}", e);
            }
            self.dirty = false;
        }
    }

    /// Overwrite current settings with these settings
    /// This function calls Settings::write()
    pub fn apply(&self) {
        let mut old = Self::write();
        old.graphics = self.graphics.clone();
    }

    /// Get read access to the current settings
    pub fn read() -> RwLockReadGuard<'static, Self> {
        SETTINGS.read().expect("Failed to get settings read lock")
    }

    /// Get write access to the current settings
    pub fn write() -> RwLockWriteGuard<'static, Self> {
        let mut settings = SETTINGS.write().expect("Failed to get settings write lock");
        // we got mutable access, mark ourselves dirty.
        settings.dirty = true;
        settings
    }

    pub fn graphics(&self) -> &GraphicsSettings { &self.graphics }

    pub fn graphics_mut(&mut self) -> &mut GraphicsSettings { &mut self.graphics }
}
