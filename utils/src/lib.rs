pub mod colors;
pub mod screenspace;

pub mod common_systems {
    use bevy::prelude::*;

    pub fn exit_on_q(kb_input: Res<ButtonInput<KeyCode>>) {
        if kb_input.just_released(KeyCode::KeyQ) {
            std::process::exit(0)
        }
    }
}
