use bevy::{color::Color, prelude::Image};

#[derive(Debug)]
pub enum ColorScheme {
    Borkfest,
    Nyx8,
    Ammo8,
    FunkyFutures,
}

impl ColorScheme {
    pub fn next(&self) -> Self {
        match self {
            ColorScheme::Borkfest => ColorScheme::Nyx8,
            ColorScheme::Nyx8 => ColorScheme::Ammo8,
            ColorScheme::Ammo8 => ColorScheme::FunkyFutures,
            ColorScheme::FunkyFutures => ColorScheme::Borkfest,
        }
    }

    pub fn colors(&self) -> [&'static str; 9] {
        match self {
            ColorScheme::Borkfest => [
                "#171711", "#202215", "#3a2802", "#963c3c", "#ca5a2e", "#ff7831", "#f39949",
                "#ebc275", "#dfd785",
            ],
            ColorScheme::Nyx8 => [
                "#01090f", "#08141e", "#0f2a3f", "#20394f", "#4e495f", "#816271", "#997577",
                "#c3a38a", "#f6d6bd",
            ],
            ColorScheme::Ammo8 => [
                "#000a03", "#040c06", "#112318", "#1e3a29", "#305d42", "#4d8061", "#89a257",
                "#bedc7f", "#eeffcc",
            ],
            ColorScheme::FunkyFutures => [
                "#120826", "#2b0f54", "#ab1f65", "#ff4f69", "#ff8142", "#ffda45", "#3368dc",
                "#49e7ec", "#fff7f8",
            ],
        }
    }

    pub fn bg_color(&self) -> Color {
        utils::colors::hex_to_color(self.colors()[1])
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
