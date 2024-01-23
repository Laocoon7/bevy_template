use std::{
    path::PathBuf,
    sync::{RwLock, RwLockReadGuard},
};

use bevy::prelude::*;
use bevy_data::{
    constants::{SAVE_FOLDER_NAME, SETTINGS_FILE_NAME},
    prelude::*,
};
use directories::ProjectDirs;
use lazy_static::lazy_static;

lazy_static! {
    static ref FOLDERS: RwLock<Files> = RwLock::new(Files::default());
}

pub struct Files(ProjectDirs);

impl Default for Files {
    fn default() -> Self {
        Self(ProjectDirs::from(DOMAIN, COMPANY, APP_NAME).expect("Failed to get project directories"))
    }
}

impl Files {
    fn read() -> RwLockReadGuard<'static, Self> { FOLDERS.read().expect("Failed to get folders read lock") }

    fn config() -> PathBuf { Self::read().0.config_dir().to_path_buf() }

    fn data() -> PathBuf { Self::read().0.data_dir().to_path_buf() }

    /// Returns a `PathBuf` pointing to the settings file for this application
    pub fn settings() -> PathBuf {
        let mut path = Self::config();
        if let Err(e) = std::fs::create_dir_all(&path) {
            error!("{}", e);
        }
        path.push(SETTINGS_FILE_NAME);
        path.set_extension(TOML_EXTENTION);
        path
    }

    /// Returns a `PathBuf` pointing to the save folder for this application
    /// It is suggested to `PathBuf::push` more organized system from here before
    /// `std::fs::create_dir_all`
    pub fn save() -> PathBuf {
        let mut path = Self::data();
        if let Err(e) = std::fs::create_dir_all(&path) {
            error!("{}", e);
        }
        path.push(SAVE_FOLDER_NAME);
        path
    }
}
