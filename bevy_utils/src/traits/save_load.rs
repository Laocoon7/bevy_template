use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SaveLoadError {
    #[error("Encountered an io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Encountered a ron serialization error: {0}")]
    RonSer(#[from] ron::Error),
    #[error("Encountered a ron deserialization error: {0}")]
    RonDe(#[from] ron::error::SpannedError),
    #[error("Encountered a toml serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("Encountered a toml deserialization error: {0}")]
    TomlDe(#[from] toml::de::Error),
}

pub trait SaveLoadRon: Serialize + for<'de> Deserialize<'de> {
    fn load_ron<P: AsRef<Path>>(path: P) -> Result<Self, SaveLoadError> {
        let contents = std::fs::read_to_string(path)?;
        Ok(ron::from_str::<Self>(&contents)?)
    }

    #[cfg(feature = "dev")]
    fn save_ron<P: AsRef<Path>>(&self, path: P) -> Result<(), SaveLoadError> {
        let contents = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?;
        let mut path_buf = path.as_ref().to_path_buf();
        path_buf.pop();
        std::fs::create_dir_all(&path_buf)?;
        Ok(std::fs::write(path, contents)?)
    }

    #[cfg(not(feature = "dev"))]
    fn save_ron<P: AsRef<Path>>(&self, path: P) -> Result<(), SaveLoadError> {
        let contents = ron::to_string(self)?;
        let mut path_buf = path.as_ref().to_path_buf();
        path_buf.pop();
        std::fs::create_dir_all(&path_buf)?;
        Ok(std::fs::write(path, contents)?)
    }
}

impl<T: Serialize + for<'de> Deserialize<'de>> SaveLoadRon for T {}

pub trait SaveLoadToml: Serialize + for<'de> Deserialize<'de> {
    fn load_toml<P: AsRef<Path>>(path: P) -> Result<Self, SaveLoadError> {
        let contents = std::fs::read_to_string(path)?;
        Ok(toml::from_str::<Self>(&contents)?)
    }

    fn save_toml<P: AsRef<Path>>(&self, path: P) -> Result<(), SaveLoadError> {
        let contents = toml::to_string_pretty(self)?;
        let mut path_buf = path.as_ref().to_path_buf();
        path_buf.pop();
        std::fs::create_dir_all(&path_buf)?;
        Ok(std::fs::write(path, contents)?)
    }
}

impl<T: Serialize + for<'de> Deserialize<'de>> SaveLoadToml for T {}
