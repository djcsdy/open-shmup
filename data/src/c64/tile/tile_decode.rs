pub trait C64TileDecode {
    fn colour_index_at(&self, x: usize, y: usize) -> u8;
}
