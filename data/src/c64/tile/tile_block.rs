use crate::c64::tile::hires_tile_block::C64HiresTileBlockData;
use crate::c64::tile::multicolour_tile_block::C64MulticolourTileBlockData;
use crate::c64::C64TileSetData;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io;
use std::io::{Read, Write};

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct C64TileBlockData {
    pub colour_data: u8,
    pub tile_indices: [u8; Self::SIZE_BYTES],
}

impl C64TileBlockData {
    const WIDTH_TILES: usize = 5;
    const HEIGHT_TILES: usize = 5;
    const SIZE_BYTES: usize = Self::WIDTH_TILES * Self::HEIGHT_TILES;

    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let colour_data = reader.read_u8()?;
        let mut tile_indices = [0u8; Self::SIZE_BYTES];
        reader.read_exact(&mut tile_indices)?;
        Ok(Self {
            colour_data,
            tile_indices,
        })
    }

    pub fn read_image_data<R: Read>(reader: &mut R, colour_data: u8) -> io::Result<Self> {
        let mut tile_indices = [0u8; Self::SIZE_BYTES];
        reader.read_exact(&mut tile_indices)?;
        Ok(Self {
            colour_data,
            tile_indices,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u8(self.colour_data)?;
        writer.write_all(&self.tile_indices)
    }

    pub fn as_multicolour<'tile_block, 'tile_set>(
        &'tile_block self,
        tile_set: &'tile_set C64TileSetData,
    ) -> C64MulticolourTileBlockData<'tile_block, 'tile_set> {
        C64MulticolourTileBlockData::new(self, tile_set)
    }

    pub fn as_hires<'tile_block, 'tile_set>(
        &'tile_block self,
        tile_set: &'tile_set C64TileSetData,
    ) -> C64HiresTileBlockData<'tile_block, 'tile_set> {
        C64HiresTileBlockData::new(self, tile_set)
    }
}
