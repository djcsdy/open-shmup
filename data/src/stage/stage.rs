use crate::stage::{EndBehaviour, ScrollType};
use binary_layout::prelude::*;

pub type StageData<'stage_data_set> = layout::View<&'stage_data_set [u8]>;

binary_layout!(layout, LittleEndian, {
    start_position: u16,
    end_position: u16,
    duration: u8,
    scroll_type: ScrollType as u8,
    end_behaviour: EndBehaviour as u8
});
