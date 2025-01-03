use std::fs;

use bevy::prelude::*;
use rand::Rng;
use serde::Deserialize;
use toml::Value;
use utils::screenspace::Space;

use crate::{colorscheme::ColorScheme, RefreshAllEvent};

#[derive(Resource, Clone, Copy, Deserialize, PartialEq)]
pub struct Options {
    pub pixels: f32,
    pub colorscheme: ColorScheme,
    pub stars: bool,
    pub dust: bool,
    pub nebulae: bool,
    pub planets: bool,
    pub tile: bool,
    pub darken: bool,
    //NOTE: Currenty does nothing
    pub transparency: bool,
    pub animate: bool,
    pub screen_size: ScreenSize,
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
    screen_size: ScreenSize::new(),
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

pub fn update_screen_size(query: Query<&Window>, mut options: ResMut<Options>) {
    let Ok(window) = query.get_single() else {
        return;
    };

    options.screen_size.set(window.size());
}

#[derive(Default, Clone, Copy, Deserialize, PartialEq)]
pub struct ScreenSize {
    pub screen_space: Space,
    pub show_ui: bool,
}

impl ScreenSize {
    const fn new() -> Self {
        ScreenSize {
            screen_space: Space {
                width: 0.,
                height: 0.,
            },
            show_ui: true,
        }
    }

    pub fn set(&mut self, size: Vec2) {
        self.screen_space.width = size.x;
        self.screen_space.height = size.y;
    }

    pub fn vec2(&self) -> Vec2 {
        Vec2::new(self.screen_space.width, self.screen_space.height)
    }

    pub fn x_offset(&self) -> f32 {
        if self.show_ui {
            -((self.vec2().x / 2.) - (self.vec2().y / 2.))
        } else {
            0.
        }
    }

    pub fn left(&self) -> f32 {
        self.vec2().y
    }

    pub fn width(&self) -> f32 {
        if self.show_ui {
            self.vec2().x - self.left()
        } else {
            self.screen_space.width
        }
    }

    pub fn height(&self) -> f32 {
        self.screen_space.height
    }

    pub fn aspect(&self) -> Vec2 {
        if self.width() > self.height() {
            Vec2::new(self.width() / self.height(), 1.)
        } else {
            Vec2::new(1., self.height() / self.width())
        }
    }

    pub fn random_postion(&self, z: f32) -> Vec3 {
        if self.screen_space.width == 0. && self.screen_space.height == 0. {
            return Vec3::ZERO;
        }
        let half_square = self.screen_space.height / 2.;
        let x_offset = self.x_offset();
        let mut rng = rand::thread_rng();
        let x = rng.gen_range((x_offset - half_square)..(x_offset + half_square));
        let y = rng.gen_range(-half_square..half_square);
        Vec3::new(x, y, z)
    }
}

fn refresh_options_from_file(mut old_options: ResMut<Options>) {
    let Ok(data) = fs::read_to_string("config.toml") else {
        warn!("could not read config file");
        return;
    };

    let Ok(options): Result<Options, _> = toml::from_str(&data) else {
        warn!("could not parse config file");
        return;
    };

    old_options.set_if_neq(options);
}
