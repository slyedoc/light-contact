use bevy::prelude::*;

use crate::{
    cleanup_system,
    enviroment::{spawn_ground, spawn_light},
    escape_system,
    style::AppStyle,
    AppState,
};

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Intro)
                .with_system(setup)
                .with_system(spawn_light)
                .with_system(spawn_ground),
        )
        .add_system_set(SystemSet::on_update(AppState::Intro).with_system(escape_system))
        .add_system_set(SystemSet::on_exit(AppState::Intro).with_system(cleanup_system));
    }
}

fn setup(mut _commands: Commands, _style: Res<AppStyle>) {}
