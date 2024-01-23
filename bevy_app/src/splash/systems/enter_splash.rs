use bevy::{
    core_pipeline::{bloom::BloomSettings, clear_color::ClearColorConfig, tonemapping::Tonemapping},
    prelude::*,
    render::camera::ScalingMode,
};

use crate::splash::{components::tags::SplashScreenTag, resources::SplashState};

pub fn enter_splash(mut commands: Commands) {
    // Spawn a camera for the splash screen
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::Fixed {
                    width: 1.0,
                    height: 1.0,
                },
                ..Default::default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            tonemapping: Tonemapping::TonyMcMapface,
            ..Default::default()
        },
        BloomSettings::default(),
        SplashScreenTag,
    ));

    commands.init_resource::<SplashState>();
}
