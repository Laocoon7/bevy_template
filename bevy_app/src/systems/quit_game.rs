use bevy::{app::AppExit, prelude::*};

pub fn quit_game(mut e_app_exit: EventWriter<AppExit>) { e_app_exit.send(AppExit); }
