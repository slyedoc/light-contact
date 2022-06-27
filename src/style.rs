use bevy::prelude::*;

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
    pub font: Handle<Font>,
    pub font_size: f32,
}
impl AppStyle {
    pub fn overlay_text(&self, color: Color) -> TextStyle {
        TextStyle {
            font: self.font.clone(),
            font_size: self.font_size,
            color: color,
        }
    }
}

impl FromWorld for AppStyle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        Self {
            normal_button: Color::rgb(0.15, 0.15, 0.15),
            hovered_button: Color::rgb(0.25, 0.25, 0.25),
            pressed_button: Color::rgb(0.35, 0.75, 0.35),
            normal_button_text: Color::WHITE,
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 30.0,
            clear: Color::rgba(0.0, 0.0, 0.0, 0.0),
        }
    }
}

fn button_color_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut style: Res<AppStyle>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
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