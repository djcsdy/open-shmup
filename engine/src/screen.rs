use web_sys::CanvasRenderingContext2d;

#[derive(PartialEq, Clone, Debug)]
pub struct Screen {
    width: u32,
    height: u32,
    play_area_x: u32,
    play_area_y: u32,
    play_area_width: u32,
    play_area_height: u32,
}

impl Screen {
    pub const C64_PAL: Self = Self {
        width: 384,
        height: 288,
        play_area_x: 32,
        play_area_y: 48,
        play_area_width: 320,
        play_area_height: 192,
    };

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn with_play_area<F: FnOnce(&CanvasRenderingContext2d) -> ()>(
        &self,
        context: &CanvasRenderingContext2d,
        draw: F,
    ) {
        context.save();
        context.rect(
            self.play_area_x as f64,
            self.play_area_y as f64,
            self.play_area_width as f64,
            self.play_area_height as f64,
        );
        context.clip();
        context
            .translate(self.play_area_x as f64, self.play_area_y as f64)
            .unwrap();
        draw(context);
        context.restore();
    }
}
