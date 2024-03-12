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

    pub fn red(&self) -> f32 {
        self.red
    }

    pub fn green(&self) -> f32 {
        self.green
    }

    pub fn blue(&self) -> f32 {
        self.blue
    }

    pub fn to_srgb(&self) -> SrgbColour {
        SrgbColour::from_linear(&self)
    }
}
