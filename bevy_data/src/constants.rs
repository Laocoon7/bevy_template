pub const DOMAIN: &'static str = "tech";
pub const COMPANY: &'static str = "CyberCitadel";
pub const APP_NAME: &'static str = "Grifter";

// pub const APP: &'static str = "app";
pub const BASE_DIR: &'static str = "base";

pub const TOML_EXTENTION: &'static str = "toml";
pub const RON_EXTENTION: &'static str = "ron";
pub const SCENE_EXTENTION: &'static str = "scn";

pub const SETTINGS_FILE_NAME: &'static str = "Settings";
pub const SAVE_FOLDER_NAME: &'static str = "Save";

pub const DEFAULT_SCREEN_WIDTH_F32: f32 = 1920.0;
pub const DEFAULT_SCREEN_HEIGHT_F32: f32 = 1080.0;

pub const WINDOW_RESIZEABLE: bool = false;
pub const WINDOW_DECORATIONS: bool = true;

#[cfg(feature = "dev")]
pub const SPLASH_TIMER: f32 = 1.0;
#[cfg(not(feature = "dev"))]
pub const SPLASH_TIMER: f32 = 5.0;
