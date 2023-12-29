#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub(crate) enum Endianness {
    Little,
    Big,
}

impl Endianness {
    pub(crate) const NATIVE: Self = if cfg!(target_endian = "little") {
        Self::Little
    } else {
        Self::Big
    };

    pub(crate) fn try_from_u8(n: u8) -> Option<Self> {
        match n {
            0 => Some(Self::Little),
            1 => Some(Self::Big),
            _ => None,
        }
    }

    pub(crate) unsafe fn from_u8_unchecked(n: u8) -> Self {
        match n {
            0 => Self::Little,
            1 => Self::Big,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}
