use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};
use clap::Parser;

use open_shmup_data::Game;

use crate::options::Options;

mod options;

const PRG_START: u64 = 0x8fe;
const BACKGROUND_SCROLL_DATA: u64 = 0x900;
const BLOCK_COLOURS: u64 = 0x1900;
const BLOCK_DATA: u64 = 0x1a00;
const OBJECT_POINTERS: u64 = 0x2c80;
const TITLE_SCREEN: u64 = 0x3204;
const ATTACK_WAVE_PATTERNS: u64 = 0x33e0;
const SOUND_EFFECTS: u64 = 0xb680;
const SPRITE_GRAPHICS: u64 = 0xc000;
const BACKGROUND_TILES: u64 = 0xf800;

fn main() {
    let options = Options::parse();

    let mut file = File::open(options.path).unwrap();
    let signature = file.read_u16::<LittleEndian>().unwrap();
    if signature != 0x42 {
        panic!("Not a SEUCK game");
    }

    let mut background_scroll_data = [0u8; 4096];
    file.seek(SeekFrom::Start(BACKGROUND_SCROLL_DATA - PRG_START))
        .unwrap();
    file.read_exact(&mut background_scroll_data).unwrap();

    let mut block_colours = [0u8; 128];
    file.seek(SeekFrom::Start(BLOCK_COLOURS - PRG_START))
        .unwrap();
    file.read_exact(&mut block_colours).unwrap();

    let mut block_data = [0u8; 3072];
    file.seek(SeekFrom::Start(BLOCK_DATA - PRG_START)).unwrap();
    file.read_exact(&mut block_data).unwrap();

    let mut object_pointers = [0u8; 1412];
    file.seek(SeekFrom::Start(OBJECT_POINTERS - PRG_START))
        .unwrap();
    file.read_exact(&mut object_pointers).unwrap();

    let mut title_screen = [0u8; 480];
    file.seek(SeekFrom::Start(TITLE_SCREEN - PRG_START))
        .unwrap();
    file.read_exact(&mut title_screen).unwrap();

    let mut attack_wave_patterns = [0u8; 3100];
    file.seek(SeekFrom::Start(ATTACK_WAVE_PATTERNS - PRG_START))
        .unwrap();
    file.read_exact(&mut attack_wave_patterns).unwrap();

    let mut sound_effects = [0u8; 2432];
    file.seek(SeekFrom::Start(SOUND_EFFECTS - PRG_START))
        .unwrap();
    file.read_exact(&mut sound_effects).unwrap();

    let mut sprite_graphics = [0u8; 8192];
    file.seek(SeekFrom::Start(SPRITE_GRAPHICS - PRG_START))
        .unwrap();
    file.read_exact(&mut sprite_graphics).unwrap();

    let mut background_tiles = [0u8; 2040];
    file.seek(SeekFrom::Start(BACKGROUND_TILES - PRG_START))
        .unwrap();
    file.read_exact(&mut background_tiles).unwrap();

    let game = Game {
        background_scroll_data,
        block_colours,
        block_data,
        object_pointers,
        title_screen,
        attack_wave_patterns,
        sound_effects,
        sprite_graphics,
        background_tiles,
    };
}
