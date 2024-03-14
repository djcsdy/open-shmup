use crate::stage::StageData;

pub struct StageDataSet([u8; 154]);

impl StageDataSet {
    pub fn new(stage_data: [u8; 154]) -> Self {
        Self(stage_data)
    }

    pub fn get(&self, index: usize) -> StageData {
        StageData::new(&self.0[index * 7..(index + 1) * 7])
    }
}
