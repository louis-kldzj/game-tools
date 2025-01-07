use bevy::prelude::*;

#[derive(Component)]
struct ElementId(&'static str);

pub struct UiText(pub String);

impl UiText {
    pub fn to_text(self) -> Text {
        Text::new(self.0)
    }
}

pub struct Config {
    pub id: &'static str,
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
            .spawn((Node::default(), component))
            .with_children(|builder| self.spawn_as_child(builder));
    }

    pub fn spawn(self, commands: &mut Commands) {
        commands
            .spawn(Node::default())
            .with_children(|builder| self.spawn_as_child(builder));
    }

    pub fn spawn_as_child(self, builder: &mut ChildBuilder) {
        let (mut commands, children) = match self {
            Element::Logical(config) => (
                builder.spawn((Node::default(), ElementId(config.id))),
                config.children,
            ),

            Element::Text { config, text } => (
                builder.spawn((Text::new(text.0), ElementId(config.id))),
                config.children,
            ),

            Element::Button { config, text } => {
                let mut children = config.children;
                children.push(Element::Text {
                    config: Config {
                        id: config.id,
                        children: vec![],
                    },
                    text,
                });
                (builder.spawn((Button, ElementId(config.id))), children)
            }
        };
        for child in children {
            commands.with_children(|b| child.spawn_as_child(b));
        }
    }
}
