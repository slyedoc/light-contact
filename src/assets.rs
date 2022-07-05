use bevy::{gltf::Gltf, prelude::*};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct SpaceAssets {
    #[asset(path = "space/astronautA.glb#Node-astronautA")]
    pub astronaut_a: Handle<Gltf>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<bevy_kira_audio::AudioSource>,

    #[asset(path = "characters/ai/warning_system_status.ogg")]
    pub charactor_ai_system_warning: Handle<bevy_kira_audio::AudioSource>,
}

//                 Character::AstronautA => "space/astronautA.glb#Node-astronautA",
//                 Character::AstronautB => "space/astronautB.glb#Node-astronautB",
//                 Character::Alien => "space/alien.glb#Node-alien",
