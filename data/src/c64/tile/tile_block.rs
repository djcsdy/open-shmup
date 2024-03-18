use crate::c64::tile::hires_tile_block::C64HiresTileBlockData;
use crate::c64::tile::multicolour_tile_block::C64MulticolourTileBlockData;
use crate::c64::{C64TileData, C64TileDecode, C64TileSetData};
use crate::image::SrgbaBitmap;
use crate::palette::SrgbPalette;
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
    pub(super) const WIDTH_PX: usize = Self::WIDTH_TILES * C64TileData::WIDTH;
    pub(crate) const HEIGHT_PX: usize = Self::HEIGHT_TILES * C64TileData::HEIGHT;
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

    pub fn read_tile_indices<R: Read>(reader: &mut R, colour_data: u8) -> io::Result<Self> {
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

    pub fn to_srgba_bitmap(
        &self,
        palettes: &[SrgbPalette<4>; 8],
        tile_set: &C64TileSetData,
    ) -> SrgbaBitmap {
        let palette = &palettes[(self.colour_data & 7) as usize];
        if self.colour_data & 8 == 8 {
            self.as_multicolour(tile_set).to_srgba_bitmap(palette)
        } else {
            self.as_hires(tile_set).to_srgba_bitmap(palette)
        }
    }

    fn as_multicolour<'tile_block, 'tile_set>(
        &'tile_block self,
        tile_set: &'tile_set C64TileSetData,
    ) -> C64MulticolourTileBlockData<'tile_block, 'tile_set> {
        C64MulticolourTileBlockData::new(self, tile_set)
    }

    fn as_hires<'tile_block, 'tile_set>(
        &'tile_block self,
        tile_set: &'tile_set C64TileSetData,
    ) -> C64HiresTileBlockData<'tile_block, 'tile_set> {
        C64HiresTileBlockData::new(self, tile_set)
    }

    pub(super) fn get_tile_index(&self, x: usize, y: usize) -> usize {
        let tile_index_in_block = x + y * Self::WIDTH_TILES;
        self.tile_indices[tile_index_in_block] as usize
    }
}
