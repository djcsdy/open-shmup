pub trait C64TileDecode {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn colour_index_at(&self, x: usize, y: usize) -> u8;
}
