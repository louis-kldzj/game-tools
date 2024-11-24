use bevy::{color::Color, prelude::Image};

pub enum ColorScheme {
    Borkfest,
}

impl ColorScheme {
    pub fn colors(&self) -> [&'static str; 9] {
        match self {
            ColorScheme::Borkfest => [
                "#171711", "#202215", "#3a2802", "#963c3c", "#ca5a2e", "#ff7831", "#f39949",
                "#ebc275", "#dfd785",
            ],
        }
    }

    pub fn gradient_image_with_bg(&self) -> (Image, Color) {
        let colors = self.colors();
        (
            utils::colors::color_gradiant(&colors[1..], 100),
            utils::colors::hex_to_color(colors[1]),
        )
    }
}

pub struct ColorSchemeData {
    colors: &'static [&'static str],
}
