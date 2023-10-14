use bevy::{
    prelude::*,
    window::{PresentMode, WindowPlugin},
    winit::WinitSettings,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|root| {
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(80.0),
                    height: Val::Percent(80.0),
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            max_height: Val::Percent(20.0),
                            flex_grow: 1.0,
                            border: UiRect::all(Val::Percent(1.0)),
                            margin: UiRect::all(Val::Percent(1.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Button",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                    });

                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            max_height: Val::Percent(20.0),
                            flex_grow: 1.0,
                            border: UiRect::all(Val::Percent(1.0)),
                            margin: UiRect::all(Val::Percent(1.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Button",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                    });

                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            max_height: Val::Percent(20.0),
                            flex_grow: 1.0,
                            border: UiRect::all(Val::Percent(1.0)),
                            margin: UiRect::all(Val::Percent(1.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Button",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                    });
            });
        });
}
