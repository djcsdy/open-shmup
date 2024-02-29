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

    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
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
