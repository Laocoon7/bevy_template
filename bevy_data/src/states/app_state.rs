use bevy::prelude::*;

/// The main state for the app
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    /// Showing the splash screen
    #[default]
    Splash,
    /// Displaying the main menu
    MainMenu,
    /// Playing the game
    Game,
    /// Displaying the settings menu
    Settings,
    /// Quitting the app
    Quit,
}
