use std::{collections::VecDeque, path::PathBuf};

use bevy::prelude::*;
use bevy_data::constants::SPLASH_TIMER;

#[derive(Resource)]
pub struct SplashState {
    timer: Option<Timer>,
    file_list: VecDeque<PathBuf>,
}

impl Default for SplashState {
    fn default() -> Self {
        // Read all files in the `./assets/splash` folder
        let file_list = if let Ok(paths) = std::fs::read_dir("./bevy_app/assets/splash") {
            paths
                // remove directories
                .filter_map(|entry| entry.ok())
                // change to pathbufs
                .map(|entry| entry.path())
                // remove files that don't end in `png`
                .filter_map(|path| {
                    if path.extension().map_or(false, |ext| ext == "png") {
                        Some(path)
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            VecDeque::new()
        };

        info!("file_list: {:?}", &file_list);

        Self {
            timer: None,
            file_list,
        }
    }
}

impl SplashState {
    pub fn tick(&mut self, time: &Time) -> bool {
        if let Some(timer) = self.timer.as_mut() {
            timer.tick(time.delta());
            timer.just_finished()
        } else {
            self.timer = Some(Timer::from_seconds(SPLASH_TIMER, TimerMode::Repeating));
            true
        }
    }

    pub fn next_path(&mut self) -> Option<PathBuf> { self.file_list.pop_front() }
}
