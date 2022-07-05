//#![allow(warnings)]
#![allow(clippy::type_complexity)]
//use bevy_inspector_egui::prelude::*;
use bevy::{app::AppExit, prelude::*};
//use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
//mod grid;
//use grid::*;
mod assets;
mod enviroment;
mod fadeout;
mod overlay;
mod states;
mod style;

//use bevy_infinite_grid::{InfiniteGridPlugin};

use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use fadeout::*;
use overlay::*;
use sly_camera_controller::{CameraController, CameraControllerPlugin};
use states::*;
use style::*;
use bevy_kira_audio::{Audio};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum AppState {
    Loading,
    MainMenu,
    Intro,
    Map,
}

pub struct MainCamera(pub Entity);

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Loading)
            //.add_plugin(InfiniteGridPlugin)
            .add_plugin(AudioPlugin)
            .add_plugin(StylePlugin)
            .add_plugin(FadeoutPlugin)
            .add_plugin(OverlayPlugin)
            .add_plugin(CameraControllerPlugin)
            .add_plugin(WorldInspectorPlugin::new())
            .add_plugin(StatePlugin)
            .add_startup_system_to_stage(StartupStage::Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    // cameras
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(Keep);

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 2.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(CameraController::default())
        .insert(Keep);
}

#[derive(Component)]
pub struct Keep;

fn cleanup_system(mut commands: Commands, q: Query<Entity, Without<Keep>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn escape_system(
    mut fadeout: EventWriter<Fadeout>,
    mut app_exit: EventWriter<AppExit>,
    state: Res<State<AppState>>,
    mut input: ResMut<Input<KeyCode>>,
    audio: Res<Audio>,
) {
    if input.just_pressed(KeyCode::Escape) {
        if state.current().eq(&AppState::MainMenu) {
            app_exit.send(AppExit);
        } else {
            fadeout.send(Fadeout::Pop);
        }
        input.clear();
        audio.pause()
    }
}