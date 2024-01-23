use std::io::Cursor;

use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use winit::window::Icon;

pub fn set_window_icon(windows: NonSend<WinitWindows>, primary_windows: Query<Entity, With<PrimaryWindow>>) {
    let primary_entity = primary_windows.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        error!("Failed to get the primary winit window");
        return;
    };

    let icon_buf = Cursor::new(include_bytes!("../../assets/app/favicon.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let Ok(icon) = Icon::from_rgba(rgba, width, height) else {
            error!("Failed to load window icon");
            return;
        };
        primary.set_window_icon(Some(icon));
    }
}
