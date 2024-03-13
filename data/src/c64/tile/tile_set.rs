use crate::c64::tile::C64TileData;
use crate::ext::array::array_from_fallible_fn;
use std::io;
use std::io::{Read, Write};
use std::ops::Index;

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct C64TileSetData([C64TileData; Self::TILE_COUNT]);

impl C64TileSetData {
    const TILE_COUNT: usize = 254;

    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self(array_from_fallible_fn(|_| C64TileData::read(reader))?))
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        for tile in &self.0 {
            tile.write(writer)?;
        }
        Ok(())
    }
}

impl Index<usize> for C64TileSetData {
    type Output = C64TileData;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
