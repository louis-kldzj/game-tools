use bevy::prelude::*;

use crate::{colorscheme::ColorScheme, RefreshAllEvent};

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
    pub animate: bool,
}

pub const DEFAULT_OPTIONS: Options = Options {
    pixels: 200.0,
    colorscheme: ColorScheme::FunkyFutures,
    stars: true,
    dust: true,
    nebulae: true,
    planets: true,
    tile: false,
    darken: false,
    transparency: false,
    animate: false,
};

pub fn change_options(
    mut options: ResMut<Options>,
    kb_input: Res<ButtonInput<KeyCode>>,
    mut refresh_all: EventWriter<RefreshAllEvent>,
) {
    if kb_input.just_pressed(KeyCode::KeyC) {
        //TODO: set colorscheme
        options.colorscheme = options.colorscheme.next();
    } else if kb_input.just_pressed(KeyCode::KeyT) {
        options.tile = !options.tile;
    } else if kb_input.just_pressed(KeyCode::KeyD) {
        options.dust = !options.dust;
    } else if kb_input.just_pressed(KeyCode::KeyA) {
        options.transparency = !options.transparency;
    } else if kb_input.just_pressed(KeyCode::KeyS) {
        options.stars = !options.stars;
    } else if kb_input.just_pressed(KeyCode::KeyN) {
        options.nebulae = !options.nebulae;
    } else if kb_input.just_pressed(KeyCode::KeyW) {
        options.darken = !options.darken;
    } else if kb_input.just_pressed(KeyCode::KeyP) {
        options.planets = !options.planets;
    } else if kb_input.just_pressed(KeyCode::KeyM) {
        options.animate = !options.animate;
    } else {
        return;
    }

    refresh_all.send(RefreshAllEvent);
}
