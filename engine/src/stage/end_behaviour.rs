use binary_layout::LayoutAs;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::convert::Infallible;

#[repr(u8)]
#[derive(Eq, PartialEq, Clone, Hash, Debug, FromPrimitive, IntoPrimitive)]
pub enum EndBehaviour {
    Redraw = 0,
    Continue = 0x80,
    Loop = 0xff,
    #[num_enum(catch_all)]
    Invalid(u8) = 1,
}

impl LayoutAs<u8> for EndBehaviour {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u8) -> Result<Self, Self::ReadError> {
        Ok(v.into())
    }

    fn try_write(v: Self) -> Result<u8, Self::WriteError> {
        Ok(v.into())
    }
}
