use bevy::{
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
};
use nebulae::SpawnNebulaeEvent;
use star_stuff::SpawnStarStuffEvent;

mod nebulae;
mod star_stuff;

const TEXTURE_SIZE: f32 = 256.;

#[derive(Resource)]
struct BGColorIndex(usize);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<nebulae::NebulaeMaterial>::default(),
            Material2dPlugin::<star_stuff::StarStuffMaterial>::default(),
        ))
        .add_event::<nebulae::SpawnNebulaeEvent>()
        .add_event::<star_stuff::SpawnStarStuffEvent>()
        .insert_resource(BGColorIndex(0))
        .add_systems(
            Startup,
            (spawn_camera, nebulae::setup, star_stuff::setup, spawn_bg),
        )
        .add_systems(
            Update,
            (
                nebulae::spawn_nebulae,
                star_stuff::spawn_star_stuff,
                controls,
            ),
        )
        .run();
}

fn spawn_bg(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window: Query<&Window>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };

    let size = window.size();

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(size.y, size.y)).into(),
        transform: Transform::from_xyz(0., 0.0, -1.0),
        material: materials.add(ColorMaterial::from_color(Color::BLACK)),
        ..default()
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn controls(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut spawn_nebulae: EventWriter<SpawnNebulaeEvent>,
    mut spawn_star_stuff: EventWriter<SpawnStarStuffEvent>,
) {
    if !kb_input.just_released(KeyCode::Space) {
        return;
    }

    spawn_nebulae.send(SpawnNebulaeEvent);
    spawn_star_stuff.send(SpawnStarStuffEvent);
}
