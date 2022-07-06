mod intro;
use bevy::prelude::*;
pub use intro::*;
mod main_menu;
pub use main_menu::*;
mod loading;
mod map;
mod sandbox;
pub use loading::*;
pub use map::*;
pub use sandbox::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LoadingPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(SandboxPlugin)
            .add_plugin(IntroPlugin)
            .add_plugin(MapPlugin);
    }
}
