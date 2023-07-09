use bevy::{
    prelude::*,
    window::{CursorGrabMode, WindowLevel, WindowMode},
};

pub const APP_ICON: &[u8] = include_bytes!("../assets/app/icon_256x256.png");
pub const APP_CURSOR_ICON: CursorIcon = CursorIcon::Default;
pub const APP_CURSOR_VISIBLE: bool = true;
pub const APP_CURSOR_GRAB_MODE: CursorGrabMode = CursorGrabMode::None;
pub const APP_CURSOR_HIT_TEST: bool = true;

pub const APP_MODE: WindowMode = WindowMode::Windowed;
pub const APP_POSITION: WindowPosition = WindowPosition::Automatic;
pub const APP_RESOLUTION: Vec2 = Vec2 {
    x: 1920.0,
    y: 1080.0,
};
pub const APP_TITLE: &str = "GAME";
pub const APP_RESIZE_CONSTRAINTS: WindowResizeConstraints = WindowResizeConstraints {
    min_width: 800.0,
    min_height: 600.0,
    max_width: f32::INFINITY,
    max_height: f32::INFINITY,
};
pub const APP_RESIZABLE: bool = true;
pub const APP_DECORATIONS: bool = true;
pub const APP_TRANSPARENT: bool = false;
pub const APP_WINDOW_LEVEL: WindowLevel = WindowLevel::Normal;
pub const APP_CANVAS: Option<String> = None;
pub const APP_FIT_CANVAS_TO_PARENT: bool = true;
