use crate::palette::Colour;
use std::array;
use std::f32::consts::PI;
use std::ops::Index;

#[derive(PartialEq, Clone, Debug)]
pub struct Palette<const S: usize>([Colour; S]);

impl Palette<16> {
    /// Generates a Colodore Palette as documented at https://www.pepto.de/projects/colorvic/
    pub fn new_colodore() -> Self {
        let brightness = 0.5;
        let contrast = 1.0;

        let luma = [
            0.0, 1.0, 0.3125, 0.625, 0.375, 0.5, 0.25, 0.75, 0.375, 0.25, 0.5, 0.3125, 0.46875,
            0.75, 0.46875, 0.625,
        ];
        let chroma = [
            0.0, 0.0, 4.0, 12.0, 2.0, 10.0, 15.0, 7.0, 5.0, 6.0, 4.0, 0.0, 0.0, 10.0, 15.0, 0.0,
        ];
        let saturation = [
            0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.0,
        ];

        let sector = PI / 8.0;
        let origin = sector / 2.0;
        let screen = 0.2;

        let mut palette = [Colour::BLACK; 16];

        for i in 0..palette.len() {
            let angle = origin + chroma[i] * sector;
            let y = (luma[i] + 0.390625 * (brightness - 0.5)) * (contrast + screen);
            let u = angle.cos() * saturation[i] * 0.390625 * (1.0 - screen) * (contrast + screen);
            let v = angle.sin() * saturation[i] * 0.390625 * (1.0 - screen) * (contrast + screen);

            palette[i] = Colour::new(
                Self::pal_to_linear((y + 1.140 * v).clamp(0.0, 1.0)),
                Self::pal_to_linear((y - 0.396 * u - 0.581 * v).clamp(0.0, 1.0)),
                Self::pal_to_linear((y + 2.029 * u).clamp(0.0, 1.0)),
            );
        }

        Self(palette)
    }

    pub fn new_tile_subpalettes(&self, tile_palette_data: &[u8; 3]) -> [Palette<4>; 8] {
        array::from_fn(|index| {
            Palette([
                self[tile_palette_data[0] as usize],
                self[tile_palette_data[1] as usize],
                self[tile_palette_data[2] as usize],
                self[index],
            ])
        })
    }

    fn pal_to_linear(value: f32) -> f32 {
        value.powf(2.8)
    }
}

impl<const S: usize> Index<usize> for Palette<S> {
    type Output = Colour;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}