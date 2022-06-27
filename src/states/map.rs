use bevy::prelude::*;

use crate::{cleanup_system, enviroment::*, escape_system, style::AppStyle, AppState};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Map)
                .with_system(setup)
                .with_system(spawn_star),
        )
        //.add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(button_system))
        .add_system_set(SystemSet::on_update(AppState::Map).with_system(escape_system))
        .add_system_set(SystemSet::on_exit(AppState::Map).with_system(cleanup_system));
    }
}

fn setup(mut commands: Commands, style: Res<AppStyle>, mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::BLACK;
}
