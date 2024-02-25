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
        let max = 256.0 - f32::EPSILON;
        String::from_iter([
            "rgb(",
            &(self.red * max).floor().to_string(),
            " ",
            &(self.green * max).floor().to_string(),
            " ",
            &(self.blue * max).floor().to_string(),
            ")",
        ])
    }
}
