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

pub use colorscheme::ColorScheme;
pub use config::{Options, ScreenSize};

#[derive(Event)]
struct RefreshAllEvent;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub trait PixelSpace {
    fn configure_default_pixel_gen(&mut self) -> &mut Self;
    fn configure_pixel_gen(&mut self, options: Options) -> &mut Self;
    fn configure_demo_ui(&mut self) -> &mut Self;
}

impl PixelSpace for App {
    fn configure_default_pixel_gen(&mut self) -> &mut Self {
        self.configure_pixel_gen(config::DEFAULT_OPTIONS)
    }

    fn configure_demo_ui(&mut self) -> &mut Self {
        self.add_event::<ui::SpawnMenuEvent>()
            .add_systems(Startup, ui::setup)
            .add_systems(Update, (ui::spawn_menu, ui::refresh))
    }

    fn configure_pixel_gen(&mut self, options: Options) -> &mut App {
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
        .add_event::<RefreshAllEvent>()
        .insert_resource(options)
        .add_systems(Startup, (spawn_camera, background::setup))
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
                stars::spawn_star,
                shaders::animate_material::<nebulae::NebulaeMaterial, nebulae::NebulaeConfig>,
                shaders::animate_material::<
                    star_stuff::StarStuffMaterial,
                    star_stuff::StarStuffConfig,
                >,
                shaders::animate_material::<planets::PlanetsMaterial, planets::PlanetsConfig>,
            ),
        )
    }
}
