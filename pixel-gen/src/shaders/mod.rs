pub trait AnimatedMaterial2D {
    fn get(&self) -> f32;
    fn update(&mut self);
}
