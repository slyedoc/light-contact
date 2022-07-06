use bevy::prelude::*;
use bevy_kira_audio::Audio;
use iyes_loopless::prelude::*;

use crate::{
    assets::AudioAssets, cleanup_system, enviroment::*, escape_system, style::AppStyle, AppState,
};

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::Intro,
            SystemSet::new()
                .with_system(setup)
                .with_system(spawn_light)
                .with_system(spawn_ground)
                .with_system(start_audio),
        )
        .add_system(escape_system.run_in_state(AppState::Intro))
        .add_exit_system(AppState::Intro, cleanup_system);
    }
}

fn setup(mut _commands: Commands, _style: Res<AppStyle>) {}

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.set_volume(0.3);
    audio.play_looped(audio_assets.charactor_ai_system_warning.clone());
    //audio.pause();
}
