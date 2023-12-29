macro_rules! decl_message {
    (@swap_bytes ($self:ident.$field:ident : u8)) => {};
    (@swap_bytes ($self:ident.$field:ident : f64)) => {
        $self.$field = f64::from_bits($self.$field.to_bits().swap_bytes());
    };
    ($name:ident {
        $($vis:vis $field:ident : $ty:ident),+
    }) => {
        #[derive(Clone, Copy)]
        #[repr(C)]
        pub(crate) struct $name {
            /// 0: little endian, 1: big endian.
            ///
            /// The endianness is the first field to enable optimization where
            /// the bytes of multi-byte fileds are swapped to match the native endianness
            /// of the server as they are received.
            pub(crate) endianness: u8,
            $($vis $field: $ty),+
        }

        impl $name {
            const SIZE: usize = std::mem::size_of::<Self>();

            #[cfg(test)]
            #[inline]
            pub(crate) fn into_bytes(self) -> [u8; Self::SIZE] {
                debug_assert!(core::mem::size_of::<Self>() == core::mem::size_of::<[u8; Self::SIZE]>());
                unsafe { std::mem::transmute::<Self, [u8; Self::SIZE]>(self) }
            }

            #[inline]
            pub(crate) fn endianness(&self) -> Option<Endianness> {
                crate::endianness::Endianness::try_from_u8(self.endianness)
            }

            pub(crate) fn reorder_bytes(&mut self) {
                $(
                    decl_message!(@swap_bytes(self.$field : $ty));
                )+
            }
        }
    };
}

pub(crate) use decl_message;
