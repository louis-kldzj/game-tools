use bevy::{
    color::{Color, ColorToComponents, Mix},
    math::Vec4,
    prelude::Image,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

pub enum Easle {
    Parchment,
}

impl Easle {
    pub fn as_color(self) -> Color {
        match self {
            Easle::Parchment => hex_to_color("#c3a38a"),
        }
    }
}

pub fn hex_to_vec4(hex: &str) -> Vec4 {
    match hex_to_srgb(hex) {
        Ok((x, y, z)) => Vec4::new(x, y, z, 1.),
        Err(_) => Vec4::ZERO,
    }
}

pub fn hex_to_color(hex: &str) -> Color {
    match hex_to_srgb(hex) {
        Ok((r, g, b)) => Color::srgb(r, g, b),
        Err(_) => Color::BLACK,
    }
}

fn hex_to_srgb(hex: &str) -> Result<(f32, f32, f32), String> {
    // Remove the leading '#' if present
    let hex = hex.trim_start_matches('#');

    // Ensure the hex string is exactly 6 characters
    if hex.len() != 6 {
        return Err(format!("Invalid hex color code length: {}", hex.len()));
    }

    // Parse the hex string into RGB components
    let r =
        u8::from_str_radix(&hex[0..2], 16).map_err(|e| format!("Invalid red component: {}", e))?;
    let g = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|e| format!("Invalid green component: {}", e))?;
    let b =
        u8::from_str_radix(&hex[4..6], 16).map_err(|e| format!("Invalid blue component: {}", e))?;

    // Convert to normalized sRGB values (0.0 to 1.0)
    let r_srgb = r as f32 / 255.0;
    let g_srgb = g as f32 / 255.0;
    let b_srgb = b as f32 / 255.0;

    Ok((r_srgb, g_srgb, b_srgb))
}

pub fn color_gradiant(hex_codes: &[&str], width: usize) -> Image {
    let mut gradient = Vec::with_capacity(width);
    let num_sections = hex_codes.len() - 1;
    let section_width = width / num_sections;

    for i in 0..num_sections {
        let start_color = hex_to_color(hex_codes[i]);
        let end_color = hex_to_color(hex_codes[i + 1]);

        for j in 0..section_width {
            let t = j as f32 / section_width as f32;
            let color = start_color.mix(&end_color, t);
            gradient.push(color);
        }
    }

    while gradient.len() < width {
        gradient.push(hex_to_color(hex_codes.last().unwrap()));
    }

    let width = gradient.len();
    let mut pixel_data = Vec::with_capacity(width * width * 4);

    for _ in 0..width {
        for color in &gradient {
            let rgba = color.to_srgba().to_vec4();
            pixel_data.extend_from_slice(&[
                (rgba[0] * 255.0) as u8,
                (rgba[1] * 255.0) as u8,
                (rgba[2] * 255.0) as u8,
                (rgba[3] * 255.0) as u8,
            ]);
        }
    }

    let mut image = Image::new(
        Extent3d {
            width: width as u32,
            height: width as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixel_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );

    image.texture_descriptor.usage = bevy::render::render_resource::TextureUsages::TEXTURE_BINDING
        | bevy::render::render_resource::TextureUsages::COPY_DST;

    image
}
