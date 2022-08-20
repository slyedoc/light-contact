use std::time::Duration;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    assets::{UiAssets},
    cleanup_system,
    style::AppStyle,
    AppState,
};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::Loading, setup)
        .add_stage_before(
            CoreStage::Update,
            "loading_update",
            FixedTimestepStage::new(Duration::from_secs_f64(0.5)).with_stage( 
                SystemStage::parallel()
                    .with_system(update_text)
            ),
        )
        .add_exit_system(AppState::Loading, cleanup_system);
    }
}

#[derive(Component)]
struct LoadingText;

fn setup(mut commands: Commands, style: Res<AppStyle>, ui_assets: Res<UiAssets>, mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::BLACK;

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect::<Val> {
                    left: Val::Percent(85.0),
                    bottom: Val::Percent(15.0),
                    ..Default::default()
                },
                align_self: AlignSelf::FlexStart,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Left },
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "Loading ".to_string(),
                        style: style.overlay_text(Color::WHITE, &ui_assets),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: style.overlay_text(Color::GOLD, &ui_assets),
                    },
                ],
            },
            ..Default::default()
        })
        .insert(Name::new("ui Loading"))
        .insert(LoadingText);
}

fn update_text(mut query: Query<&mut Text, With<LoadingText>>, mut count: Local<usize>) {
    for mut text in query.iter_mut() {
        // Update the value of the second section
        let str = match *count {
            0 => ".",
            1 => "..",
            _ => "...",
        };
        text.sections[1].value = str.to_string();
        *count += 1;
        *count %= 3;
    }
}
