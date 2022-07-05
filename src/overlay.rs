use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{style::AppStyle, AppState, Keep};

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup_overlay)
            .add_system(update_game_state)
            .add_system(update_fps);
        //.add_system(update_bvh_tri_count)
        //.add_system(update_render_time)
    }
}

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct TriCountText;

#[derive(Component)]
struct RenderTimeText;

#[derive(Component)]
struct AppStateText;

fn setup_overlay(mut commands: Commands, style: Res<AppStyle>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                position: Rect::<Val> {
                    bottom: Val::Px(100.0),
                    left: Val::Px(10.0),
                    ..Default::default()
                },

                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "State ".to_string(),
                        style: style.overlay_text(Color::WHITE),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: style.overlay_text(Color::GOLD),
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("ui State"))
        .insert(Keep)
        .insert(AppStateText);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect::<Val> {
                    left: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..Default::default()
                },
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: style.overlay_text(Color::WHITE),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: style.overlay_text(Color::GOLD),
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Keep)
        .insert(Name::new("ui FPS"))
        .insert(FpsText);
}

fn update_fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.0}", average);
                text.sections[1].style.color = match average {
                    x if x >= 50.0 => Color::GREEN,
                    x if x > 40.0 && x < 50.0 => Color::YELLOW,
                    x if x <= 40.0 => Color::RED,
                    _ => Color::WHITE,
                };
            }
        }
    }
}

fn update_game_state(mut query: Query<&mut Text, With<AppStateText>>, state: Res<State<AppState>>) {
    for mut text in query.iter_mut() {
        // Update the value of the second section
        text.sections[1].value = format!("{:?}", state.current());
    }
}
