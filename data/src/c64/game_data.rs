use crate::c64::C64TileBlockSetData;
use crate::GameData;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io;
use std::io::{ErrorKind, Read, Seek, SeekFrom};

const PRG_START: u64 = 0x8fe;
const BACKGROUND_SCROLL_DATA: u64 = 0x900;
const BLOCK_COLOURS: u64 = 0x1900;
const BLOCK_DATA: u64 = 0x1a00;
const OBJECT_POINTERS: u64 = 0x2c80;
const TITLE_SCREEN: u64 = 0x3204;
const ATTACK_WAVE_PATTERNS: u64 = 0x33e0;
const BACKGROUND_COLOURS: u64 = 0x4085;
const STAGE_DATA: u64 = 0xae92;
const SOUND_EFFECTS: u64 = 0xb680;
const SPRITE_GRAPHICS: u64 = 0xc000;
const TITLE_FONT: u64 = 0xf400;
const BACKGROUND_TILES: u64 = 0xf800;

impl GameData {
    pub fn read_c64_prg<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let signature = reader.read_u16::<LittleEndian>()?;
        if signature != 0x42 {
            return Err(io::Error::from(ErrorKind::InvalidData));
        }

        let mut background_scroll_data = [0u8; 4096];
        reader.seek(SeekFrom::Start(BACKGROUND_SCROLL_DATA - PRG_START))?;
        reader.read_exact(&mut background_scroll_data)?;

        let mut block_colours = [0u8; 128];
        reader.seek(SeekFrom::Start(BLOCK_COLOURS - PRG_START))?;
        reader.read_exact(&mut block_colours)?;

        let mut block_data = [0u8; 3200];
        reader.seek(SeekFrom::Start(BLOCK_DATA - PRG_START))?;
        reader.read_exact(&mut block_data)?;

        let mut object_pointers = [0u8; 1412];
        reader.seek(SeekFrom::Start(OBJECT_POINTERS - PRG_START))?;
        reader.read_exact(&mut object_pointers)?;

        let mut title_screen = [0u8; 480];
        reader.seek(SeekFrom::Start(TITLE_SCREEN - PRG_START))?;
        reader.read_exact(&mut title_screen)?;

        let mut attack_wave_patterns = [0u8; 3100];
        reader.seek(SeekFrom::Start(ATTACK_WAVE_PATTERNS - PRG_START))?;
        reader.read_exact(&mut attack_wave_patterns)?;

        let mut background_colours = [0u8; 3];
        reader.seek(SeekFrom::Start(BACKGROUND_COLOURS - PRG_START))?;
        reader.read_exact(&mut background_colours)?;

        let mut stage_data = [0u8; 154];
        reader.seek(SeekFrom::Start(STAGE_DATA - PRG_START))?;
        reader.read_exact(&mut stage_data)?;

        let mut sound_effects = [0u8; 2432];
        reader.seek(SeekFrom::Start(SOUND_EFFECTS - PRG_START))?;
        reader.read_exact(&mut sound_effects)?;

        let mut sprite_graphics = [0u8; 8192];
        reader.seek(SeekFrom::Start(SPRITE_GRAPHICS - PRG_START))?;
        reader.read_exact(&mut sprite_graphics)?;

        let mut title_font = [0u8; 512];
        reader.seek(SeekFrom::Start(TITLE_FONT - PRG_START))?;
        reader.read_exact(&mut title_font)?;

        let mut background_tiles = [0u8; 2032];
        reader.seek(SeekFrom::Start(BACKGROUND_TILES - PRG_START))?;
        reader.read_exact(&mut background_tiles)?;

        Ok(Self {
            tile_set: C64TileBlockSetData {
                block_colours,
                block_data,
                shared_colours: background_colours,
                tiles: background_tiles,
            },
            background_scroll_data,
            object_pointers,
            title_screen,
            attack_wave_patterns,
            stage_data,
            sound_effects,
            sprite_graphics,
            title_font,
        })
    }
}
