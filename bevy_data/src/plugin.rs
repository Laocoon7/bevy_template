use bevy::prelude::*;

use crate::states::{AppState, InputState};

pub struct BevyDataPlugin;

impl Plugin for BevyDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>();
        app.add_state::<InputState>();
    }
}
