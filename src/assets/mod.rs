use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

mod space;
pub use space::*;

pub struct AppAssetsPlugin;

impl Plugin for AppAssetsPlugin {
    fn build(&self, app: &mut App) {
        // assets
        app.add_plugin(SpaceAssetPlugin);
    }
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<bevy_kira_audio::AudioSource>,

    #[asset(path = "characters/ai/warning_system_status.ogg")]
    pub charactor_ai_system_warning: Handle<bevy_kira_audio::AudioSource>,
}

#[derive(AssetCollection)]
pub struct UiAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub font: Handle<Font>,
}
