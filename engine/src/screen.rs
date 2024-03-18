use open_shmup_data::Rect;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Screen {
    pub width: u32,
    pub height: u32,
    pub play_area: Rect,
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
}
