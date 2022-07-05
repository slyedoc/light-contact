use crate::{
    assets::SpaceAssets, cleanup_system, enviroment::*, escape_system, style::AppStyle, AppState,
};
use bevy::{core::FixedTimestep, gltf::Gltf, math::vec3, prelude::*, render::camera::Camera3d};
use rand::{thread_rng, Rng};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Map)
                .with_system(setup)
                .with_system(spawn_star_background)
                .with_system(generate_bodies),
        )
        //.add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(button_system))
        .add_system_set(SystemSet::on_update(AppState::Map).with_system(escape_system))
        .add_system_set(
            SystemSet::on_update(AppState::Map)
                .with_run_criteria(FixedTimestep::step(DELTA_TIME))
                .with_system(interact_bodies)
                .with_system(integrate.after(interact_bodies)), //.with_system(spawn_test_system),
        )
        .add_system_set(SystemSet::on_exit(AppState::Map).with_system(cleanup_system));
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

#[allow(dead_code)]
fn setup_ship(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // create some orbital bodies that
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 1.0,
                ..default()
            })),
            ..default()
        })
        .insert(Name::new("Ship"));
}

const DELTA_TIME: f64 = 1.0 / 60.0;
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

fn generate_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 1.0,
        subdivisions: 3,
    }));

    let pos_range = 50.0..100.0;
    let color_range = 0.5..1.0;
    let vel_range = -0.5..0.5;

    let bodies = commands
        .spawn_bundle((Transform::default(), GlobalTransform::default()))
        .insert(Name::new("Solar System"))
        .id();

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

    // add bigger "star" body in the center
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

fn integrate(mut query: Query<(&mut Acceleration, &mut Transform, &mut LastPos)>) {
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

#[allow(dead_code)]
fn spawn_test_system(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    space_assets: Res<SpaceAssets>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    if input.just_pressed(KeyCode::Key3) {
        // to be able to position our 3d model:
        // spawn a parent entity with a TransformBundle
        // and spawn our gltf as a scene under it

        if let Some(gltf) = assets_gltf.get(&space_assets.astronaut_a) {
            commands.spawn_scene(gltf.scenes[0].clone());
        }
    }
}
