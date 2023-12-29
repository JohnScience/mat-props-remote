macro_rules! decl_message {
    (@swap_bytes ($self:ident.$field:ident : u8)) => {};
    (@swap_bytes ($self:ident.$field:ident : f64)) => {
        $self.$field = f64::from_bits($self.$field.to_bits().swap_bytes());
    };
    (@py_struct_format_string_char (u8)) => { 'B' };
    (@py_struct_format_string_char (f64)) => { 'd' };
    (@to_py_struct_format_string_raw_loop($buf:ident, $cur_size:ident, $($ty:tt),+)) => {
        $(
            let size: usize = std::mem::size_of::<$ty>();
            let alignment: usize = std::mem::align_of::<$ty>();
            let padding = (alignment - ($cur_size % alignment)) % alignment;
            for _ in 0..padding {
                $buf.push('x');
            }
            let ch = decl_message!(@py_struct_format_string_char($ty));
            $buf.push(ch);
            // using += instead of the fully qualified syntax rendered the line an expression rather than a statement
            #[allow(unused_assignments)]
            <usize as core::ops::AddAssign>::add_assign(&mut $cur_size, padding + size);
        )+
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

            #[cfg(test)]
            pub(crate) fn to_py_struct_format_string() -> String {
                let mut buf = String::new();
                let mut cur_size = 0;

                // for alignment
                buf.push('B');
                cur_size += 1;

                decl_message!(@to_py_struct_format_string_raw_loop(buf, cur_size, $($ty),+));
                buf
            }
        }
    };
}

pub(crate) use decl_message;
