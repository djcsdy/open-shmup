use crate::colour::Colour;
use std::f32::consts::PI;

pub struct Palette16([Colour; 16]);

pub struct Palette4([Colour; 4]);

impl Palette16 {
    /// Generates a Colodore Palette as documented at https://www.pepto.de/projects/colorvic/
    pub fn new_colodore() -> Self {
        let brightness_percent = 50.0;
        let contrast_percent = 100.0;
        let saturation_percent = 50.0;

        let luma = [
            0.0, 32.0, 10.0, 20.0, 12.0, 16.0, 8.0, 24.0, 12.0, 8.0, 16.0, 10.0, 15.0, 24.0, 15.0,
            20.0,
        ];
        let chroma = [
            0.0, 0.0, 4.0, 12.0, 2.0, 10.0, 15.0, 7.0, 5.0, 6.0, 4.0, 0.0, 0.0, 10.0, 15.0, 0.0,
        ];

        let sector = PI / 8.0;
        let origin = sector / 2.0;
        let screen = 0.2;

        let brightness = brightness_percent - 50.0;
        let contrast = contrast_percent / 100.0;
        let saturation = saturation_percent * (1.0 - screen);

        let source_gamma = 2.8; // PAL
        let target_gamma = 2.2; // sRGB

        let mut palette = [Colour {
            red: 0,
            green: 0,
            blue: 0,
        }; 16];

        for i in 0..palette.len() {
            let angle = origin + chroma[i] * sector;
            let y = (8.0 * luma[i] + brightness) * (contrast + screen);
            let u = angle.cos() * saturation * (contrast + screen);
            let v = angle.sin() * saturation * (contrast + screen);

            palette[i].red = Self::gamma_correct(
                (y + 1.140 * v).clamp(0.0, 255.0),
                source_gamma,
                target_gamma,
            ) as u8;

            palette[i].green = Self::gamma_correct(
                (y - 0.396 * u - 0.581 * v).clamp(0.0, 255.0),
                source_gamma,
                target_gamma,
            ) as u8;

            palette[i].blue = Self::gamma_correct(
                (y + 2.029 * u).clamp(0.0, 255.0),
                source_gamma,
                target_gamma,
            ) as u8;
        }

        Self(palette)
    }

    fn gamma_correct(value: f32, source_gamma: f32, target_gamma: f32) -> f32 {
        (value.powf(source_gamma) * f32::powf(255.0, 1.0 - source_gamma)).powf(1.0 / target_gamma)
            * f32::powf(255.0, 1.0 - 1.0 / target_gamma)
    }
}
