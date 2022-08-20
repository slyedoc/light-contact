use std::time::Duration;

use crate::{cleanup_system, enviroment::*, escape_system, style::AppStyle, AppState, DELTA_TIME};
use bevy::{math::vec3, prelude::*, render::camera::Camera3d};
use iyes_loopless::prelude::*;
use rand::{thread_rng, Rng};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        // ... add systems to it ..
        app.add_enter_system_set(
            AppState::Map,
            SystemSet::new()
                .with_system(setup)
                .with_system(spawn_star_background)
                .with_system(spawn_solar_system),
        )
        .add_system(escape_system.run_in_state(AppState::Map))
         .add_stage_before(
             CoreStage::Update,
             "my_fixed_update",
             FixedTimestepStage::new(Duration::from_secs_f64(DELTA_TIME)).with_stage(
                 SystemStage::parallel()
                     .with_system(interact_bodies)
                     //.with_system(integrate.after(interact_bodies))
                 ),
         )
        .add_exit_system(AppState::Map, cleanup_system);
    }
}

fn setup(
    mut _commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    _style: Res<AppStyle>,
) {
    clear_color.0 = Color::BLACK;

    // move camera
    for mut c in camera_query.iter_mut() {
        c.translation = vec3(0.0, 10.0, -50.0);
        c.look_at(Vec3::ZERO, Vec3::Y)
    }
}

// TODO, move this to dynamic plugin for my physics

const GRAVITY_CONSTANT: f32 = 0.001;
const SOFTENING: f32 = 0.01;
const NUM_BODIES: usize = 20;

#[derive(Component, Default)]
struct Mass(f32);
#[derive(Component, Default)]
struct Acceleration(Vec3);
#[derive(Component, Default)]
struct LastPos(Vec3);


#[derive(Bundle, Default)]
struct BodyBundle {
    #[bundle]
    pbr: PbrBundle,
    mass: Mass,
    last_pos: LastPos,
    acceleration: Acceleration,
}

fn spawn_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    let mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 1.0,
        subdivisions: 3,
    }));

    let bodies = commands
        .spawn_bundle((Transform::default(), GlobalTransform::default()))
        .insert(Name::new("Solar System"))
        .id();

    let pos_range = 50.0..100.0;
    let color_range = 0.5..1.0;
    let vel_range = -0.5..0.5;

    // add bigger "star" body in the center
  
    let mut rng = thread_rng();
    for i in 0..NUM_BODIES {
        let mass_value_cube_root: f32 = rng.gen_range(0.4..6.0);
        let mass_value: f32 = mass_value_cube_root * mass_value_cube_root * mass_value_cube_root;

        let position = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-0.01..0.01),
            rng.gen_range(-1.0..1.0),
        )
        .normalize()
            * rng.gen_range(pos_range.clone());

        commands.entity(bodies).with_children(|parent| {
            parent
                .spawn_bundle(BodyBundle {
                    pbr: PbrBundle {
                        transform: Transform {
                            translation: position,
                            scale: Vec3::splat(mass_value_cube_root * 0.5),
                            ..Default::default()
                        },
                        mesh: mesh.clone(),
                        material: materials.add(
                            Color::rgb_linear(
                                rng.gen_range(color_range.clone()),
                                rng.gen_range(color_range.clone()),
                                rng.gen_range(color_range.clone()),
                            )
                            .into(),
                        ),
                        ..Default::default()
                    },
                    mass: Mass(mass_value),
                    acceleration: Acceleration(Vec3::ZERO),
                    last_pos: LastPos(
                        position
                            - Vec3::new(
                                rng.gen_range(vel_range.clone()),
                                rng.gen_range(vel_range.clone()),
                                rng.gen_range(vel_range.clone()),
                            ) * DELTA_TIME as f32,
                    ),
                })
                .insert(Name::new(format!("Body{}", i)));
        });
    }

    commands.entity(bodies).with_children(|parent| {
        parent
            .spawn_bundle(BodyBundle {
                pbr: PbrBundle {
                    transform: Transform {
                        scale: Vec3::splat(3.0),
                        ..Default::default()
                    },
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 1.0,
                        subdivisions: 5,
                    })),
                    material: materials.add((Color::ORANGE_RED * 10.0).into()),
                    ..Default::default()
                },
                mass: Mass(1000.0),
                ..Default::default()
            })
            .insert(Name::new("Star"));

        // star light
        parent
            .spawn_bundle(PointLightBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                point_light: PointLight {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    intensity: 100_000.0,
                    radius: 1000.0,
                    range: 1000.0,
                    ..default()
                },
                ..default()
            })
            .insert(Name::new("Star Light"));
    });

}

fn interact_bodies(mut query: Query<(&Mass, &GlobalTransform, &mut Acceleration)>) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(Mass(m1), transform1, mut acc1), (Mass(m2), transform2, mut acc2)]) =
        iter.fetch_next()
    {
        let delta = transform2.translation - transform1.translation;
        let distance_sq: f32 = delta.length_squared();

        let f = GRAVITY_CONSTANT / (distance_sq * (distance_sq + SOFTENING).sqrt());
        let force_unit_mass = delta * f;
        acc1.0 += force_unit_mass * *m2;
        acc2.0 -= force_unit_mass * *m1;
    }
}

fn _integrate(mut query: Query<(&mut Acceleration, &mut Transform, &mut LastPos)>) {
    let dt_sq = (DELTA_TIME * DELTA_TIME) as f32;
    for (mut acceleration, mut transform, mut last_pos) in query.iter_mut() {
        // verlet integration
        // x(t+dt) = 2x(t) - x(t-dt) + a(t)dt^2 + O(dt^4)
        let new_pos =
            transform.translation + transform.translation - last_pos.0 + acceleration.0 * dt_sq;
        acceleration.0 = Vec3::ZERO;
        last_pos.0 = transform.translation;
        transform.translation = new_pos;
    }
}
