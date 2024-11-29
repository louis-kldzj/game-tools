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
    colorscheme: ColorScheme::FunkyFutures,
    stars: true,
    dust: true,
    nebulae: true,
    planets: true,
    tile: false,
    darken: false,
    transparency: false,
};

pub fn change_options(
    mut options: ResMut<Options>,
    mut text: Query<&mut Text>,
    kb_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
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
    } else {
        return;
    }
    let Ok(mut text) = text.get_single_mut() else {
        return;
    };

    text.sections = vec![TextSection::new(
        format!(
            "TILE: {}\nDUST: {}\nALPHA: {}\nSTARS: {}\nNEB: {}\nDARK: {}\nCOLORSCHEME: {:?}\nPLANETS: {}",
            options.tile,
            options.dust,
            options.transparency,
            options.stars,
            options.nebulae,
            options.darken,
            options.colorscheme,
            options.planets
        ),
        TextStyle {
            font: asset_server.load("slkscre.ttf"),
            font_size: 48.,
            color: Color::WHITE,
        },
    )];
}

pub fn spawn_debug_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(TextBundle {
        text: Text::from_section(
            "DEBUG: ",
            TextStyle {
                font: asset_server.load("slkscre.ttf"),
                font_size: 48.,
                color: Color::WHITE,
            },
        ),
        ..default()
    });
}
