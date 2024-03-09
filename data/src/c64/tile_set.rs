#[derive(Eq, PartialEq, Clone, Hash)]
pub struct C64TileSetData {
    pub block_colours: [u8; 128],
    pub block_data: [u8; 3200],
    pub shared_colours: [u8; 3],
    pub tiles: [u8; 2032],
}
