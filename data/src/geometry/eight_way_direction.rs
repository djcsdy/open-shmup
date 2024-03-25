use binary_layout::LayoutAs;
use bitflags::bitflags;
use std::convert::Infallible;

bitflags! {
    #[derive(Eq, PartialEq, Clone, Hash, Debug)]
    pub struct EightWayDirection: u8 {
        const RIGHT = 1;
        const LEFT = 2;
        const DOWN = 4;
        const UP = 8;
    }
}

impl LayoutAs<u8> for EightWayDirection {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u8) -> Result<Self, Self::ReadError> {
        Ok(EightWayDirection::from_bits_truncate(v))
    }

    fn try_write(v: Self) -> Result<u8, Self::WriteError> {
        Ok(v.bits())
    }
}
