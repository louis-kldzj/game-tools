use bevy::{prelude::*, window::WindowMode};
use pixel_gen::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .configure_default_pixel_gen()
        .configure_demo_ui()
        .run();
}
