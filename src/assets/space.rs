use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::RegisterInspectable;
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;

pub struct SpaceAssetPlugin;

impl Plugin for SpaceAssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_space_asset.run_if_resource_exists::<SpaceAssets>())  
            .add_system(update_space_asset.run_if_resource_exists::<SpaceAssets>())            
            .register_inspectable::<SpaceAssetType>();
    }
}

fn spawn_space_asset(
    mut commands: Commands,
    space_assets: Res<SpaceAssets>,
    query: Query<(Entity, &SpaceAssetType), Changed<SpaceAssetType>>,
) {

    for (e, sa_type) in query.iter() {
        let scene = space_assets.get(sa_type);
        commands.entity(e)
            .insert_bundle(RigidBodyBundle {
                collider: Collider::sphere(0.5),                    
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_scene(scene.clone());
            });
    }
}

fn update_space_asset(
    mut commands: Commands,
    space_assets: Res<SpaceAssets>,
    mut query: Query<( Entity, &SpaceAssetType, &Children, Option<&mut Name>), Changed<SpaceAssetType>>,
) {

    for (e, sa_type, children, name_opt) in query.iter_mut() {
        // delete old children
        for child in children.iter() {
            commands.entity(*child).despawn_recursive();
        }

        let new_scene = space_assets.get(sa_type);

        // Set name if present
        if let Some(mut name) = name_opt {
            let new_name = format!("{:?}", sa_type);
            name.set(new_name);
        }
        
        commands.entity(e)
            .with_children(|parent| {
                parent.spawn_scene(new_scene);
            });

        // TODO: reset physics
    }
}

#[derive(AssetCollection)]
pub struct SpaceAssets {
    
    // Note: Order matters here
    #[asset(
        paths(
            "space/astronautA.glb#Scene0",
            "space/astronautB.glb#Scene0",
            "space/alien.glb#Scene0",
            "space/barrel.glb#Scene0",
            "space/barrels_rail.glb#Scene0",
            "space/barrels.glb#Scene0",
            "space/bones.glb#Scene0",
        ),
        collection(typed)
    )]
    pub models: Vec<Handle<Scene>>,
}

impl SpaceAssets {
    pub fn get(&self, sa_type: &SpaceAssetType) -> Handle<Scene> {
        match sa_type {
            SpaceAssetType::Character(c) => {
                match c {
                    Character::AstronautA => self.models[0].clone(),
                    Character::AstronautB => self.models[1].clone(),
                    Character::Alien => self.models[2].clone(),
                }
            },
            SpaceAssetType::Prop(p) => {
                match p {
                    Prop::Barrel => self.models[3].clone(),
                    Prop::BarrelsRail => self.models[4].clone(),
                    Prop::Barrels => self.models[5].clone(),
                    Prop::Bones => self.models[6].clone(),
                }
            },
        }        
    }

    pub fn list() -> Vec<SpaceAssetType> {
        return vec![
            SpaceAssetType::Character(Character::AstronautA),
            SpaceAssetType::Character(Character::AstronautB),
            SpaceAssetType::Character(Character::Alien),
            SpaceAssetType::Prop(Prop::Barrel),
            SpaceAssetType::Prop(Prop::BarrelsRail),
            SpaceAssetType::Prop(Prop::Barrels),
            SpaceAssetType::Prop(Prop::Bones),
        ]
    }
}
#[derive(Component, Debug, Inspectable)]
pub enum SpaceAssetType {
    Character(Character),
    Prop(Prop),
}

impl Default for SpaceAssetType {
    fn default() -> Self {
        SpaceAssetType::Character(Character::AstronautA)
    }
}

#[derive(Inspectable, Debug)]
pub enum Character {
    AstronautA,
    AstronautB,
    Alien,
}

impl Default for Character {
    fn default() -> Self {
        Character::AstronautA
    }
}

#[derive(Inspectable, Debug)]
pub enum Prop {
    Barrel,
    BarrelsRail,
    Barrels,
    Bones,
}

impl Default for Prop {
    fn default() -> Self {
        Prop::Barrel
    }
}
