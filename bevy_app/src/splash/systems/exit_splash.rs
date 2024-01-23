use bevy::prelude::*;

use crate::splash::{components::tags::SplashScreenTag, resources::SplashState};

pub fn exit_splash(mut commands: Commands, q_splash_screen: Query<Entity, With<SplashScreenTag>>) {
    for entity in q_splash_screen.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<SplashState>();
}
