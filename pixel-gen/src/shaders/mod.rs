use bevy::{math::VectorSpace, prelude::*, sprite::Material2d};

use crate::{config::Options, nebulae::Nebulae};

pub trait AnimatedMaterial2D: Material2d {
    fn get(&self) -> f32;
    fn update(&mut self, new_value: f32);
}

// Could make this a single trait and have everything in the material.

pub trait AnimatedMaterialConfig: Resource {
    fn start(&mut self, value: f32);

    fn progress(&self) -> f32;
    fn update_progress(&mut self, new_progress: f32);

    fn target(&self) -> f32;

    fn change_direction(&mut self);

    fn speed(&self) -> f32;
}

#[derive(Resource, Default)]
pub struct DefaultAnimationConfig {
    pub progress: f32,
    pub start: f32,
    pub target: f32,
    pub direction: bool,
}

impl AnimatedMaterialConfig for DefaultAnimationConfig {
    fn start(&mut self, value: f32) {
        self.start = value;
        self.direction = true;
        self.change_direction();
    }

    fn progress(&self) -> f32 {
        self.progress
    }

    fn update_progress(&mut self, new_progress: f32) {
        self.progress = new_progress;
    }

    fn target(&self) -> f32 {
        self.target
    }

    fn change_direction(&mut self) {
        if self.direction {
            self.target = self.start + 10.;
        } else {
            self.target = self.start
        }
        self.direction = !self.direction;
    }

    fn speed(&self) -> f32 {
        0.01
    }
}

pub fn animate_material<M, R>(
    time: Res<Time>,
    options: Res<Options>,
    material: Query<&mut Handle<M>>,
    mut material_assets: ResMut<Assets<M>>,
    mut config: ResMut<R>,
) where
    M: AnimatedMaterial2D,
    R: AnimatedMaterialConfig,
{
    if !options.animate {
        return;
    }

    let Ok(handle) = material.get_single() else {
        return;
    };

    let Some(material) = material_assets.get_mut(handle.id()) else {
        return;
    };

    if config.progress() >= config.target() {
        config.update_progress(0.);
        config.change_direction();
    }

    material.update(
        material
            .get()
            .lerp(config.target(), time.delta_seconds() * config.speed()),
    );
}
