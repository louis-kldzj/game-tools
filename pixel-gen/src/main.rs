use bevy::{
    prelude::*,
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::WindowMode,
};
use nebulae::SpawnNebulaeEvent;
use options::{Options, DEFAULT_OPTIONS};
use planets::SpawnPlanetsEvent;
use star_stuff::SpawnStarStuffEvent;

mod colorscheme;
mod nebulae;
mod options;
mod planets;
mod star_stuff;

#[derive(Resource)]
struct ScreenSize(pub Vec2);

impl ScreenSize {
    pub fn x_offset(&self) -> f32 {
        -((self.0.x / 2.) - (self.0.y / 2.))
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }),
            Material2dPlugin::<nebulae::NebulaeMaterial>::default(),
            Material2dPlugin::<star_stuff::StarStuffMaterial>::default(),
            Material2dPlugin::<planets::PlanetsMaterial>::default(),
        ))
        .add_event::<nebulae::SpawnNebulaeEvent>()
        .add_event::<star_stuff::SpawnStarStuffEvent>()
        .add_event::<planets::SpawnPlanetsEvent>()
        .add_event::<SpawnBackgroundEvent>()
        .insert_resource(ScreenSize(Vec2::ZERO))
        .insert_resource(DEFAULT_OPTIONS)
        .add_systems(Startup, (spawn_camera, options::spawn_debug_text, setup))
        .add_systems(
            PostStartup,
            (nebulae::setup, star_stuff::setup, planets::setup),
        )
        .add_systems(FixedUpdate, update_screen_size)
        .add_systems(
            Update,
            (
                nebulae::spawn_nebulae,
                planets::spawn_planets,
                star_stuff::spawn_star_stuff,
                controls,
                options::change_options,
                utils::common_systems::exit_on_q,
                spawn_bg,
                (planets::update_scale, planets::lerp_scale).chain(),
            ),
        )
        .run();
}

#[derive(Event)]
pub struct SpawnBackgroundEvent;

fn update_screen_size(query: Query<&Window>, mut screen_size: ResMut<ScreenSize>) {
    let Ok(window) = query.get_single() else {
        return;
    };

    screen_size.0 = window.size();
}

fn setup(
    mut writer: EventWriter<SpawnBackgroundEvent>,
    mut screen_size: ResMut<ScreenSize>,
    window: Query<&Window>,
) {
    writer.send(SpawnBackgroundEvent);

    let Ok(window) = window.get_single() else {
        return;
    };

    screen_size.0 = window.size();
}

fn spawn_bg(
    mut reader: EventReader<SpawnBackgroundEvent>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut screen_size: ResMut<ScreenSize>,
    window: Query<&Window>,
    options: Res<Options>,
) {
    let Some(_) = reader.read().next() else {
        return;
    };
    reader.clear();

    let Ok(window) = window.get_single() else {
        return;
    };

    let size = window.size();
    screen_size.0 = size;

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(size.x, size.y)).into(),
        transform: Transform::from_xyz(0., 0.0, -1.0),
        material: materials.add(ColorMaterial::from_color(
            utils::colors::hex_to_color(options.colorscheme.colors().first().unwrap())
                .with_alpha(if options.transparency { 0.0 } else { 1.0 }),
        )),
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
    mut spawn_planets: EventWriter<SpawnPlanetsEvent>,
    mut spawn_bg: EventWriter<SpawnBackgroundEvent>,
) {
    if !kb_input.just_released(KeyCode::Space) {
        return;
    }

    spawn_nebulae.send(SpawnNebulaeEvent);
    spawn_star_stuff.send(SpawnStarStuffEvent);
    spawn_planets.send(SpawnPlanetsEvent);
    spawn_bg.send(SpawnBackgroundEvent);
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
