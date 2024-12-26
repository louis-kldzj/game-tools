use bevy::prelude::*;
use lens::Lens;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .configure_lens()
        .run();
}
