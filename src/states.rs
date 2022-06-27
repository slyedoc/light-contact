mod intro;
use bevy::prelude::*;
pub use intro::*;
mod main_menu;
pub use main_menu::*;
mod map;
pub use map::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MainMenuPlugin)
            .add_plugin(IntroPlugin)
            .add_plugin(MapPlugin);
    }
}