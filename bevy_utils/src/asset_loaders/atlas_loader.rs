use bevy::{
    asset::{io::Reader, AssetLoader, AssetPath, AsyncReadExt, LoadContext},
    prelude::*,
    utils::BoxedFuture,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Defines how a `.atlas` file should be deserialized
#[derive(Serialize, Deserialize)]
struct AtlasRecord {
    pub name: String,
    pub texture_path: AssetPath<'static>,

    pub tile_size: Vec2,
    pub columns: usize,
    pub rows: usize,
    pub padding: Option<Vec2>,
    pub offset: Option<Vec2>,
}

#[derive(Error, Debug)]
pub enum AssetLoadError {
    #[error("Encountered an io error while loading atlas asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Encountered a deserialization error while loading atlas asset: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

pub struct AtlasLoader;

impl AssetLoader for AtlasLoader {
    type Asset = TextureAtlas;
    type Error = AssetLoadError;
    type Settings = ();

    fn extensions(&self) -> &[&str] { &["atlas"] }

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            // Read bytes from the reader
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            // Deserialize to an `AtlasRecord`
            let record = ron::de::from_bytes::<AtlasRecord>(&bytes)?;

            let texture = load_context.load(record.texture_path);

            Ok(TextureAtlas::from_grid(
                texture,
                record.tile_size,
                record.columns,
                record.rows,
                record.padding,
                record.offset,
            ))
        })
    }
}
