use bevy::prelude::*;
use rand::Rng;
use utils::screenspace::Space;

use crate::{colorscheme::ColorScheme, RefreshAllEvent};

#[derive(Resource, Clone, Copy)]
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

#[derive(Default, Clone, Copy)]
pub struct ScreenSize {
    pub space: Space,
}

impl ScreenSize {
    const fn new() -> Self {
        ScreenSize {
            space: Space {
                width: 0.,
                height: 0.,
            },
        }
    }

    pub fn set(&mut self, size: Vec2) {
        self.space.width = size.x;
        self.space.height = size.y;
    }

    pub fn vec2(&self) -> Vec2 {
        Vec2::new(self.space.width, self.space.height)
    }

    pub fn x_offset(&self) -> f32 {
        -((self.vec2().x / 2.) - (self.vec2().y / 2.))
    }

    pub fn left(&self) -> f32 {
        self.vec2().y
    }

    pub fn width(&self) -> f32 {
        self.vec2().x - self.left()
    }

    pub fn random_postion(&self, z: f32) -> Vec3 {
        if self.space.width == 0. && self.space.height == 0. {
            return Vec3::ZERO;
        }
        let half_square = self.space.height / 2.;
        let x_offset = self.x_offset();
        let mut rng = rand::thread_rng();
        let x = rng.gen_range((x_offset - half_square)..(x_offset + half_square));
        let y = rng.gen_range(-half_square..half_square);
        Vec3::new(x, y, z)
    }
}
