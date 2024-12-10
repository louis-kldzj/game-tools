mod background;
mod colorscheme;
mod config;
mod controls;
mod nebulae;
mod planets;
mod shaders;
mod star_stuff;
mod stars;
mod ui;

pub(crate) use bevy::{
    prelude::*,
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

#[derive(Event)]
struct RefreshAllEvent;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub trait PixelSpace {
    fn configure_pixel_gen(&mut self) -> &mut Self;
}

impl PixelSpace for App {
    fn configure_pixel_gen(&mut self) -> &mut App {
        self.add_plugins((
            Material2dPlugin::<nebulae::NebulaeMaterial>::default(),
            Material2dPlugin::<star_stuff::StarStuffMaterial>::default(),
            Material2dPlugin::<planets::PlanetsMaterial>::default(),
            Material2dPlugin::<stars::BigStarMaterial>::default(),
        ))
        .add_event::<nebulae::SpawnNebulaeEvent>()
        .add_event::<star_stuff::SpawnStarStuffEvent>()
        .add_event::<planets::SpawnPlanetsEvent>()
        .add_event::<stars::SpawnBigStarEvent>()
        .add_event::<background::SpawnBackgroundEvent>()
        .add_event::<ui::SpawnMenuEvent>()
        .add_event::<RefreshAllEvent>()
        .insert_resource(config::ScreenSize::default())
        .insert_resource(config::DEFAULT_OPTIONS)
        .insert_resource(nebulae::NebulaeConfig::new())
        .insert_resource(star_stuff::StarStuffConfig::new())
        .add_systems(Startup, (spawn_camera, background::setup, ui::setup))
        .add_systems(
            PostStartup,
            (nebulae::setup, star_stuff::setup, planets::setup),
        )
        .add_systems(FixedUpdate, config::update_screen_size)
        .add_systems(
            Update,
            (
                nebulae::spawn_nebulae,
                planets::spawn_planets,
                star_stuff::spawn_star_stuff,
                controls::controls,
                config::change_options,
                utils::common_systems::exit_on_q,
                background::spawn,
                (planets::update_scale, planets::lerp_scale).chain(),
                ui::spawn_menu,
                stars::spawn_star,
                shaders::animate_material::<nebulae::NebulaeMaterial, nebulae::NebulaeConfig>,
                shaders::animate_material::<
                    star_stuff::StarStuffMaterial,
                    star_stuff::StarStuffConfig,
                >,
            ),
        )
    }
}
