use crate::rect::Rect;
use web_sys::CanvasRenderingContext2d;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Screen {
    width: u32,
    height: u32,
    play_area: Rect,
}

impl Screen {
    pub const C64_PAL: Self = Self {
        width: 384,
        height: 288,
        play_area: Rect {
            x: 32,
            y: 48,
            width: 320,
            height: 192,
        },
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
            self.play_area.x as f64,
            self.play_area.y as f64,
            self.play_area.width as f64,
            self.play_area.height as f64,
        );
        context.clip();
        context
            .translate(self.play_area.x as f64, self.play_area.y as f64)
            .unwrap();
        draw(context);
        context.restore();
    }
}
