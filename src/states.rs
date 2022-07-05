mod intro;
use bevy::prelude::*;
pub use intro::*;
mod main_menu;
pub use main_menu::*;
mod map;
mod loading;
pub use loading::*;
pub use map::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(LoadingPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(IntroPlugin)
            .add_plugin(MapPlugin);
    }
}
