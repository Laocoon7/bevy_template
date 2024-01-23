use bevy::prelude::*;
use bevy_data::prelude::*;

use crate::splash::{
    components::tags::{SplashScreenImageTag, SplashScreenTag},
    resources::SplashState,
};

pub fn update_splash(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut splash_state: ResMut<SplashState>,
    mut next_state: ResMut<NextState<AppState>>,
    q_images: Query<Entity, With<SplashScreenImageTag>>,
) {
    // Check if we are ready for next image
    if splash_state.tick(&time) {
        // Despawn old images
        for entity in q_images.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // Spawn new image
        if let Some(path) = splash_state.next_path() {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::ONE),
                        ..Default::default()
                    },
                    texture: asset_server.load(format!("{}://{}", BASE_DIR, path.display())),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..Default::default()
                },
                SplashScreenImageTag,
                SplashScreenTag,
            ));
        } else {
            // No more images: Exit
            next_state.set(AppState::MainMenu);
        }
    }
}
