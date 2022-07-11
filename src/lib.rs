//#![allow(warnings)]
#![allow(clippy::type_complexity)]
use bevy::{app::AppExit, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::plugin::InspectorWindows;
use bevy_inspector_egui::Inspectable;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::Audio;
use bevy_kira_audio::AudioPlugin;

mod assets;
mod cursor;
mod enviroment;
mod overlay;
mod player;
mod states;
mod style;

use iyes_loopless::prelude::*;
use sly_camera_controller::{CameraController, CameraControllerPlugin};
use sly_physics::prelude::*;


use assets::*;
use cursor::*;
use enviroment::*;
use overlay::*;
use player::Player;
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
    Reset,
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(AppState::Splash)
            .init_resource::<ResetTarget>()
            
            // crates
            .add_plugin(PhysicsPlugin)
            .add_plugin(PhysicsBvhCameraPlugin)
            .add_plugin(CameraControllerPlugin)
            .add_plugin(WorldInspectorPlugin::new())

            // local plugins
            .add_plugin(CursorPlugin)
            .add_plugin(AudioPlugin)
            .add_plugin(StylePlugin)
            .add_plugin(OverlayPlugin)
            .add_plugin(EnviromentPlugin)
            .add_plugin(AppAssetsPlugin)

            // states
            .add_plugin(StatePlugin)

            // states
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
            .add_startup_system(setup_camera)
            
            // reset state
            .add_enter_system(AppState::Reset, reset)
            .add_system(reset_listen)
            .add_system(toggle_state);
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
        .insert(BvhCamera::new(256, 256))
        .insert(Player)

        .insert(Keep);
}

fn toggle_state(mut input: ResMut<Input<KeyCode>>, mut state: ResMut<State<PhysicsState>>) {
    if input.just_pressed(KeyCode::Space) {
        match state.current() {            
            PhysicsState::Running => state.push(PhysicsState::Paused).unwrap(),
            PhysicsState::Paused => state.pop().unwrap(),
        }
        input.clear();
    }
}


pub struct ResetTarget {
    target: AppState,    
}
impl Default for ResetTarget {
    fn default() -> Self {
        Self {
            target: AppState::MainMenu,            
        }
    }
}
pub fn reset(
    mut commands: Commands,
    reset_target: Res<ResetTarget>) {
    // go back
    commands.insert_resource(NextState(reset_target.target));    
}

pub fn reset_listen(
    mut commands: Commands,
    mut input: ResMut<Input<KeyCode>>,
    mut reset_target: ResMut<ResetTarget>,
    current_state: ResMut<CurrentState<AppState>>,
) {
    if input.just_pressed(KeyCode::R) {
        reset_target.target = current_state.0;

        commands.insert_resource(NextState(AppState::Reset));        
        // TODO remove clear
        input.clear();
    }
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

pub fn hide_window<T: Inspectable + Send + Sync + 'static>(
    mut inspector_windows: ResMut<InspectorWindows>,
) {
    let mut inspector_window_data = inspector_windows.window_data_mut::<T>();
    inspector_window_data.visible = false;
}

pub fn show_window<T: Inspectable + Send + Sync + 'static>(
    mut inspector_windows: ResMut<InspectorWindows>,
) {
    let mut inspector_window_data = inspector_windows.window_data_mut::<T>();
    inspector_window_data.visible = true;
}
