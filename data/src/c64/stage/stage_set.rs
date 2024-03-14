use crate::c64::stage::StageData;

pub struct StageDataSet([u8; Self::SIZE_BYTES]);

impl StageDataSet {
    const STAGE_COUNT: usize = 22;
    const SIZE_BYTES: usize = Self::STAGE_COUNT * StageData::SIZE_BYTES;

    pub fn new(stage_data: [u8; Self::SIZE_BYTES]) -> Self {
        Self(stage_data)
    }

    pub fn get(&self, index: usize) -> StageData {
        StageData::new(&self.0[index * StageData::SIZE_BYTES..(index + 1) * StageData::SIZE_BYTES])
    }
}
