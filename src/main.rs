//#![allow(warnings)]
#![allow(clippy::type_complexity)]
//use bevy_inspector_egui::prelude::*;
use bevy::{app::AppExit, prelude::*, window::PresentMode};
//use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
//mod grid;
//use grid::*;
mod enviroment;
mod fadeout;
mod overlay;
mod states;
mod style;
mod assets;

//use bevy_infinite_grid::{InfiniteGridPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use sly_camera_controller::{CameraController, CameraControllerPlugin};
use bevy_asset_loader::prelude::*;
use fadeout::*;
use overlay::*;
use states::*;
use style::*;
use assets::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum AppState {
    AssetLoading,
    MainMenu,
    Intro,
    Map,
}

pub struct MainCamera(pub Entity);


fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading)
                .continue_to_state(AppState::MainMenu)
                .with_collection::<SpaceAssets>(),
        )
        .add_state(AppState::AssetLoading)
        .add_plugins(DefaultPlugins)
        //.add_plugin(InfiniteGridPlugin)
        .add_plugin(StylePlugin)
        .add_plugin(FadeoutPlugin)
        .add_plugin(OverlayPlugin)
        .add_plugin(CameraControllerPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(StatePlugin)

        // global starup
        .add_startup_system_to_stage(StartupStage::Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // light
    commands
        .spawn_bundle(DirectionalLightBundle {
            transform: Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Keep);

    // cameras
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(Keep);

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 2.0, -2.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        // Add our controller
        .insert(CameraController::default())
        .insert(Keep);



    // commands.spawn_bundle(InfiniteGridBundle::default())
    // .insert(Keep);
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
) {
    if input.just_pressed(KeyCode::Escape) {
        if state.current().eq(&AppState::MainMenu) {
            app_exit.send(AppExit);
        } else {
            fadeout.send(Fadeout::Pop);
        }
        input.clear();
    }
}
