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
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(ButtonBundle::default());
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
