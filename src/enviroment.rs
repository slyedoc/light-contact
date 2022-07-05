use std::f32::consts::*;

use bevy::{math::vec2, prelude::*};
use rand::Rng;

pub fn spawn_light(mut commands: Commands) {
    // light
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::DARK_GREEN,
                cull_mode: None,
                ..default()
            }),
            ..default()
        })
        .insert(Name::new("Ground"));
}

const NUM_BACKGROUND: usize = 2000;
const BACKGROUND_RANGE: f32 = 500.0;

#[allow(dead_code)]
pub fn spawn_star_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Quad {
        size: vec2(1.0, 1.0),
        ..default()
    }));

    let background = commands
        .spawn_bundle((Transform::default(), GlobalTransform::default()))
        .insert(Name::new("Background"))
        .id();

    let color_range = 0.5..1.0;

    let mut rng = rand::thread_rng();

    for _ in 0..NUM_BACKGROUND {
        let size: f32 = rng.gen_range(1.0..2.0);

        let direction = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        )
        .normalize();

        let mut transform = Transform {
            translation: direction * BACKGROUND_RANGE,
            scale: Vec3::splat(size),
            ..default()
        }
        .looking_at(Vec3::ZERO, Vec3::Y);

        transform.rotate(Quat::from_axis_angle(transform.right(), PI));

        commands.entity(background).with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                transform,
                mesh: mesh.clone(),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb_linear(
                        rng.gen_range(color_range.clone()),
                        rng.gen_range(color_range.clone()),
                        rng.gen_range(color_range.clone()),
                    ),
                    unlit: true,
                    ..Default::default()
                }),
                ..Default::default()
            });
        });
    }
}
