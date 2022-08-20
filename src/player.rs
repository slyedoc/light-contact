use bevy::prelude::*;
use sly_physics::prelude::*;

#[derive(Component)]
pub struct Player;

#[allow(dead_code)]
pub fn spawn_player( mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::RED.into()),
                ..default()
            })
            .insert_bundle(RigidBodyBundle {
                collider: Collider::Cuboid { size: Vec3::ONE },
                ..default()
            });
    }
