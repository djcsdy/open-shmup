use std::io;
use std::io::{Read, Write};

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct C64TileSetData {
    pub block_colours: [u8; 128],
    pub block_data: [u8; 3200],
    pub shared_colours: [u8; 3],
    pub tiles: [u8; 2032],
}

impl C64TileSetData {
    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut block_colours = [0u8; 128];
        reader.read_exact(&mut block_colours)?;

        let mut block_data = [0u8; 3200];
        reader.read_exact(&mut block_data)?;

        let mut shared_colours = [0u8; 3];
        reader.read_exact(&mut shared_colours)?;

        let mut tiles = [0u8; 2032];
        reader.read_exact(&mut tiles)?;

        Ok(Self {
            block_colours,
            block_data,
            shared_colours,
            tiles,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.block_colours)?;
        writer.write_all(&self.block_data)?;
        writer.write_all(&self.shared_colours)?;
        writer.write_all(&self.tiles)?;
        Ok(())
    }
}
