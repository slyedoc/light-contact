use bevy::{prelude::*};
use bevy_asset_loader::prelude::*;



#[derive(AssetCollection)]
pub struct UiAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub font: Handle<Font>,
}


#[derive(AssetCollection)]
pub struct SpaceAssets {
    #[asset(paths(
        "space/astronautA.glb#Scene0",
        "space/astronautB.glb#Scene0",
        "space/alien.glb#Scene0",
        "space/barrel.glb#Scene0",
        "space/barrels_rail.glb#Scene0",
        "space/barrels.glb#Scene0",
        "space/bones.glb#Scene0",
    ), collection(typed))]
    pub models: Vec<Handle<Scene>>,
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
