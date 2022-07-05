use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::{
    assets::AudioAssets, cleanup_system, enviroment::*, escape_system, style::AppStyle, AppState,
};

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Intro)
                .with_system(setup)
                .with_system(spawn_light)
                .with_system(spawn_ground)
                .with_system(start_audio),
        )
        .add_system_set(SystemSet::on_update(AppState::Intro).with_system(escape_system))
        .add_system_set(SystemSet::on_exit(AppState::Intro).with_system(cleanup_system));
    }
}

fn setup(mut _commands: Commands, _style: Res<AppStyle>) {}

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.set_volume(0.3);
    audio.play_looped(audio_assets.charactor_ai_system_warning.clone());
    //audio.pause();
}
