use bevy::prelude::*;

use crate::colorscheme::ColorScheme;

#[derive(Resource)]
pub struct Options {
    pub pixels: f32,
    pub colorscheme: ColorScheme,
    pub stars: bool,
    pub dust: bool,
    pub nebulae: bool,
    pub planets: bool,
    pub tile: bool,
    pub darken: bool,
    pub transparency: bool,
}

pub const DEFAULT_OPTIONS: Options = Options {
    pixels: 200.0,
    colorscheme: ColorScheme::Borkfest,
    stars: false,
    dust: true,
    nebulae: true,
    planets: false,
    tile: true,
    darken: true,
    transparency: false,
};
