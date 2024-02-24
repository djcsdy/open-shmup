use crate::colour::Colour;
use std::f32::consts::PI;
use std::ops::Index;
use std::slice::Iter;

#[derive(PartialEq, Clone, Debug)]
pub struct Palette<const S: usize>([Colour; S]);

impl<const S: usize> Palette<S> {
    pub fn new(colours: [Colour; S]) -> Self {
        Self(colours)
    }

    pub fn len(&self) -> usize {
        S
    }

    pub fn iter(&self) -> Iter<'_, Colour> {
        self.0.iter()
    }
}

impl Palette<16> {
    /// Generates a Colodore Palette as documented at https://www.pepto.de/projects/colorvic/
    pub fn new_colodore() -> Self {
        let brightness = 0.5;
        let contrast = 1.0;
        let saturation = 0.5;

        let luma = [
            0.0, 1.0, 0.3125, 0.625, 0.375, 0.5, 0.25, 0.75, 0.375, 0.25, 0.5, 0.3125, 0.46875,
            0.75, 0.46875, 0.625,
        ];
        let chroma = [
            0.0, 0.0, 4.0, 12.0, 2.0, 10.0, 15.0, 7.0, 5.0, 6.0, 4.0, 0.0, 0.0, 10.0, 15.0, 0.0,
        ];

        let sector = PI / 8.0;
        let origin = sector / 2.0;
        let screen = 0.2;

        let source_gamma = 2.8; // PAL
        let target_gamma = 2.2; // sRGB

        let mut palette = [Colour {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }; 16];

        for i in 0..palette.len() {
            let angle = origin + chroma[i] * sector;
            let y = (luma[i] + 0.390625 * (brightness - 0.5)) * (contrast + screen);
            let u = angle.cos() * saturation * 0.390625 * (1.0 - screen) * (contrast + screen);
            let v = angle.sin() * saturation * 0.390625 * (1.0 - screen) * (contrast + screen);

            palette[i].red =
                Self::gamma_correct((y + 1.140 * v).clamp(0.0, 1.0), source_gamma, target_gamma);

            palette[i].green = Self::gamma_correct(
                (y - 0.396 * u - 0.581 * v).clamp(0.0, 1.0),
                source_gamma,
                target_gamma,
            );

            palette[i].blue =
                Self::gamma_correct((y + 2.029 * u).clamp(0.0, 1.0), source_gamma, target_gamma);
        }

        Self(palette)
    }

    fn gamma_correct(value: f32, source_gamma: f32, target_gamma: f32) -> f32 {
        value.powf(source_gamma).powf(1.0 / target_gamma)
    }
}

impl<const S: usize> Index<usize> for Palette<S> {
    type Output = Colour;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
