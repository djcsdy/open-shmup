use crate::palette::linear_rgb_colour::LinearRgbColour;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct SrgbColour {
    red: u8,
    green: u8,
    blue: u8,
}

impl SrgbColour {
    pub const BLACK: Self = Self {
        red: 0,
        green: 0,
        blue: 0,
    };

    pub fn from_linear(colour: &LinearRgbColour) -> Self {
        Self {
            red: Self::component_from_linear(colour.red()),
            green: Self::component_from_linear(colour.green()),
            blue: Self::component_from_linear(colour.blue()),
        }
    }

    fn component_from_linear(component: f32) -> u8 {
        (if component < 0.03928 / 12.92321 {
            component * 12.92321
        } else {
            component.powf(1.0 / 2.4) * (1.055) - 0.055
        } * (256.0 - f32::EPSILON)) as u8
    }

    pub fn red(&self) -> u8 {
        self.red
    }

    pub fn green(&self) -> u8 {
        self.green
    }

    pub fn blue(&self) -> u8 {
        self.blue
    }

    pub fn css(&self) -> String {
        String::from_iter([
            "rgb(",
            &self.red.to_string(),
            " ",
            &self.green.to_string(),
            " ",
            &self.blue.to_string(),
            ")",
        ])
    }
}
