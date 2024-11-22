use bevy::{prelude::*, sprite::Material2dPlugin};

mod nebulae;
mod star_stuff;

const TEXTURE_SIZE: f32 = 256.;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<nebulae::NebulaeMaterial>::default(),
            Material2dPlugin::<star_stuff::StarStuffMaterial>::default(),
        ))
        .add_systems(
            Startup,
            (
                spawn_camera,
                nebulae::spawn_nebulae,
                star_stuff::spawn_star_stuff,
            ),
        )
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Setup a simple 2d scene
