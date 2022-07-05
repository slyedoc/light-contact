use bevy::{prelude::*, gltf::Gltf};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct SpaceAssets {
    #[asset(path = "space/astronautA.glb#Node-astronautA")]
    pub astronaut_a: Handle<Gltf>,
}

//                 Character::AstronautA => "space/astronautA.glb#Node-astronautA",
//                 Character::AstronautB => "space/astronautB.glb#Node-astronautB",
//                 Character::Alien => "space/alien.glb#Node-alien",
