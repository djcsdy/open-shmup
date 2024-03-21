use crate::Point;
use binary_layout::binary_layout;
use std::io;
use std::io::Read;

pub struct C64SpawnEnemyAtTilemapPosition(layout::View<[u8; Self::SIZE_BYTES]>);

binary_layout!(layout, LittleEndian, {
    scroll_position: u16,
    movement_pointer: u16,
    x_position: u8,
    y_position: u8,
    enemy_type: u8
});

impl C64SpawnEnemyAtTilemapPosition {
    pub(super) const SIZE_BYTES: usize = 7;

    pub(super) fn read<R: Read>(reader: &mut R) -> io::Result<Option<Self>> {
        let mut data = [0u8; Self::SIZE_BYTES];
        reader.read_exact(&mut data)?;
        let view = layout::View::new(data);
        Ok(if view.scroll_position().read() == 0xffff {
            None
        } else {
            Some(Self(view))
        })
    }

    pub fn scroll_position(&self) -> u16 {
        self.0.scroll_position().read()
    }

    pub fn spawn_point(&self) -> Point {
        Point {
            x: self.0.x_position().read() as i32 * 2,
            y: self.0.y_position().read() as i32,
        }
    }

    pub fn enemy_type(&self) -> usize {
        self.0.enemy_type().read() as usize
    }
}
