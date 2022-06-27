use bevy::{app::AppExit, prelude::*};

use crate::{cleanup_system, enviroment::*, fadeout::Fadeout, style::AppStyle, AppState};

pub struct MainMenuPlugin;

#[derive(Debug, Component, Clone, Copy)]
enum Button {
    Intro,
    Map,
    Exit,
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup)
                .with_system(spawn_light)
                .with_system(spawn_ground),
        )
        .add_system_set(
            SystemSet::on_resume(AppState::MainMenu)
                .with_system(setup)
                .with_system(spawn_light)
                .with_system(spawn_ground),
        )
        .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(button_system))
        .add_system_set(SystemSet::on_pause(AppState::MainMenu).with_system(cleanup_system));
    }
}

fn setup(mut commands: Commands, style: Res<AppStyle>, mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::GRAY;

    // Title Bar
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                position: UiRect {
                    top: Val::Percent(10.0),
                    ..Default::default()
                },
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: style.clear.into(),
            ..Default::default()
        })
        .insert(Name::new("Title Bar"))
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Light Contact",
                        TextStyle {
                            font: style.font.clone(),
                            font_size: 90.0,
                            color: Color::GOLD,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(Name::new("Title"));

            // dev tag
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    color: Color::RED.into(),
                    ..Default::default()
                })
                .insert(Name::new("Dev Tag"))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                margin: UiRect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "from TwinGames",
                                TextStyle {
                                    font: style.font.clone(),
                                    //ui_font.base.clone(),
                                    font_size: style.font_size,
                                    color: Color::GOLD,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(Name::new("Dev"));
                });
        });

    // Menu
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Percent(40.0),
                    left: Val::Percent(30.0),
                    ..Default::default()
                },
                size: Size::new(Val::Percent(40.0), Val::Percent(40.0)),
                flex_direction: FlexDirection::ColumnReverse,
                align_content: AlignContent::Stretch,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: style.clear.into(),
            ..Default::default()
        })
        .insert(Name::new("Menu"))
        .with_children(|parent| {
            create_menu_button(Button::Intro, "Intro", parent, &style);
            create_menu_button(Button::Map, "Map", parent, &style);
            create_menu_button(Button::Exit, "Exit", parent, &style);
        });
}

fn button_system(
    mut interaction_query: Query<(&Interaction, &Button), (Changed<Interaction>, With<Button>)>,
    mut app_exit: EventWriter<AppExit>,
    mut fade_event: EventWriter<Fadeout>,
) {
    for (interaction, btn) in interaction_query.iter_mut() {
        if Interaction::Clicked == *interaction {
            match btn {
                Button::Map => fade_event.send(Fadeout::Push(AppState::Map)),
                Button::Intro => fade_event.send(Fadeout::Push(AppState::Intro)),
                Button::Exit => app_exit.send(AppExit),
            }
        }
    }
}

fn create_menu_button(
    btn: Button,
    text: impl Into<String>,
    parent: &mut ChildBuilder,
    style: &AppStyle,
) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                margin: UiRect::all(Val::Px(10.0)),
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        //.insert(ButtonActive(true))
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    text,
                    TextStyle {
                        font: style.font.clone(),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(btn)
        .insert(Name::new(format!("{:?} Button", btn.clone())));
}
