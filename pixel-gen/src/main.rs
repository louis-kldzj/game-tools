use bevy::{
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
    window::WindowMode,
};
use nebulae::SpawnNebulaeEvent;
use options::{Options, DEFAULT_OPTIONS};
use planets::SpawnPlanetsEvent;
use rand::Rng;
use star_stuff::SpawnStarStuffEvent;
use ui::SpawnMenuEvent;

mod colorscheme;
mod nebulae;
mod options;
mod planets;
mod star_stuff;
mod ui;

#[derive(Resource)]
struct ScreenSize(pub Vec2);

const DEFAULT_SCREEN_SIZE: Vec2 = Vec2 { x: 3840., y: 2160. };

impl ScreenSize {
    pub fn x_offset(&self) -> f32 {
        -((self.0.x / 2.) - (self.0.y / 2.))
    }

    pub fn left(&self) -> f32 {
        self.0.y
    }

    pub fn width(&self) -> f32 {
        self.0.x - self.left()
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
        .add_event::<ui::SpawnMenuEvent>()
        .add_event::<RefreshAllEvent>()
        .insert_resource(ScreenSize(DEFAULT_SCREEN_SIZE))
        .insert_resource(DEFAULT_OPTIONS)
        .add_systems(
            Startup,
            (spawn_camera, options::spawn_debug_text, setup, ui::setup),
        )
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
                ui::spawn_menu,
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

fn setup(mut writer: EventWriter<SpawnBackgroundEvent>) {
    writer.send(SpawnBackgroundEvent);
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

#[derive(Event)]
struct RefreshAllEvent;

fn controls(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut refresh_event: EventReader<RefreshAllEvent>,
    mut spawn_nebulae: EventWriter<SpawnNebulaeEvent>,
    mut spawn_star_stuff: EventWriter<SpawnStarStuffEvent>,
    mut spawn_planets: EventWriter<SpawnPlanetsEvent>,
    mut spawn_bg: EventWriter<SpawnBackgroundEvent>,
    mut spawn_menu: EventWriter<SpawnMenuEvent>,
) {
    if !kb_input.just_released(KeyCode::Space) {
        let Some(_) = refresh_event.read().next() else {
            return;
        };
        refresh_event.clear();
    }

    spawn_nebulae.send(SpawnNebulaeEvent);
    spawn_star_stuff.send(SpawnStarStuffEvent);
    for _ in 0..rand::thread_rng().gen_range(1..5) {
        spawn_planets.send(SpawnPlanetsEvent);
    }
    spawn_bg.send(SpawnBackgroundEvent);
    spawn_menu.send(SpawnMenuEvent);
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
