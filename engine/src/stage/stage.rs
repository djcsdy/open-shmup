use binary_layout::prelude::*;

pub type Stage<S> = layout::View<S>;

binary_layout!(layout, LittleEndian, {
    start_position: u16,
    end_position: u16,
    duration: u8,
    scroll_type: u8,
    end_behaviour: u8
});
