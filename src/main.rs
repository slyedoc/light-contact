//#![allow(warnings)]
#![allow(clippy::type_complexity)]
//use bevy_inspector_egui::prelude::*;
use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
    window::PresentMode,
};
//use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
//mod grid;
//use grid::*;
mod camera_controller;
use camera_controller::*;
mod fadeout;
use fadeout::*;
mod states;
use states::*;
mod style;
use style::*;
mod enviroment;
mod overlay;
use overlay::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum AppState {
    AssetLoading,
    MainMenu,
    Intro,
    Map,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_state(AppState::MainMenu)
        .add_plugins(DefaultPlugins)
        //.add_plugin(InfiniteGridPlugin)
        .add_plugin(StylePlugin)
        .add_plugin(FadeoutPlugin)
        .add_plugin(OverlayPlugin)
        .add_plugin(CameraControllerPlugin)
        //.add_plugin(WorldInspectorPlugin::new())
        .add_plugin(StatePlugin)
        // global systems
        .add_startup_system_to_stage(StartupStage::PreStartup, setup)
        .run();
}

pub struct CameraImage {
    pub image: Handle<Image>,
    pub width: u32,
    pub height: u32,
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>, mut images: ResMut<Assets<Image>>) {
    let window = windows.get_primary_mut().unwrap();
    let size = Extent3d {
        width: window.physical_width(),
        height: window.physical_height(),
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);
    let image_handle = images.add(image);

    commands
        .spawn_bundle(Camera3dBundle {
            // camera_3d: Camera3d {
            //     clear_color: ClearColorConfig::Custom(Color::WHITE),
            //     ..default()
            // },
            camera: Camera {
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 2.0, 15.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(CameraController::default())
        .insert(Keep);

    commands.insert_resource(CameraImage {
        width: window.physical_width(),
        height: window.physical_height(),
        image: image_handle,
    });
}

#[derive(Component)]
pub struct Keep;

fn cleanup_system(mut commands: Commands, q: Query<Entity, Without<Keep>>) {
    for e in q.iter() {
        info!("removing entity {:?}", e);
        commands.entity(e).despawn_recursive();
    }
}

fn escape_system(mut fadeout: EventWriter<Fadeout>, mut input: ResMut<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        info!("Exiting Intro");
        fadeout.send(Fadeout::Pop);
        input.clear();
    }
}
