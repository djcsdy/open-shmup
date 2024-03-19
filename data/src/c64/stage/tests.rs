use crate::c64::stage::{stage, C64StageData};

#[test]
fn stage_data_has_expected_size() {
    assert_eq!(stage::layout::SIZE, Some(C64StageData::SIZE_BYTES));
}
