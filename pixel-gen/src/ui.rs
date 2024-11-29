use bevy::prelude::*;

use crate::ScreenSize;

#[derive(Event)]
pub struct SpawnMenuEvent;

#[derive(Component)]
pub struct Menu;

pub fn setup(mut writer: EventWriter<SpawnMenuEvent>) {
    writer.send(SpawnMenuEvent);
}

pub fn spawn_menu(
    mut trigger: EventReader<SpawnMenuEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    screen_size: Res<ScreenSize>,
    existing_query: Query<Entity, With<Menu>>,
) {
    let Some(_) = trigger.read().next() else {
        return;
    };
    trigger.clear();

    if let Ok(entity) = existing_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }

    commands
        .spawn((
            Menu,
            NodeBundle {
                style: Style {
                    left: Val::Px(screen_size.left()),
                    width: Val::Px(screen_size.width()),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    builder
                        .spawn(ButtonBundle {
                            style: Style {
                                height: Val::Px(65.),
                                border: UiRect::all(Val::Px(5.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            border_color: BorderColor(Color::BLACK),
                            border_radius: BorderRadius::MAX,
                            background_color: Color::WHITE.into(),
                            ..default()
                        })
                        .with_children(|builder| {
                            builder.spawn(TextBundle::from_section(
                                "NEW IMAGE",
                                default_text_style_with_color(&asset_server, Color::BLACK),
                            ));
                        });
                    builder.spawn(TextBundle::from_section(
                        "BOOO!",
                        default_text_style(&asset_server),
                    ));
                });
        });
}

fn default_text_style(asset_server: &AssetServer) -> TextStyle {
    TextStyle {
        font: asset_server.load("slkscre.ttf"),
        font_size: 48.,
        color: Color::WHITE,
    }
}

fn default_text_style_with_color(asset_server: &AssetServer, color: Color) -> TextStyle {
    TextStyle {
        font: asset_server.load("slkscre.ttf"),
        font_size: 48.,
        color,
    }
}
