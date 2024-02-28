#[derive(Eq, PartialEq, Clone)]
pub struct Tile(pub(super) [u8; 8]);

impl Tile {
    pub fn tile_data(&self) -> &[u8; 8] {
        &self.0
    }
}
