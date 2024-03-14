use crate::c64::stage::StageData;
use crate::ext::ReadExt;
use std::io;
use std::io::Read;

pub struct StageDataSet([u8; Self::SIZE_BYTES]);

impl StageDataSet {
    const STAGE_COUNT: usize = 22;
    const SIZE_BYTES: usize = Self::STAGE_COUNT * StageData::SIZE_BYTES;

    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self(reader.read_u8_array()?))
    }

    pub fn get(&self, index: usize) -> StageData {
        StageData::new(&self.0[index * StageData::SIZE_BYTES..(index + 1) * StageData::SIZE_BYTES])
    }
}
