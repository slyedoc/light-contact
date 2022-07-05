#![allow(clippy::forget_non_drop)]
// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
use bevy::window::PresentMode;
use light_contact::AppPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::Fifo,
            width: 800.,
            height: 600.,
            title: "Light Contact".to_string(), // ToDo
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AppPlugin)
        .run();
}
