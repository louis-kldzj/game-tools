use bevy::{prelude::*, sprite::Material2d};

use crate::config::Options;

pub trait AnimatedMaterial2D: Material2d {
    fn get(&self) -> f32;
    fn update(&mut self, new_value: f32);
}

// Could make this a single trait and have everything in the material.

pub trait AnimatedMaterialConfig: Component {
    fn start(&mut self, start: f32, target: f32);

    fn progress(&self) -> f32;
    fn update_progress(&mut self, new_progress: f32);

    fn target(&self) -> f32;

    fn change_direction(&mut self);

    fn cycle(&self) -> bool;

    fn speed(&self) -> f32;
}

#[derive(Component, Default)]
pub struct DefaultAnimationConfig {
    pub progress: f32,
    pub start: f32,
    pub target: f32,
    pub direction: bool,
}

impl AnimatedMaterialConfig for DefaultAnimationConfig {
    fn start(&mut self, start: f32, target: f32) {
        self.start = start;
        self.progress = start;
        self.target = target;
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
        if self.direction {
            self.target
        } else {
            self.start
        }
    }

    fn change_direction(&mut self) {
        self.direction = !self.direction;
    }

    fn cycle(&self) -> bool {
        if self.direction {
            self.progress >= self.target()
        } else {
            self.progress <= self.target()
        }
    }

    fn speed(&self) -> f32 {
        0.01
    }
}

pub fn animate_material<M, C>(
    time: Res<Time>,
    options: Res<Options>,
    mut material: Query<(&mut Handle<M>, &mut C)>,
    mut material_assets: ResMut<Assets<M>>,
) where
    M: AnimatedMaterial2D,
    C: AnimatedMaterialConfig,
{
    if !options.animate {
        return;
    }

    for (handle, mut config) in material.iter_mut() {
        let Some(material) = material_assets.get_mut(handle.id()) else {
            return;
        };

        if config.cycle() {
            config.change_direction();
        }

        material.update(
            material
                .get()
                .lerp(config.target(), time.delta_seconds() * config.speed()),
        );
    }
}
