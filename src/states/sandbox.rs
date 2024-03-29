use std::f32::consts::FRAC_PI_2;

use bevy::{math::vec3, prelude::*};
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use iyes_loopless::prelude::*;
use sly_physics::{prelude::*, PHYSISCS_TIMESTEP};

use crate::{*, enviroment::*, escape_system, AppState};

pub struct SandboxPlugin;

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(InspectorPlugin::<Stack>::new())
            .add_startup_system(hide_window::<Stack>)
            .add_enter_system_set(
                AppState::Sandbox,
                SystemSet::new()
                    .with_system(setup)
                    .with_system(spawn_light)
                    .with_system(setup_room)
                    .with_system(show_window::<Stack>),
            )
            .add_system(escape_system.run_in_state(AppState::Sandbox))
            .add_system(apply_scale.run_in_state(AppState::Sandbox))
            .add_exit_system(AppState::Sandbox, cleanup_system)
            .add_exit_system(AppState::Sandbox, hide_window::<Stack>);
    }
}

fn setup() {}

fn apply_scale(mut config: ResMut<PhysicsConfig>, stack: Res<Stack>) {
    config.time = stack.time_scale * PHYSISCS_TIMESTEP as f32;
}

#[derive(Inspectable)]
pub struct Stack {
    count: u32,
    //#[inspector()]
    time_scale: f32,
    mode: StackMode,
    ball_velocity: f32,
}

#[derive(Inspectable)]
pub enum StackMode {
    Sphere,
    Cube,
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            count: 5,
            time_scale: 1.0,
            mode: StackMode::Cube,
            ball_velocity: 60.0,
        }
    }
}

pub fn setup_room(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    stack: Res<Stack>,
) {
    let floor_size = 100.0;
    let wall_height = 10.0;
    let floor_half = floor_size * 0.5;
    let wall_height_half = wall_height * 0.5;
    // floor
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            mesh: meshes.add(Mesh::from(shape::Box::new(floor_size, 1.0, floor_size))),
            material: materials.add(StandardMaterial {
                base_color: Color::DARK_GREEN,
                ..default()
            }),
            ..default()
        })
        .insert_bundle(RigidBodyBundle {
            collider: Collider::Cuboid {
                size: vec3(floor_size, 1.0, floor_size),
            },
            mode: RigidBodyMode::Static,
            ..default()
        })
        .insert(Name::new("Floor"));

    // walls
    for wall in 0..4 {
        let mut transform = Transform::from_xyz(0.0, wall_height_half, floor_half);
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_axis_angle(Vec3::Y, wall as f32 * FRAC_PI_2),
        );

        commands
            .spawn_bundle(PbrBundle {
                transform,
                mesh: meshes.add(Mesh::from(shape::Box::new(floor_size, wall_height, 1.0))),
                material: materials.add(StandardMaterial {
                    base_color: Color::ALICE_BLUE,
                    ..default()
                }),
                ..default()
            })
            .insert_bundle(RigidBodyBundle {
                collider: Collider::Cuboid {
                    size: vec3(floor_size, wall_height, 1.0),
                },
                mode: RigidBodyMode::Static,
                ..default()
            })
            .insert(Name::new("Wall"));
    }

    // Stack
    let radius = 0.5;
    let size = stack.count;
    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                let pos = vec3(
                    i as f32,
                    j as f32 + radius,
                    k as f32 - (radius * 2.0 * size as f32 / 2.0),
                ) * 1.1;
                commands
                    .spawn_bundle(PbrBundle {
                        //mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
                        mesh: match stack.mode {
                            StackMode::Sphere => meshes.add(Mesh::from(shape::UVSphere {
                                radius,
                                ..default()
                            })),
                            StackMode::Cube => {
                                meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0)))
                            }
                        },
                        material: materials.add(StandardMaterial {
                            base_color_texture: Some(asset_server.load("textures/checker_red.png")),
                            ..default()
                        }),
                        transform: Transform::from_translation(pos),
                        ..default()
                    })
                    .insert_bundle(RigidBodyBundle {
                        collider: match stack.mode {
                            StackMode::Sphere => Collider::Sphere { radius },
                            StackMode::Cube => Collider::Cuboid { size: Vec3::ONE },
                        },
                        mode: RigidBodyMode::Dynamic,
                        ..default()
                    })
                    .insert(Name::new(format!("Sphere ({}, {}, {})", i, j, k)));
            }
        }
    }

    // Wreaking Ball

    let wreak_ball_radius = 2.0;

    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(30.0, wreak_ball_radius + 1.0, 0.0),
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: wreak_ball_radius,
                sectors: 64,
                stacks: 64,
            })),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("textures/checker_red.png")),
                ..default()
            }),
            ..default()
        })
        .insert_bundle(RigidBodyBundle {
            linear_velocity: LinearVelocity(vec3(-stack.ball_velocity, 0.0, 0.0)),
            collider: Collider::Sphere {
                radius: wreak_ball_radius,
            },
            mass: Mass(20.0),
            ..default()
        })
        .insert(Name::new("Wreaking Ball"));
}
