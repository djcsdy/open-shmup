use binary_layout::LayoutAs;
use bitflags::bitflags;
use std::convert::Infallible;

bitflags! {
    #[derive(Eq, PartialEq, Clone, Hash, Debug)]
    pub struct ScrollType: u8 {
        const SCROLL = 0x80;
        const PUSH = 0x40;
        const FAST = 0x01;
    }
}

impl LayoutAs<u8> for ScrollType {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u8) -> Result<Self, Self::ReadError> {
        Ok(Self::from_bits_truncate(v))
    }

    fn try_write(v: Self) -> Result<u8, Self::WriteError> {
        Ok(v.bits())
    }
}
