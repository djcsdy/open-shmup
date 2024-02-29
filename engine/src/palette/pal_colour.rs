use crate::palette::linear_rgb_colour::LinearRgbColour;

pub struct PalColour {
    red: u8,
    green: u8,
    blue: u8,
}

impl PalColour {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn to_linear_rgb(&self) -> LinearRgbColour {
        LinearRgbColour::new(
            ((self.red as f32) / (256.0 - f32::EPSILON)).powf(2.8),
            ((self.green as f32) / (256.0 - f32::EPSILON)).powf(2.8),
            ((self.blue as f32) / (256.0 - f32::EPSILON)).powf(2.8),
        )
    }
}
