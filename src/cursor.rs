use bevy::prelude::*;
use sly_physics::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_cursor)
            .add_system_to_stage(
                PhysicsFixedUpdate,
                move_cursor.after(PhysicsSystems::ResolvePhase),
            );
    }
}

#[derive(Component)]
pub struct Cursor;

fn setup_cursor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.1,
                sectors: 30,
                stacks: 30,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(1.0, 0.0, 0.0, 0.2),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .insert(Cursor)
        .insert(Name::new("Cursor"));
}

#[allow(dead_code)]
fn move_cursor(
    windows: Res<Windows>,
    camera_query: Query<(&GlobalTransform, &PerspectiveProjection)>,
    mut cusror_query: Query<(&mut Transform, &mut Visibility), With<Cursor>>,
    tlas: Res<Tlas>,
) {
    if let Some(window) = windows.get_primary() {
        if let Some(mouse_pos) = window.cursor_position() {
            if let Ok((trans, projection)) = camera_query.get_single() {
                if let Ok((mut cursor_trans, mut cursor_vis)) = cusror_query.get_single_mut() {
                    // create a ray
                    let mut ray = Ray::from_screenspace(mouse_pos, window, projection, trans);

                    // test ray agaist tlas and see if we hit
                    if let Some(hit) = ray.intersect_tlas(&tlas) {
                        // we could do something with the entity here
                        cursor_trans.translation = ray.origin + ray.direction * hit.distance;
                        cursor_vis.is_visible = true;
                    } else {
                        cursor_vis.is_visible = false;
                    }
                }
            }
        }
    }
}
