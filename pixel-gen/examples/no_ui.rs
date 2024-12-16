use bevy::{prelude::*, window::WindowMode};
use pixel_gen::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .configure_pixel_gen(OPTIONS)
        .run();
}

const OPTIONS: pixel_gen::Options = Options {
    pixels: 400.,
    colorscheme: ColorScheme::FunkyFutures,
    stars: true,
    dust: true,
    nebulae: true,
    planets: true,
    tile: false,
    darken: false,
    transparency: false,
    animate: false,
    screen_size: ScreenSize {
        screen_space: utils::screenspace::CommonResolution::FourK16x9.space(),
        show_ui: false,
    },
};
