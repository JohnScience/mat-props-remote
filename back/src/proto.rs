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

            pub(crate) const fn example_as_bytes() -> [u8; Self::SIZE] {
                Self::example().into_bytes()
            }

            // raison d'être:
            // https://stackoverflow.com/questions/48782047/how-do-i-use-serde-to-deserialize-arrays-greater-than-32-elements-such-as-u8
            pub(crate) const fn example_as_serde_big_array() -> serde_big_array::Array<u8, { Self::SIZE }> {
                serde_big_array::Array(Self::example_as_bytes())
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
            let ch = decl_resp_message!(@py_struct_format_string_char($ty));
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

            pub(crate) const fn content_type() -> &'static str {
                $content_type
            }

            pub(crate) fn py_struct_format_string() -> String {
                let mut buf = String::new();
                let mut cur_size = 0;

                decl_resp_message!(@to_py_struct_format_string_raw_loop(buf, cur_size, $($ty),+));
                buf
            }

            pub(crate) const fn example_as_array() -> [u8; Self::SIZE] {
                unsafe { core::mem::transmute(Self::example()) }
            }

            pub(crate) fn example_as_serde_big_array() -> serde_big_array::Array<u8, { Self::SIZE }> {
                serde_big_array::Array(Self::example_as_array())
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
                    ElasticModulesForUnidirectionalCompositeResponseMessage,
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

use std::f64::consts::PI;

decl_req_message!(
    #[content_type = "application/x.elastic-modules-for-unidirectional-composite-args-message"]
    message ElasticModulesForUnidirectionalCompositeArgsMessage {
        #[schema(minimum = 1, maximum = 2)]
        pub(crate) number_of_model: u8,
        pub(crate) fibre_content: f64,
        pub(crate) e_for_fiber: f64,
        pub(crate) nu_for_fiber: f64,
        pub(crate) e_for_matrix: f64,
        pub(crate) nu_for_matrix: f64
    }
);

decl_resp_message!(
    #[content_type = "application/x.elastic-modules-for-unidirectional-composite-response-message"]
    #[parcel =  ElasticModulesForUnidirectionalCompositeResponseParcel]
    message ElasticModulesForUnidirectionalCompositeResponseMessage {
        pub(crate) e1: f64,
        pub(crate) e2: f64,
        pub(crate) e3: f64,
        pub(crate) nu12: f64,
        pub(crate) nu13: f64,
        pub(crate) nu23: f64,
        pub(crate) g12: f64,
        pub(crate) g13: f64,
        pub(crate) g23: f64
    }
);

decl_req_message!(
    #[content_type = "application/x.elastic-modules-for-honeycomb-args-message"]
    message ElasticModulesForHoneycombArgsMessage {
        #[schema(minimum = 1, maximum = 1)]
        pub(crate) number_of_model: u8,
        pub(crate) l_cell_side_size: f64,
        pub(crate) h_cell_side_size: f64,
        pub(crate) wall_thickness: f64,
        pub(crate) angle: f64,
        pub(crate) e_for_honeycomb: f64,
        pub(crate) nu_for_honeycomb: f64
    }
);

decl_resp_message!(
    #[content_type = "application/x.elastic-modules-for-honeycomb-response-message"]
    #[parcel =  ElasticModulesForHoneycombResponseParcel]
    message ElasticModulesForHoneycombResponseMessage {
        pub(crate) e1: f64,
        pub(crate) e2: f64,
        pub(crate) e3: f64,
        pub(crate) nu12: f64,
        pub(crate) nu13: f64,
        pub(crate) nu23: f64,
        pub(crate) g12: f64,
        pub(crate) g13: f64,
        pub(crate) g23: f64
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
}

impl ElasticModulesForUnidirectionalCompositeResponseMessage {
    pub(crate) const fn example() -> Self {
        Self {
            e1: 40.01172332942556,
            e2: 6.7364802254566305,
            e3: 6.7364802254566305,
            nu12: 0.03840958253366131,
            nu13: 0.03840958253366131,
            nu23: 0.21620579415556423,
            g12: 2.9945407835581253,
            g13: 2.9945407835581253,
            g23: 2.769465602708258,
        }
    }
}

impl ElasticModulesForHoneycombArgsMessage {
    pub(crate) const fn example() -> Self {
        const ANGLE: f64 = PI / 6.0;
        Self {
            endianness: 0,
            number_of_model: 1,
            l_cell_side_size: 9.24,
            h_cell_side_size: 8.4619,
            wall_thickness: 0.4,
            angle: ANGLE,
            e_for_honeycomb: 7.07,
            nu_for_honeycomb: 0.2,
        }
    }
}

impl ElasticModulesForHoneycombResponseMessage {
    pub(crate) const fn example() -> Self {
        Self {
            e1: 0.0014972693834675922,
            e2: 0.0013344741623586129,
            e3: 0.3592394105863781,
            nu12: 1.0512175946777975,
            nu13: 0.0008335774635770805,
            nu23: 0.0007429441887683659,
            g12: 0.000288216866909449,
            g13: 0.07995563727728495,
            g23: 0.0755763830773748,
        }
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

    #[test]
    fn example_data_is_consistent_for_elastic_modules_for_unidirectional_composite() {
        let ElasticModulesForUnidirectionalCompositeArgsMessage {
            endianness: _,
            number_of_model,
            fibre_content,
            e_for_fiber,
            nu_for_fiber,
            e_for_matrix,
            nu_for_matrix,
        } = ElasticModulesForUnidirectionalCompositeArgsMessage::example();
        let res = mat_props::elastic_modules_for_unidirectional_composite(
            number_of_model,
            fibre_content,
            e_for_fiber,
            nu_for_fiber,
            e_for_matrix,
            nu_for_matrix,
        );
        let Ok(res) = res else { panic!() };
        let ElasticModulesForUnidirectionalCompositeResponseMessage {
            e1,
            e2,
            e3,
            nu12,
            nu13,
            nu23,
            g12,
            g13,
            g23,
        } = ElasticModulesForUnidirectionalCompositeResponseMessage::example();
        assert_eq!(res[0], e1);
        assert_eq!(res[1], e2);
        assert_eq!(res[2], e3);
        assert_eq!(res[3], nu12);
        assert_eq!(res[4], nu13);
        assert_eq!(res[5], nu23);
        assert_eq!(res[6], g12);
        assert_eq!(res[7], g13);
        assert_eq!(res[8], g23);
    }

    #[test]
    fn example_data_is_consistent_for_elastic_modules_for_honeycomb() {
        let ElasticModulesForHoneycombArgsMessage {
            endianness: _,
            number_of_model,
            l_cell_side_size,
            h_cell_side_size,
            wall_thickness,
            angle,
            e_for_honeycomb,
            nu_for_honeycomb,
        } = ElasticModulesForHoneycombArgsMessage::example();
        let res = mat_props::elastic_modules_for_honeycomb(
            number_of_model,
            l_cell_side_size,
            h_cell_side_size,
            wall_thickness,
            angle,
            e_for_honeycomb,
            nu_for_honeycomb,
        );
        let Ok(res) = res else { panic!() };
        let ElasticModulesForHoneycombResponseMessage {
            e1,
            e2,
            e3,
            nu12,
            nu13,
            nu23,
            g12,
            g13,
            g23,
        } = ElasticModulesForHoneycombResponseMessage::example();
        assert_eq!(res[0], e1);
        assert_eq!(res[1], e2);
        assert_eq!(res[2], e3);
        assert_eq!(res[3], nu12);
        assert_eq!(res[4], nu13);
        assert_eq!(res[5], nu23);
        assert_eq!(res[6], g12);
        assert_eq!(res[7], g13);
        assert_eq!(res[8], g23);
    }
}
