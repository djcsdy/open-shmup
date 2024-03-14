use crate::c64::stage::StageDataSet;
use crate::c64::tile::C64TileSetData;
use crate::c64::{C64TileBlockData, C64TileBlockSetData};
use crate::ext::array::array_from_fallible_fn;
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

        reader.seek(SeekFrom::Start(BLOCK_DATA - PRG_START))?;
        let blocks = array_from_fallible_fn(|i| {
            C64TileBlockData::read_tile_indices(reader, block_colours[i])
        })?;

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

        reader.seek(SeekFrom::Start(STAGE_DATA - PRG_START))?;
        let stage_data = StageDataSet::read(reader)?;

        let mut sound_effects = [0u8; 2432];
        reader.seek(SeekFrom::Start(SOUND_EFFECTS - PRG_START))?;
        reader.read_exact(&mut sound_effects)?;

        let mut sprite_graphics = [0u8; 8192];
        reader.seek(SeekFrom::Start(SPRITE_GRAPHICS - PRG_START))?;
        reader.read_exact(&mut sprite_graphics)?;

        let mut title_font = [0u8; 512];
        reader.seek(SeekFrom::Start(TITLE_FONT - PRG_START))?;
        reader.read_exact(&mut title_font)?;

        reader.seek(SeekFrom::Start(BACKGROUND_TILES - PRG_START))?;
        let tile_set = C64TileSetData::read(reader)?;

        Ok(Self {
            tile_set: C64TileBlockSetData::new(blocks, background_colours, tile_set),
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
