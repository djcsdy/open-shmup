#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Colour {
    red: f32,
    green: f32,
    blue: f32,
}

impl Colour {
    pub const BLACK: Self = Self {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };

    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self {
            red: red.clamp(0.0, 1.0),
            green: green.clamp(0.0, 1.0),
            blue: blue.clamp(0.0, 1.0),
        }
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

    pub fn css(&self) -> String {
        String::from_iter([
            "color(srgb-linear ",
            &self.red.to_string(),
            " ",
            &self.green.to_string(),
            " ",
            &self.blue.to_string(),
            ")",
        ])
    }
}
