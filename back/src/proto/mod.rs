mod elastic_modules_for_honeycomb;
mod elastic_modules_for_unidirectional_composite;
mod thermal_conductivity_for_unidirectional_composite;
mod thermal_expansion_for_honeycomb;
mod thermal_expansion_for_unidirectional_composite;

pub(crate) use elastic_modules_for_honeycomb::{
    ElasticModulesForHoneycombArgsMessage, ElasticModulesForHoneycombResponseMessage,
    ElasticModulesForHoneycombResponseParcel,
};
pub(crate) use elastic_modules_for_unidirectional_composite::{
    ElasticModulesForUnidirectionalCompositeArgsMessage,
    ElasticModulesForUnidirectionalCompositeResponseMessage,
    ElasticModulesForUnidirectionalCompositeResponseParcel,
};
pub(crate) use thermal_conductivity_for_unidirectional_composite::{
    ThermalConductivityForUnidirectionalCompositeArgsMessage,
    ThermalConductivityForUnidirectionalCompositeResponseMessage,
    ThermalConductivityForUnidirectionalCompositeResponseParcel,
};
pub(crate) use thermal_expansion_for_honeycomb::{
    ThermalExpansionForHoneycombArgsMessage, ThermalExpansionForHoneycombResponseMessage,
    ThermalExpansionForHoneycombResponseParcel,
};
pub(crate) use thermal_expansion_for_unidirectional_composite::{
    ThermalExpansionForUnidirectionalCompositeArgsMessage,
    ThermalExpansionForUnidirectionalCompositeResponseMessage,
    ThermalExpansionForUnidirectionalCompositeResponseParcel,
};

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
            let ch = crate::proto::decl_req_message!(@py_struct_format_string_char($ty));
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
            #[cfg(test)]
            pub(crate) const FIELD_COUNT: usize = {
                // 1 accounts for "endianness" field
                let mut count = 1;
                $(
                    stringify!($field);
                    count += 1;
                )+
                count
            };
            #[cfg(test)]
            pub(crate) const FIELDS: [&'static str; Self::FIELD_COUNT] = [
                "endianness",
                $(
                    stringify!($field),
                )+
            ];

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
                    crate::proto::decl_req_message!(@swap_bytes(self.$field : $ty));
                )+
            }

            pub(crate) fn py_struct_format_string() -> String {
                let mut buf = String::new();
                let mut cur_size = 0;

                // for alignment
                buf.push('B');
                cur_size += 1;

                crate::proto::decl_req_message!(@to_py_struct_format_string_raw_loop(buf, cur_size, $($ty),+));
                buf
            }

            pub(crate) fn content_type() -> &'static str {
                $content_type
            }

            pub(crate) const fn example_as_bytes() -> [u8; Self::SIZE] {
                Self::example().into_bytes()
            }

            // raison d'être:
            // https://stackoverflow.com/questions/48782047/how-do-i-use-serde-to-deserialize-arrays-greater-than-32-elements-such-as-u8
            pub(crate) const fn example_as_serde_big_array() -> serde_big_array::Array<u8, { Self::SIZE }> {
                serde_big_array::Array(Self::example_as_bytes())
            }

            #[cfg(test)]
            pub(crate) fn js_build_args_buffer() -> String {
                use std::fmt::Write;
                let mut s = String::new();
                write!(&mut s, "const args_buffer = new ArrayBuffer({});\n", Self::SIZE).unwrap();
                write!(&mut s, "const args = new DataView(args_buffer);\n").unwrap();
                let py_struct_format_string = Self::py_struct_format_string();
                let mut offset = 0;
                let mut field_idx = 0;
                for ch in py_struct_format_string.chars() {
                    match ch {
                        'B' => {
                            write!(&mut s, "args.setUint8({}, {});\n", offset, Self::FIELDS[field_idx]).unwrap();
                            offset += 1;
                            field_idx += 1;
                        },
                        'x' => {
                            write!(
                                &mut s,
                                "args.setUint8({}, 0); // padding\n",
                                offset,
                            ).unwrap();
                            offset += 1;
                        },
                        'd' => {
                            write!(&mut s, "args.setFloat64({}, {}, native_endianness);\n", offset, Self::FIELDS[field_idx]).unwrap();
                            offset += 8;
                            field_idx += 1;
                        },
                        _ => panic!("Unsupported character in py_struct_format_string: {}", ch),
                    };
                }
                s
            }
        }

        impl actix_web::FromRequest for $name {
            type Error = actix_web::Error;
            type Future = std::pin::Pin<
                Box<
                    dyn std::future::Future<
                        Output = Result<
                        $name,
                            actix_web::Error,
                        >,
                    >,
                >,
            >;

            fn from_request(
                req: &actix_web::HttpRequest,
                payload: &mut actix_web::dev::Payload,
            ) -> Self::Future {
                use futures_util::StreamExt;

                let payload = actix_web::web::Payload::from_request(req, payload);
                Box::pin(async move {
                    let payload = payload.await;
                    match payload {
                        Ok(mut p) => {
                            let mut buf = heapless::Vec::<
                                u8,
                                { $name::SIZE },
                                $name,
                            >::new();
                            while let Some(chunk) = p.next().await {
                                let Ok(chunk) = chunk else {
                                    return Err(actix_web::error::ErrorBadRequest(
                                        "Error receiving the payload",
                                    ));
                                };
                                buf.extend_from_slice(&chunk).map_err(|()| {
                                    actix_web::error::ErrorInternalServerError("Args buffer overflow")
                                })?;
                            }
                            let mut args: $name =
                                unsafe { buf.transmute_buffer() };
                            let Some(endianness) = args.endianness() else {
                                return Err(actix_web::error::ErrorBadRequest("Invalid endianness"));
                            };
                            if endianness != crate::Endianness::NATIVE {
                                args.reorder_bytes();
                            };
                            Ok(args)
                        }
                        Err(e) => return Err(e),
                    }
                })
            }
        }
    };
}

macro_rules! decl_resp_message {
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
            let ch = crate::proto::decl_resp_message!(@py_struct_format_string_char($ty));
            $buf.push(ch);
            // using += instead of the fully qualified syntax rendered the line an expression rather than a statement
            #[allow(unused_assignments)]
            <usize as core::ops::AddAssign>::add_assign(&mut $cur_size, padding + size);
        )+
    };
    (
        #[content_type = $content_type:tt]
        #[parcel = $parcel:ident]
        message $name:ident {
            $(
                $(#[$attr:meta])*
                $vis:vis $field:ident : $ty:ident
            ),+
        }
    ) => {
        #[derive(Clone, Copy, utoipa::ToSchema, bytemuck::Pod, bytemuck::Zeroable)]
        #[repr(C)]
        #[schema(example = $name::example_as_serde_big_array)]
        pub(crate) struct $name {
            $(
                $(#[$attr])*
                $vis $field: $ty
            ),+
        }

        impl $name {
            pub(crate) const SIZE: usize = core::mem::size_of::<Self>();
            #[cfg(test)]
            pub(crate) const FIELD_COUNT: usize = {
                let mut count = 0;
                $(
                    stringify!($field);
                    count += 1;
                )+
                count
            };
            #[cfg(test)]
            pub(crate) const FIELDS: [&'static str; Self::FIELD_COUNT] = [
                $(
                    stringify!($field),
                )+
            ];

            pub(crate) const fn content_type() -> &'static str {
                $content_type
            }

            pub(crate) fn py_struct_format_string() -> String {
                let mut buf = String::new();
                let mut cur_size = 0;

                crate::proto::decl_resp_message!(@to_py_struct_format_string_raw_loop(buf, cur_size, $($ty),+));
                buf
            }

            pub(crate) const fn example_as_array() -> [u8; Self::SIZE] {
                unsafe { core::mem::transmute(Self::example()) }
            }

            pub(crate) fn example_as_serde_big_array() -> serde_big_array::Array<u8, { Self::SIZE }> {
                serde_big_array::Array(Self::example_as_array())
            }

            #[cfg(test)]
            pub(crate) fn js_send_req() -> String {
                use std::fmt::Write;

                let mut s = String::new();
                s.push_str("const req = new XMLHttpRequest();\n");
                s.push_str("req.open('POST', url);\n");
                s.push_str("req.responseType = 'arraybuffer';\n");
                s.push_str("req.onload = (_event) => {\n");
                s.push_str("\t//const status_code = req.status;\n");
                s.push_str("\t//const console.log(`status_code = ${status_code}`);\n");
                s.push_str("\tconst array_buffer = req.response;\n");
                s.push_str("\tif (array_buffer) {\n");
                // FIXME: assumes that all the returned fields are f64
                s.push_str("\t\tconst resp_view = new Float64Array(array_buffer);\n");
                for (i, field) in Self::FIELDS.iter().enumerate() {
                    write!(
                        &mut s,
                        "\t\tconst {field} = resp_view[{i}];\n"
                    ).unwrap();
                }
                s.push_str("\t};\n");
                s.push_str("};\n");
                s
            }
        }

        pub(crate) struct $parcel {
            pub(crate) endianness: crate::Endianness,
            pub(crate) already_sent: bool,
            pub(crate) message: $name,
        }

        impl $parcel {
            pub(crate) fn new(
                endianness: crate::Endianness,
                message: $name,
            ) -> Self {
                Self {
                    endianness,
                    already_sent: false,
                    message,
                }
            }
        }

        impl actix_web::body::MessageBody for $parcel {
            type Error = actix_web::Error;

            fn size(&self) -> actix_web::body::BodySize {
                actix_web::body::BodySize::Sized(core::mem::size_of::<
                    $name,
                >() as u64)
            }

            fn poll_next(
                self: std::pin::Pin<&mut Self>,
                _: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Option<Result<actix_web::web::Bytes, actix_web::Error>>> {
                let Self {
                    ref endianness,
                    ref mut already_sent,
                    ref message,
                } = std::pin::Pin::into_inner(self);

                if *already_sent {
                    return std::task::Poll::Ready(None);
                }

                let iter = bytemuck::bytes_of(message);
                let bytes = if *endianness == crate::Endianness::NATIVE {
                    actix_web::web::Bytes::from_iter(iter.iter().copied())
                } else {
                    let iter = iter
                        .chunks_exact(core::mem::size_of::<f64>())
                        .flat_map(|chunk| {
                            let chunk = <&[u8; core::mem::size_of::<f64>()]>::try_from(chunk).unwrap();
                            chunk.iter().copied().rev()
                        });
                    actix_web::web::Bytes::from_iter(iter)
                };
                *already_sent = true;
                std::task::Poll::Ready(Some(Ok(bytes)))
            }
        }
    }
}

macro_rules! decl_req_resp_message_pair {
    (
        test $test_name:ident;
        fn $fn_name:path;

        #[content_type = $req_content_type:tt]
        message(req) $req_name:ident {
            $(
                $(#[$req_attr:meta])*
                $req_vis:vis $req_field:ident : $req_ty:ident
            ),+ $(,)?
        }

        #[content_type = $resp_content_type:tt]
        #[parcel = $resp_parcel:ident]
        message(resp) $resp_name:ident {
            $(
                $(#[$resp_attr:meta])*
                $resp_vis:vis $resp_field:ident : $resp_ty:ident
            ),+ $(,)?
        }

        impl $req_name_dup:ident {
            pub(crate) const fn example() -> Self $req_example_block:block
        }

        impl $resp_name_dup:ident {
            pub(crate) const fn example() -> Self $resp_example_block:block
        }
    ) => {
        crate::proto::decl_req_message!(
            #[content_type = $req_content_type]
            message $req_name {
                $(
                    $(#[$req_attr])*
                    $req_vis $req_field : $req_ty
                ),+
            }
        );

        crate::proto::decl_resp_message!(
            #[content_type = $resp_content_type]
            #[parcel = $resp_parcel]
            message $resp_name {
                $(
                    $(#[$resp_attr])*
                    $resp_vis $resp_field : $resp_ty
                ),+
            }
        );

        impl $req_name_dup {
            pub(crate) const fn example() -> Self $req_example_block
        }

        impl $resp_name_dup {
            pub(crate) const fn example() -> Self $resp_example_block
        }

        #[cfg(test)]
        #[test]
        fn $test_name() {
            let $req_name {
                endianness: _,
                $(
                    $req_field
                ),+
            } = $req_name::example();
            let res = $fn_name(
                $(
                    $req_field
                ),+
            ).unwrap();
            let $resp_name {
                $(
                    $resp_field
                ),+
            } = $resp_name::example();
            let mut i = 0;
            $(
                assert_eq!(res[i], $resp_field);
                #[allow(unused_assignments)]
                <usize as core::ops::AddAssign>::add_assign(&mut i, 1);
            )+
        }
    };
}

pub(crate) use decl_req_message;
pub(crate) use decl_req_resp_message_pair;
pub(crate) use decl_resp_message;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn see_js_build_args_buffer_for_elastic_modules_for_unidirectional_composite_args_message() {
        println!(
            "{}",
            ElasticModulesForUnidirectionalCompositeArgsMessage::js_build_args_buffer()
        );
    }

    #[test]
    fn see_js_send_req_for_elastic_modules_for_unidirectional_composite_response_message() {
        println!(
            "{}",
            ElasticModulesForUnidirectionalCompositeResponseMessage::js_send_req()
        );
    }

    #[test]
    fn see_js_build_args_buffer_for_elastic_modules_for_honeycomb_args_message() {
        println!(
            "{}",
            ElasticModulesForHoneycombArgsMessage::js_build_args_buffer()
        );
    }

    #[test]
    fn see_js_send_req_for_elastic_modules_for_honeycomb_response_message() {
        println!(
            "{}",
            ElasticModulesForHoneycombResponseMessage::js_send_req()
        );
    }

    #[test]
    fn see_js_build_args_buffer_for_thermal_expansion_for_unidirectional_composite_args_message() {
        println!(
            "{}",
            ThermalExpansionForUnidirectionalCompositeArgsMessage::js_build_args_buffer()
        );
    }

    #[test]
    fn see_js_send_req_for_thermal_expansion_for_unidirectional_composite_response_message() {
        println!(
            "{}",
            ThermalExpansionForUnidirectionalCompositeResponseMessage::js_send_req()
        );
    }

    #[test]
    fn see_js_build_args_buffer_for_thermal_expansion_for_honeycomb_args_message() {
        println!(
            "{}",
            ThermalExpansionForHoneycombArgsMessage::js_build_args_buffer()
        );
    }

    #[test]
    fn see_js_send_req_for_thermal_expansion_for_honeycomb_args_message() {
        println!(
            "{}",
            ThermalExpansionForHoneycombResponseMessage::js_send_req()
        );
    }

    #[test]
    fn see_js_build_args_buffer_for_thermal_conductivity_for_unidirectional_composite_args_message()
    {
        println!(
            "{}",
            ThermalConductivityForUnidirectionalCompositeArgsMessage::js_build_args_buffer()
        );
    }

    #[test]
    fn see_js_send_req_for_thermal_conductivity_for_unidirectional_composite_response_message() {
        println!(
            "{}",
            ThermalConductivityForUnidirectionalCompositeResponseMessage::js_send_req()
        );
    }
}
