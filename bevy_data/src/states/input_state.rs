use bevy::prelude::*;

/// The state governing whether or not to allow input
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputState {
    /// Allow input to be processed
    #[default]
    Allow,
    /// Don't take input
    Disallow,
    /// Special mode to adjust keybindings
    Set,
}
