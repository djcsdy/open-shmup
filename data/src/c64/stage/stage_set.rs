use crate::c64::stage::C64StageData;
use crate::ext::ReadExt;
use std::io;
use std::io::{Read, Write};

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct C64StageSetData([u8; Self::SIZE_BYTES]);

impl C64StageSetData {
    const STAGE_COUNT: usize = 22;
    const SIZE_BYTES: usize = Self::STAGE_COUNT * C64StageData::SIZE_BYTES;

    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self(reader.read_u8_array()?))
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.0)
    }

    pub fn get(&self, index: usize) -> C64StageData {
        C64StageData::new(
            &self.0[index * C64StageData::SIZE_BYTES..(index + 1) * C64StageData::SIZE_BYTES],
        )
    }
}
