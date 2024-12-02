use bevy::prelude::*;

#[derive(Component)]
struct ElementId(&'static str);

pub struct UiText(pub &'static str, pub TextStyle);

impl UiText {
    pub fn to_text(self) -> Text {
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
    pub fn spawn_with<C>(self, commands: &mut Commands, component: C)
    where
        C: Component,
    {
        commands
            .spawn((NodeBundle::default(), component))
            .with_children(|builder| self.spawn_as_child(builder));
    }

    pub fn spawn(self, commands: &mut Commands) {
        commands
            .spawn(NodeBundle::default())
            .with_children(|builder| self.spawn_as_child(builder));
    }

    pub fn spawn_as_child(self, builder: &mut ChildBuilder) {
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
                            background_color: utils::Easle::Parchment.as_color().into(),
                            ..default()
                        },
                        ElementId(config.id),
                    )),
                    children,
                )
            }
        };
        for child in children {
            commands.with_children(|b| child.spawn_as_child(b));
        }
    }
}
