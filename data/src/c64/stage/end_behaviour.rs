use binary_layout::LayoutAs;
use num_enum::IntoPrimitive;
use std::convert::Infallible;

#[repr(u8)]
#[derive(Eq, PartialEq, Clone, Hash, Debug, IntoPrimitive)]
pub enum EndBehaviour {
    Redraw = 0,
    Continue = 0x80,
    Loop = 0xff,
}

impl From<u8> for EndBehaviour {
    fn from(value: u8) -> Self {
        if value == 0xff {
            Self::Loop
        } else if value & 0x80 == 0x80 {
            Self::Continue
        } else {
            Self::Redraw
        }
    }
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
