pub(crate) mod asset_loaders;
pub mod files;
pub mod settings;
pub mod traits;

mod plugin;
pub use self::plugin::*;

pub mod prelude {
    pub use crate::{files::*, settings::*, traits::*};
}
