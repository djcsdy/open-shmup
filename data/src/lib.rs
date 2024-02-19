pub struct Game {
    pub background_scroll_data: [u8; 4096],
    pub block_colours: [u8; 128],
    pub block_data: [u8; 3072],
    pub object_pointers: [u8; 1412],
    pub title_screen: [u8; 480],
    pub attack_wave_patterns: [u8; 3100],
    pub sound_effects: [u8; 23296],
    pub sprite_graphics: [u8; 8192],
    pub background_tiles: [u8; 2040],
}
