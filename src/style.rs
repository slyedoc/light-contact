use bevy::prelude::*;

use crate::assets::UiAssets;

pub struct StylePlugin;

impl Plugin for StylePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppStyle>()
            .add_system(button_color_system);
    }
}

pub struct AppStyle {
    pub normal_button: Color,
    pub normal_button_text: Color,
    pub hovered_button: Color,
    pub pressed_button: Color,
    pub clear: Color,
    pub font_size: f32,
}

impl Default for AppStyle {
    fn default() -> Self {
        Self {
            normal_button: Color::rgb(0.15, 0.15, 0.15),
            hovered_button: Color::rgb(0.25, 0.25, 0.25),
            pressed_button: Color::rgb(0.35, 0.75, 0.35),
            normal_button_text: Color::WHITE,
            font_size: 30.0,
            clear: Color::rgba(0.0, 0.0, 0.0, 0.0),
        }
    }
}

impl AppStyle {
    pub fn overlay_text(&self, color: Color, ui_assets: &UiAssets) -> TextStyle {
        TextStyle {
            font: ui_assets.font.clone(),
            font_size: self.font_size,
            color,
        }
    }
}

fn button_color_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,

    style: Res<AppStyle>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = style.pressed_button.into();
            }
            Interaction::Hovered => {
                *color = style.hovered_button.into();
            }
            Interaction::None => {
                *color = style.normal_button.into();
            }
        }
    }
}
