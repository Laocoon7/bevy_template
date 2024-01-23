# BevyTemplate
BevyTemplate is used to quickly scaffold a Bevy(ver 0.12) project.

## `bevy_app`
- Creates the `bevy::prelude::App`
- Registers any custom asset sources (See `bevy::asset::AssetPath.source`)
- Creates the `DefaultPlugins`
- Sets up additional plugins
- Sets the primary window's favicon
- Shows any splash screens located in `bevy_app/assets/splash`
- Sets `bevy_data::prelude::AppState::MainMenu` once all splash screens are done

## `bevy_data`
- Holds `AppState` and `InputState`
- Holds various constants and configurations

## `bevy_utils`
- Registers an AssetLoader for TextureAtlases
- Provides a static `Files` api for getting common files/directories
- Provides a static `Settings` api for save/loading settings
- Provides an extention trait to allow easy save/loading to ron/toml

## `game`
- Your code goes here...