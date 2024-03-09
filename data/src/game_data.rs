use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io;
use std::io::{ErrorKind, Read, Write};

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct GameData {
    pub background_scroll_data: [u8; 4096],
    pub block_colours: [u8; 128],
    pub block_data: [u8; 3200],
    pub object_pointers: [u8; 1412],
    pub title_screen: [u8; 480],
    pub attack_wave_patterns: [u8; 3100],
    pub background_colours: [u8; 3],
    pub stage_data: [u8; 154],
    pub sound_effects: [u8; 2432],
    pub sprite_graphics: [u8; 8192],
    pub title_font: [u8; 512],
    pub background_tiles: [u8; 2040],
}

const SIGNATURE: &[u8; 20] = b"\x00open_shmup_game\xff\xfe\xfd\xfc";

impl GameData {
    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut signature = [0u8; SIGNATURE.len()];
        reader.read_exact(&mut signature)?;

        if signature != *SIGNATURE {
            return Err(io::Error::from(ErrorKind::InvalidData));
        }

        let version = reader.read_u32::<LittleEndian>()?;
        if version != 1 {
            return Err(io::Error::from(ErrorKind::InvalidData));
        }

        let mut background_scroll_data = [0u8; 4096];
        reader.read_exact(&mut background_scroll_data)?;

        let mut block_colours = [0u8; 128];
        reader.read_exact(&mut block_colours)?;

        let mut block_data = [0u8; 3200];
        reader.read_exact(&mut block_data)?;

        let mut object_pointers = [0u8; 1412];
        reader.read_exact(&mut object_pointers)?;

        let mut title_screen = [0u8; 480];
        reader.read_exact(&mut title_screen)?;

        let mut attack_wave_patterns = [0u8; 3100];
        reader.read_exact(&mut attack_wave_patterns)?;

        let mut background_colours = [0u8; 3];
        reader.read_exact(&mut background_colours)?;

        let mut stage_data = [0u8; 154];
        reader.read_exact(&mut stage_data)?;

        let mut sound_effects = [0u8; 2432];
        reader.read_exact(&mut sound_effects)?;

        let mut sprite_graphics = [0u8; 8192];
        reader.read_exact(&mut sprite_graphics)?;

        let mut title_font = [0u8; 512];
        reader.read_exact(&mut title_font)?;

        let mut background_tiles = [0u8; 2040];
        reader.read_exact(&mut background_tiles)?;

        Ok(Self {
            background_scroll_data,
            block_colours,
            block_data,
            object_pointers,
            title_screen,
            attack_wave_patterns,
            background_colours,
            stage_data,
            sound_effects,
            sprite_graphics,
            title_font,
            background_tiles,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(SIGNATURE)?;
        writer.write_u32::<LittleEndian>(1)?;
        writer.write_all(&self.background_scroll_data)?;
        writer.write_all(&self.block_colours)?;
        writer.write_all(&self.block_data)?;
        writer.write_all(&self.object_pointers)?;
        writer.write_all(&self.title_screen)?;
        writer.write_all(&self.attack_wave_patterns)?;
        writer.write_all(&self.background_colours)?;
        writer.write_all(&self.stage_data)?;
        writer.write_all(&self.sound_effects)?;
        writer.write_all(&self.sprite_graphics)?;
        writer.write_all(&self.title_font)?;
        writer.write_all(&self.background_tiles)?;
        Ok(())
    }
}
