use crate::c64::stage::{EndBehaviour, ScrollType};
use crate::c64::tile::C64TileBlockData;
use crate::{Point, Rect};
use binary_layout::prelude::*;

pub type C64StageData<'stage_set> = layout::View<&'stage_set [u8]>;

binary_layout!(layout, LittleEndian, {
    start_position: u16,
    end_position: u16,
    duration: u8,
    scroll_type: ScrollType as u8,
    end_behaviour: EndBehaviour as u8
});

impl C64StageData<'_> {
    pub(crate) const SIZE_BYTES: usize = 7;
    const MAP_WIDTH_PX: i32 = 320;
    const MAP_HEIGHT_PX: i32 = 512 * C64TileBlockData::HEIGHT_PX as i32;

    pub fn map_rect(&self) -> Rect {
        let bottom = Self::translate_position(self.start_position().read());
        let top = if self
            .scroll_type()
            .read()
            .contains(ScrollType::SCROLL | ScrollType::PUSH)
        {
            Self::translate_position(self.end_position().read())
        } else {
            bottom - 192
        };
        Rect::from_top_left_bottom_right(
            Point { x: 0, y: top },
            Point {
                x: Self::MAP_WIDTH_PX,
                y: bottom,
            },
        )
    }

    fn translate_position(position: u16) -> i32 {
        Self::MAP_HEIGHT_PX - (position as i32 * 40)
    }
}
