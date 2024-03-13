use crate::c64::tile::tile_block::C64TileBlockData;
use crate::c64::tile::C64TileSetData;
use crate::ext::array::array_from_fallible_fn;
use crate::image::SrgbaBitmap;
use crate::palette::SrgbPalette;
use std::io;
use std::io::{Read, Write};

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct C64TileBlockSetData {
    blocks: [C64TileBlockData; Self::BLOCK_COUNT],
    shared_colours: [u8; Self::SHARED_COLOUR_COUNT],
    tile_set: C64TileSetData,
}

impl C64TileBlockSetData {
    const BLOCK_COUNT: usize = 128;
    const SHARED_COLOUR_COUNT: usize = 3;

    pub(in crate::c64) fn new(
        blocks: [C64TileBlockData; Self::BLOCK_COUNT],
        shared_colours: [u8; Self::SHARED_COLOUR_COUNT],
        tile_set: C64TileSetData,
    ) -> Self {
        Self {
            blocks,
            shared_colours,
            tile_set,
        }
    }

    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let blocks = array_from_fallible_fn(|_| C64TileBlockData::read(reader))?;

        let mut shared_colours = [0u8; Self::SHARED_COLOUR_COUNT];
        reader.read_exact(&mut shared_colours)?;

        let tile_set = C64TileSetData::read(reader)?;

        Ok(Self {
            blocks,
            shared_colours,
            tile_set,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        for block in &self.blocks {
            block.write(writer)?;
        }
        writer.write_all(&self.shared_colours)?;
        self.tile_set.write(writer)?;
        Ok(())
    }

    pub fn to_srgba_bitmap_iter(
        &self,
        palette: &SrgbPalette<16>,
    ) -> C64TileBlockSetDataSrgbaBitmapIterator {
        let shared_palette = palette.new_shared_tile_palette(&self.shared_colours);
        let palettes = palette.new_tile_subpalettes(&shared_palette);

        C64TileBlockSetDataSrgbaBitmapIterator {
            tile_block_set: self,
            palettes,
            next_index: 0,
        }
    }
}

pub struct C64TileBlockSetDataSrgbaBitmapIterator<'tile_block_set> {
    tile_block_set: &'tile_block_set C64TileBlockSetData,
    palettes: [SrgbPalette<4>; 8],
    next_index: usize,
}

impl<'tile_block_set> Iterator for C64TileBlockSetDataSrgbaBitmapIterator<'tile_block_set> {
    type Item = SrgbaBitmap;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.tile_block_set.blocks.len() {
            None
        } else {
            let bitmap = self.tile_block_set.blocks[self.next_index]
                .to_srgba_bitmap(&self.palettes, &self.tile_block_set.tile_set);
            self.next_index += 1;
            Some(bitmap)
        }
    }
}
