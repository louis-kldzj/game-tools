use bevy::{prelude::*, sprite::Material2dPlugin};

mod nebulae;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<nebulae::NebulaeMaterial>::default(),
        ))
        .add_systems(Startup, nebulae::spawn_nebulae)
        .run();
}

// Setup a simple 2d scene
