use crate::palette::srgb_colour::SrgbColour;

pub struct LinearRgbColour {
    red: f32,
    green: f32,
    blue: f32,
}

impl LinearRgbColour {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    pub fn to_srgb(&self) -> SrgbColour {
        SrgbColour::new(
            (self.red.clamp(0.0, 1.0).powf(1.0 / 2.2) * (256.0 - f32::EPSILON)) as u8,
            (self.green.clamp(0.0, 1.0).powf(1.0 / 2.2) * (256.0 - f32::EPSILON)) as u8,
            (self.blue.clamp(0.0, 1.0).powf(1.0 / 2.2) * (256.0 - f32::EPSILON)) as u8,
        )
    }
}
