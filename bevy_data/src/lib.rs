#![allow(clippy::redundant_static_lifetimes)]

pub mod constants;
pub mod states;

mod plugin;
pub use self::plugin::*;

pub mod prelude {
    pub use crate::{
        constants::{APP_NAME, BASE_DIR, COMPANY, DOMAIN, RON_EXTENTION, SCENE_EXTENTION, TOML_EXTENTION},
        states::*,
    };
}
