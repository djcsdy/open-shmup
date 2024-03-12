use crate::palette::SrgbColour;

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct SrgbaBitmap {
    pub width: usize,
    pub height: usize,
    pub bitmap: Vec<u8>,
}

impl SrgbaBitmap {
    pub fn from_srgb_fn<F: FnMut(usize, usize) -> SrgbColour>(
        width: usize,
        height: usize,
        mut f: F,
    ) -> Self {
        let mut bitmap = vec![0u8; width * height * 4];
        for y in 0..height {
            for x in 0..width {
                let colour = f(x, y);
                bitmap[(x + y * width) * 4] = colour.red();
                bitmap[(x + y * width) * 4 + 1] = colour.green();
                bitmap[(x + y * width) * 4 + 2] = colour.blue();
                bitmap[(x + y * width) * 4 + 3] = 255;
            }
        }
        Self {
            width,
            height,
            bitmap,
        }
    }
}
