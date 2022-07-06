//#![allow(warnings)]
#![allow(clippy::type_complexity)]
use bevy::{app::AppExit, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::Audio;
use bevy_kira_audio::AudioPlugin;

pub mod assets;
mod enviroment;
mod fadeout;
mod overlay;
mod states;
mod style;

use assets::*;
use fadeout::*;

use iyes_loopless::prelude::*;
use overlay::*;
use sly_camera_controller::{CameraController, CameraControllerPlugin};

use sly_physics::PhysicsPlugin;
use states::*;
use style::*;

const DELTA_TIME: f64 = 1.0 / 60.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum AppState {
    Splash,
    Loading,
    MainMenu,
    Sandbox,
    Intro,
    Map,
}

pub struct MainCamera(pub Entity);

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loopless_state(AppState::Splash)
            // crates
            .add_plugin(PhysicsPlugin)
            .add_plugin(CameraControllerPlugin)
            .add_plugin(WorldInspectorPlugin::new())
            //.add_plugin(InfiniteGridPlugin)

            // local plugins
            .add_plugin(AudioPlugin)
            .add_plugin(StylePlugin)
            .add_plugin(FadeoutPlugin)
            .add_plugin(OverlayPlugin)

            // assets
            .add_loading_state(
                LoadingState::new(AppState::Splash)
                    .continue_to_state(AppState::Loading)
                    .with_collection::<UiAssets>(),
            )
            .add_loading_state(
                LoadingState::new(AppState::Loading)
                    .continue_to_state(AppState::MainMenu)
                    .with_collection::<AudioAssets>()
                    .with_collection::<SpaceAssets>(),
            )
            // states
            .add_plugin(StatePlugin)
            .add_startup_system(setup_camera);
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

    // commands
    //     .spawn_bundle(InfiniteGridBundle::default())
    //     .insert(Keep);
}

#[derive(Component)]
pub struct Keep;

fn cleanup_system(mut commands: Commands, q: Query<Entity, Without<Keep>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn escape_system(
    mut commands: Commands,
    mut app_exit: EventWriter<AppExit>,
    state: Res<CurrentState<AppState>>,
    mut input: ResMut<Input<KeyCode>>,
    audio: Res<Audio>,
) {
    if input.just_pressed(KeyCode::Escape) {
        if state.0.eq(&AppState::MainMenu) {
            app_exit.send(AppExit);
        } else {
            commands.insert_resource(NextState(AppState::MainMenu));
        }
        input.clear();
        audio.pause()
    }
}
