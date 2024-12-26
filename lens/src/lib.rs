use bevy::prelude::*;

mod scene;

pub trait Lens {
    fn configure_lens(&mut self) -> &mut Self;
}

impl Lens for App {
    fn configure_lens(&mut self) -> &mut Self {
        self
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera {
        hdr: true,
        ..default()
    });
}
