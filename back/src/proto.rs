macro_rules! decl_req_message {
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
            let ch = decl_req_message!(@py_struct_format_string_char($ty));
            $buf.push(ch);
            // using += instead of the fully qualified syntax rendered the line an expression rather than a statement
            #[allow(unused_assignments)]
            <usize as core::ops::AddAssign>::add_assign(&mut $cur_size, padding + size);
        )+
    };
    (
        #[content_type = $content_type:tt]
        message $name:ident {
        $(
            $(#[$attr:meta])*
            $vis:vis $field:ident : $ty:ident
        ),+
    }) => {
        #[derive(Clone, Copy, utoipa::ToSchema)]
        #[repr(C)]
        pub(crate) struct $name {
            /// 0: little endian, 1: big endian.
            ///
            /// The endianness is the first field to enable optimization where
            /// the bytes of multi-byte fileds are swapped to match the native endianness
            /// of the server as they are received.
            #[schema(minimum = 0, maximum = 1)]
            pub(crate) endianness: u8,
            $(
                $(#[$attr])*
                $vis $field: $ty
            ),+
        }

        impl $name {
            pub(crate) const SIZE: usize = std::mem::size_of::<Self>();

            #[inline]
            pub(crate) const fn into_bytes(self) -> [u8; Self::SIZE] {
                debug_assert!(core::mem::size_of::<Self>() == core::mem::size_of::<[u8; Self::SIZE]>());
                unsafe { std::mem::transmute::<Self, [u8; Self::SIZE]>(self) }
            }

            #[inline]
            pub(crate) fn endianness(&self) -> Option<crate::endianness::Endianness> {
                crate::endianness::Endianness::try_from_u8(self.endianness)
            }

            pub(crate) fn reorder_bytes(&mut self) {
                $(
                    decl_req_message!(@swap_bytes(self.$field : $ty));
                )+
            }

            pub(crate) fn py_struct_format_string() -> String {
                let mut buf = String::new();
                let mut cur_size = 0;

                // for alignment
                buf.push('B');
                cur_size += 1;

                decl_req_message!(@to_py_struct_format_string_raw_loop(buf, cur_size, $($ty),+));
                buf
            }

            pub(crate) fn content_type() -> &'static str {
                $content_type
            }
        }
    };
}

pub(crate) use decl_req_message;

decl_req_message!(
    #[content_type = "application/x.elastic-modules-for-unidirectional-composite-args-message"]
    message ElasticModulesForUnidirectionalCompositeArgsMessage {
        #[schema(minimum = 0, maximum = 2)]
        pub(crate) number_of_model: u8,
        pub(crate) fibre_content: f64,
        pub(crate) e_for_fiber: f64,
        pub(crate) nu_for_fiber: f64,
        pub(crate) e_for_matrix: f64,
        pub(crate) nu_for_matrix: f64
    }
);

impl ElasticModulesForUnidirectionalCompositeArgsMessage {
    pub(crate) const fn example() -> Self {
        Self {
            endianness: 0,
            number_of_model: 2,
            fibre_content: 0.2,
            e_for_fiber: 100.0,
            nu_for_fiber: 0.3,
            e_for_matrix: 5.0,
            nu_for_matrix: 0.2,
        }
    }

    pub(crate) const fn example_as_bytes() -> [u8; Self::SIZE] {
        Self::example().into_bytes()
    }

    // raison d'être:
    // https://stackoverflow.com/questions/48782047/how-do-i-use-serde-to-deserialize-arrays-greater-than-32-elements-such-as-u8
    pub(crate) const fn example_as_serde_big_array() -> serde_big_array::Array<u8, { Self::SIZE }> {
        serde_big_array::Array(Self::example_as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn see_elastic_modules_for_unidirectional_composite_args_message_example_as_bytes() {
        println!(
            "{:?}",
            ElasticModulesForUnidirectionalCompositeArgsMessage::example_as_bytes()
        );
    }
}
