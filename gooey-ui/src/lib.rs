use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Component)]
struct ElementId(&'static str);

struct UiText(&'static str, TextStyle);

impl UiText {
    fn to_text(self) -> Text {
        Text::from_section(self.0, self.1)
    }
}

pub struct Config {
    pub id: &'static str,
    pub style: Style,
    pub children: Vec<Element>,
}

pub enum Element {
    Logical(Config),
    Text { config: Config, text: UiText },
    Button { config: Config, text: UiText },
}

impl Element {
    pub fn spawn_as_child(
        self,
        builder: &mut ChildBuilder,
        content_map: HashMap<&'static str, impl Bundle>,
    ) {
        let (mut commands, children) = match self {
            Element::Logical(config) => (
                builder.spawn((
                    NodeBundle {
                        style: config.style,
                        ..default()
                    },
                    ElementId(config.id),
                )),
                config.children,
            ),

            Element::Text { config, text } => (
                builder.spawn((
                    TextBundle {
                        style: config.style,
                        text: text.to_text(),
                        ..default()
                    },
                    ElementId(config.id),
                )),
                config.children,
            ),

            Element::Button { config, text } => {
                let mut children = config.children;
                children.push(Element::Text {
                    config: Config {
                        id: config.id,
                        style: Style::default(),
                        children: vec![],
                    },
                    text,
                });
                (
                    builder.spawn((
                        ButtonBundle {
                            style: config.style,
                            border_color: BorderColor(Color::BLACK),
                            border_radius: BorderRadius::MAX,
                            background_color: Color::WHITE.into(),
                            ..default()
                        },
                        ElementId(config.id),
                    )),
                    children,
                )
            }
        };
    }
}
