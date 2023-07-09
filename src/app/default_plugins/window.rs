use bevy::{app::PluginGroupBuilder, prelude::*, window::Cursor};

use crate::{
    APP_CANVAS, APP_CURSOR_GRAB_MODE, APP_CURSOR_HIT_TEST, APP_CURSOR_ICON, APP_CURSOR_VISIBLE,
    APP_DECORATIONS, APP_FIT_CANVAS_TO_PARENT, APP_MODE, APP_POSITION, APP_RESIZABLE, APP_RESIZE_CONSTRAINTS,
    APP_RESOLUTION, APP_TITLE, APP_TRANSPARENT, APP_WINDOW_LEVEL,
};

pub(super) fn set_window_plugin<T: PluginGroup>(default_plugins: T) -> PluginGroupBuilder {
    let mut cursor = Cursor::default();
    cursor.icon = APP_CURSOR_ICON;
    cursor.visible = APP_CURSOR_VISIBLE;
    cursor.grab_mode = APP_CURSOR_GRAB_MODE;
    cursor.hit_test = APP_CURSOR_HIT_TEST;

    default_plugins.set(WindowPlugin {
        primary_window: Some(Window {
            cursor,
            mode: APP_MODE,
            position: APP_POSITION,
            resolution: APP_RESOLUTION.into(),
            title: APP_TITLE.to_string(),
            resize_constraints: APP_RESIZE_CONSTRAINTS,
            resizable: APP_RESIZABLE,
            decorations: APP_DECORATIONS,
            transparent: APP_TRANSPARENT,
            window_level: APP_WINDOW_LEVEL,
            canvas: APP_CANVAS,
            fit_canvas_to_parent: APP_FIT_CANVAS_TO_PARENT,
            ..Default::default()
        }),
        ..Default::default()
    })
}
