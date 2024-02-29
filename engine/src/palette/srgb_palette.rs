use crate::palette::pal_colour::PalColour;
use crate::palette::SrgbColour;
use std::array;
use std::f32::consts::PI;
use std::ops::Index;

#[derive(PartialEq, Clone, Debug)]
pub struct SrgbPalette<const S: usize>([SrgbColour; S]);

impl SrgbPalette<16> {
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

        let mut palette = [SrgbColour::BLACK; 16];

        for i in 0..palette.len() {
            let angle = origin + chroma[i] * sector;
            let y = (luma[i] + 0.390625 * (brightness - 0.5)) * (contrast + screen);
            let u = angle.cos() * saturation[i] * 0.390625 * (1.0 - screen) * (contrast + screen);
            let v = angle.sin() * saturation[i] * 0.390625 * (1.0 - screen) * (contrast + screen);

            palette[i] = PalColour::new(
                ((y + 1.140 * v) * (256.0 - f32::EPSILON)) as u8,
                ((y - 0.396 * u - 0.581 * v) * (256.0 - f32::EPSILON)) as u8,
                ((y + 2.029 * u) * (256.0 - f32::EPSILON)) as u8,
            )
            .to_linear_rgb()
            .to_srgb();
        }

        Self(palette)
    }

    pub fn new_shared_tile_palette(&self, tile_palette_data: &[u8; 3]) -> SrgbPalette<3> {
        SrgbPalette([
            self[tile_palette_data[0] as usize],
            self[tile_palette_data[1] as usize],
            self[tile_palette_data[2] as usize],
        ])
    }

    pub fn new_tile_subpalettes(&self, shared_palette: &SrgbPalette<3>) -> [SrgbPalette<4>; 8] {
        array::from_fn(|index| {
            SrgbPalette([
                shared_palette[0],
                shared_palette[1],
                shared_palette[2],
                self[index],
            ])
        })
    }
}

impl<const S: usize> Index<usize> for SrgbPalette<S> {
    type Output = SrgbColour;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
