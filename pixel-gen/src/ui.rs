use bevy::prelude::*;
use gooey_ui::{Element, UiText};

use crate::{options::Options, ScreenSize};

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
    options: Res<Options>,
) {
    let Some(_) = trigger.read().next() else {
        return;
    };
    trigger.clear();

    if let Ok(entity) = existing_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }

    menu(&screen_size, &asset_server, &options).spawn_with(&mut commands, Menu);
}

fn menu(
    screen_size: &ScreenSize,
    asset_server: &AssetServer,
    options: &Options,
) -> gooey_ui::Element {
    Element::Logical(gooey_ui::Config {
        id: "ROOT",
        style: Style {
            left: Val::Px(screen_size.space.height),
            width: Val::Px(screen_size.space.width - screen_size.space.height),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children: vec![
            Element::Button {
                text: ui_text_for_button("NEW IMAGE", asset_server, options.colorscheme.bg_color()),
                config: gooey_ui::Config {
                    id: "NEW-IMAGE",
                    style: Style {
                        height: Val::Px(65.),
                        border: UiRect::all(Val::Px(5.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    children: vec![],
                },
            },
            Element::Text {
                text: ui_text("SIZE (PIXELS):", asset_server),
                config: gooey_ui::Config {
                    id: "LABEL-1",
                    style: Style::default(),
                    children: vec![],
                },
            },
            Element::Text {
                text: ui_text("WIDTH:", asset_server),
                config: gooey_ui::Config {
                    id: "LABEL-WIDTH",
                    style: Style::default(),
                    children: vec![],
                },
            },
            Element::Text {
                text: ui_text("HEIGHT:", asset_server),
                config: gooey_ui::Config {
                    id: "LABEL-HEIGHT",
                    style: Style::default(),
                    children: vec![],
                },
            },
            Element::Text {
                text: ui_text("COLORSCHEME:", asset_server),
                config: gooey_ui::Config {
                    id: "LABEL-CS",
                    style: Style::default(),
                    children: vec![],
                },
            },
            Element::Button {
                text: ui_text_for_button("STARS:", asset_server, options.colorscheme.bg_color()),
                config: gooey_ui::Config {
                    id: "BTN-STARS",
                    style: button_style(),
                    children: vec![],
                },
            },
            Element::Button {
                text: ui_text_for_button("DUST:", asset_server, options.colorscheme.bg_color()),
                config: gooey_ui::Config {
                    id: "BTN-DUST",
                    style: button_style(),
                    children: vec![],
                },
            },
            Element::Button {
                text: ui_text_for_button("NEBULAE:", asset_server, options.colorscheme.bg_color()),
                config: gooey_ui::Config {
                    id: "BTN-NEBULAE",
                    style: button_style(),
                    children: vec![],
                },
            },
            Element::Button {
                text: ui_text_for_button("PLANETS:", asset_server, options.colorscheme.bg_color()),
                config: gooey_ui::Config {
                    id: "BTN-PLANETS",
                    style: button_style(),
                    children: vec![],
                },
            },
            Element::Button {
                text: ui_text_for_button("TILE:", asset_server, options.colorscheme.bg_color()),
                config: gooey_ui::Config {
                    id: "BTN-TILE",
                    style: button_style(),
                    children: vec![],
                },
            },
            Element::Button {
                text: ui_text_for_button("DARKEN:", asset_server, options.colorscheme.bg_color()),
                config: gooey_ui::Config {
                    id: "BTN-DARKEN",
                    style: button_style(),
                    children: vec![],
                },
            },
            Element::Button {
                text: ui_text_for_button(
                    "TRANSPARENCY:",
                    asset_server,
                    options.colorscheme.bg_color(),
                ),
                config: gooey_ui::Config {
                    id: "BTN-TRANSPARENCY",
                    style: button_style(),
                    children: vec![],
                },
            },
        ],
    })
}

fn button_style() -> Style {
    Style {
        height: Val::Px(65.),
        border: UiRect::all(Val::Px(5.)),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }
}

fn ui_text(text: &'static str, asset_server: &AssetServer) -> UiText {
    UiText(
        text,
        default_text_style_with_color(asset_server, utils::Easle::Parchment.as_color()),
    )
}

fn ui_text_for_button(text: &'static str, asset_server: &AssetServer, color: Color) -> UiText {
    UiText(text, default_text_style_with_color(asset_server, color))
}

fn default_text_style(asset_server: &AssetServer) -> TextStyle {
    TextStyle {
        font: asset_server.load("slkscre.ttf"),
        font_size: 48.,
        color: Color::BLACK,
    }
}

fn default_text_style_with_color(asset_server: &AssetServer, color: Color) -> TextStyle {
    TextStyle {
        font: asset_server.load("slkscre.ttf"),
        font_size: 48.,
        color,
    }
}
