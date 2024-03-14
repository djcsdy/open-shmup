use crate::c64::stage::{EndBehaviour, ScrollType};
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
}
