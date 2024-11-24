use bevy::{
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
};
use menu::DEFAULT_OPTIONS;
use nebulae::SpawnNebulaeEvent;
use star_stuff::SpawnStarStuffEvent;
use utils::colors::{hex_to_color, hex_to_vec4};

mod colorscheme;
mod menu;
mod nebulae;
mod star_stuff;

const BACKGROUND_COLOR: &str = "#04183c";

#[derive(Resource)]
struct BGColorIndex(usize);

#[derive(Resource)]
struct ScreenSize(Vec2);

fn background_color() -> Color {
    hex_to_color(BACKGROUND_COLOR)
}

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
        .insert_resource(ScreenSize(Vec2::ZERO))
        .insert_resource(DEFAULT_OPTIONS)
        .add_systems(Startup, (spawn_camera, spawn_bg))
        .add_systems(PostStartup, (nebulae::setup, star_stuff::setup))
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
    mut screen_size: ResMut<ScreenSize>,
    window: Query<&Window>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };

    let size = window.size();
    screen_size.0 = size;

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(size.x, size.y)).into(),
        transform: Transform::from_xyz(0., 0.0, -1.0),
        material: materials.add(ColorMaterial::from_color(background_color())),
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
