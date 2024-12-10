use bevy::{prelude::*, window::WindowMode};
use pixel_gen::*;

fn main() {
    crate::configure(App::new().add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        }),
        ..default()
    })))
    .run();
}

// TODO: Think about this
/*
#[derive(SystemParam)]
struct MeshSpawnerParams<'w, 's, E, M, C>
where
    E: Event,
    M: Material2d,
    C: Component,
{
    trigger: EventReader<'w, 's, E>,
    commands: Commands<'w, 's>,
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<M>>,
    images: ResMut<'w, Assets<Image>>,
    options: Res<'w, Options>,
    query: Query<'w, 's, Entity, With<C>>,
}

fn spawn_element<E, M, C>(mut params: MeshSpawnerParams<E, M, C>)
where
    E: Event,
    M: Material2d,
    C: Component,
{
    let Some(_) = params.trigger.read().next() else {
        return;
    };
    params.trigger.clear();

    if let Ok(entity) = params.query.get_single() {
        params.commands.entity(entity).despawn_recursive();
    }
}
*/
