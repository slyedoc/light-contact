use bevy::prelude::*;

use crate::{
    cleanup_system,    
    style::AppStyle,
    AppState,
};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(AppState::AssetLoading)
            .with_system(setup)
        )
        .add_system_set(SystemSet::on_update(AppState::AssetLoading)
            .with_system(update_text)
        )
        .add_system_set(SystemSet::on_exit(AppState::AssetLoading).with_system(cleanup_system));
    }
}

#[derive(Component)]
struct LoadingText;

fn setup(
    mut commands: Commands, 
    style: Res<AppStyle>,
    mut clear_color: ResMut<ClearColor>,
) {
    clear_color.0 = Color::BLACK;

    commands
    .spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect::<Val> {
                right: Val::Px(10.0),
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
                    value: "Loading".to_string(),
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
    .insert(Name::new("ui Loading"))
    .insert(LoadingText);

}

fn update_text(
    mut query: Query<&mut Text, With<LoadingText>>,
    mut count: Local<usize>
) {
    for mut text in query.iter_mut() {
        // Update the value of the second section
        let str = match *count {
            0 => ".",
            1 => "..",
            _ => "..."
        };
        text.sections[1].value = str.to_string();
        *count = *count % 3;
    }
}