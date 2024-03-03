use crate::stage::Stage;

pub struct StageSet([u8; 154]);

impl StageSet {
    pub fn new(stage_data: [u8; 154]) -> Self {
        Self(stage_data)
    }

    pub fn get(&self, index: usize) -> Stage<&[u8]> {
        Stage::new(&self.0[index * 7..(index + 1) * 7])
    }
}
