use bevy::prelude::*;
use bevy_data::prelude::*;

use crate::splash::systems::{enter_splash, exit_splash, update_splash};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Splash), enter_splash);
        app.add_systems(Update, update_splash.run_if(in_state(AppState::Splash)));
        app.add_systems(OnExit(AppState::Splash), exit_splash);
    }
}
