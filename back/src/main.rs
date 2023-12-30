use actix_web::{get, post, web, App, HttpServer, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod endianness;
mod proto;

use endianness::Endianness;

#[derive(OpenApi)]
#[openapi(
    paths(index, elastic_modules_for_unidirectional_composite),
    components(schemas(
        ElasticModulesForUnidirectionalCompositeArgsMessage,
        elastic_modules_for_unidirectional_composite_extra::ElasticModulesForUnidirectionalCompositeResponseMessage,
    ))
)]
struct ApiDoc;

mod elastic_modules_for_unidirectional_composite_extra {
    use crate::proto::decl_message;
    use crate::Endianness;
    use actix_web::body::MessageBody;
    use bytemuck::{Pod, Zeroable};
    use futures_util::{Future, StreamExt};
    use std::pin::Pin;
    use utoipa::ToSchema;

    decl_message!(ElasticModulesForUnidirectionalCompositeArgsMessage {
        #[schema(minimum = 0, maximum = 2)]
        pub(crate) number_of_model: u8,
        pub(crate) fibre_content: f64,
        pub(crate) e_for_fiber: f64,
        pub(crate) nu_for_fiber: f64,
        pub(crate) e_for_matrix: f64,
        pub(crate) nu_for_matrix: f64
    });

    #[derive(Clone, Copy, ToSchema, Pod, Zeroable)]
    #[repr(C)]
    #[schema(example = json!([211, 61, 106, 38, 128, 1, 68, 64, 159, 242, 73, 223, 39, 242, 26, 64, 159, 242, 73, 223, 39, 242, 26, 64, 172, 144, 171, 185, 107, 170, 163, 63, 172, 144, 171, 185, 107, 170, 163, 63, 42, 75, 141, 167, 161, 172, 203, 63, 231, 92, 95, 204, 209, 244, 7, 64, 231, 92, 95, 204, 209, 244, 7, 64, 46, 59, 248, 148, 221, 39, 6, 64]))]
    pub(crate) struct ElasticModulesForUnidirectionalCompositeResponseMessage {
        pub(crate) e1: f64,
        pub(crate) e2: f64,
        pub(crate) e3: f64,
        pub(crate) nu12: f64,
        pub(crate) nu13: f64,
        pub(crate) nu23: f64,
        pub(crate) g12: f64,
        pub(crate) g13: f64,
        pub(crate) g23: f64,
    }

    pub(crate) struct ElasticModulesForUnidirectionalCompositeResponseParcel {
        pub(crate) endianness: Endianness,
        pub(crate) already_sent: bool,
        pub(crate) message: ElasticModulesForUnidirectionalCompositeResponseMessage,
    }

    impl ElasticModulesForUnidirectionalCompositeResponseParcel {
        pub(crate) fn new(
            endianness: Endianness,
            message: ElasticModulesForUnidirectionalCompositeResponseMessage,
        ) -> Self {
            Self {
                endianness,
                already_sent: false,
                message,
            }
        }
    }

    // TODO: an implementation for any type that implements `AnyBitPattern` and suggest it for actix_web
    impl actix_web::FromRequest for ElasticModulesForUnidirectionalCompositeArgsMessage {
        type Error = actix_web::Error;
        type Future = Pin<
            Box<
                dyn Future<
                    Output = Result<
                        ElasticModulesForUnidirectionalCompositeArgsMessage,
                        actix_web::Error,
                    >,
                >,
            >,
        >;

        fn from_request(
            req: &actix_web::HttpRequest,
            payload: &mut actix_web::dev::Payload,
        ) -> Self::Future {
            let payload = actix_web::web::Payload::from_request(req, payload);
            Box::pin(async move {
                let payload = payload.await;
                match payload {
                    Ok(mut p) => {
                        let mut buf = heapless::AlignedVec::<
                            u8,
                            ElasticModulesForUnidirectionalCompositeArgsMessage,
                            { ElasticModulesForUnidirectionalCompositeArgsMessage::SIZE },
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
                        let mut args: ElasticModulesForUnidirectionalCompositeArgsMessage =
                            unsafe { buf.transmute_buffer() };
                        let Some(endianness) = args.endianness() else {
                            return Err(actix_web::error::ErrorBadRequest("Invalid endianness"));
                        };
                        if endianness != Endianness::NATIVE {
                            args.reorder_bytes();
                        };
                        Ok(args)
                    }
                    Err(e) => return Err(e),
                }
            })
        }
    }

    impl MessageBody for ElasticModulesForUnidirectionalCompositeResponseParcel {
        type Error = actix_web::Error;

        fn size(&self) -> actix_web::body::BodySize {
            actix_web::body::BodySize::Sized(core::mem::size_of::<
                ElasticModulesForUnidirectionalCompositeResponseMessage,
            >() as u64)
        }

        fn poll_next(
            self: Pin<&mut Self>,
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
            let bytes = if *endianness == Endianness::NATIVE {
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

pub(crate) use elastic_modules_for_unidirectional_composite_extra::{
    ElasticModulesForUnidirectionalCompositeArgsMessage,
    ElasticModulesForUnidirectionalCompositeResponseMessage,
    ElasticModulesForUnidirectionalCompositeResponseParcel,
};

#[utoipa::path(
    get,
    request_body = (),
    responses (
        (status = 200, description = "Hello world!", content_type = "text/plain"),
    )
)]
#[get("/")]
async fn index() -> impl Responder {
    "Hello world!"
}

#[utoipa::path(
    post,
    request_body(
        content = ElasticModulesForUnidirectionalCompositeArgsMessage,
        description = "Python struct format string: \"BBxxxxxxddddd\". See <https://docs.python.org/3/library/struct.html#format-strings>. \
        See schema for the order of the fields (but not their sizes).",
        content_type = "application/x.elastic-modules-for-unidirectional-composite-args-message",
        example = json!("[0, 2, 0, 0, 0, 0, 0, 0, 154, 153, 153, 153, 153, 153, 201, 63, 0, 0, 0, 0, 0, 0, 89, 64, 51, 51, 51, 51, 51, 51, 211, 63, 0, 0, 0, 0, 0, 0, 20, 64, 154, 153, 153, 153, 153, 153, 201, 63]"),
    ),
    responses (
        (
            status = 200,
            description = "Computes elastic_modules_for_unidirectional_composite. \
            Returns [E1, E2, E3, nu12, nu13, nu23, G12, G13, G23].",
            body = ElasticModulesForUnidirectionalCompositeResponseMessage,
            content_type = "application/x.elastic-modules-for-unidirectional-composite-response-message"
        ),
    )
)]
#[post("/compute/elastic_modules_for_unidirectional_composite")]
async fn elastic_modules_for_unidirectional_composite(
    args: ElasticModulesForUnidirectionalCompositeArgsMessage,
) -> impl Responder {
    let ElasticModulesForUnidirectionalCompositeArgsMessage {
        endianness,
        number_of_model,
        fibre_content,
        e_for_fiber,
        nu_for_fiber,
        e_for_matrix,
        nu_for_matrix,
    } = args;
    // the extractor validated the endianness, so it's safe to use `from_u8_unchecked`
    let endianness = unsafe { Endianness::from_u8_unchecked(endianness) };
    let res: [f64; 9] = match mat_props::elastic_modules_for_unidirectional_composite(
        number_of_model,
        fibre_content,
        e_for_fiber,
        nu_for_fiber,
        e_for_matrix,
        nu_for_matrix,
    ) {
        Some(r) => r,
        None => return actix_web::HttpResponse::InternalServerError().finish(),
    };
    let [e1, e2, e3, nu12, nu13, nu23, g12, g13, g23] = res;
    let message = ElasticModulesForUnidirectionalCompositeResponseMessage {
        e1,
        e2,
        e3,
        nu12,
        nu13,
        nu23,
        g12,
        g13,
        g23,
    };
    let parcel = ElasticModulesForUnidirectionalCompositeResponseParcel::new(endianness, message);
    actix_web::HttpResponse::Ok().body(parcel)
}

async fn serve_openapi_json() -> impl Responder {
    let json = ApiDoc::openapi().to_pretty_json().unwrap();
    actix_web::HttpResponse::Ok().body(json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if cfg!(target_endian = "little") {
        println!("Endianness: little.");
    } else {
        println!("Endianness: big.");
    };
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(elastic_modules_for_unidirectional_composite)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .route("/openapi.json", web::get().to(serve_openapi_json))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {

    #[test]
    fn elastic_modules_for_unidirectional_composite() {
        let res =
            mat_props::elastic_modules_for_unidirectional_composite(2, 0.2, 100.0, 0.3, 5.0, 0.2);
        assert_eq!(
            res,
            Some([
                40.01172332942556,
                6.7364802254566305,
                6.7364802254566305,
                0.03840958253366131,
                0.03840958253366131,
                0.21620579415556423,
                2.9945407835581253,
                2.9945407835581253,
                2.769465602708258
            ])
        );
    }

    #[test]
    fn see_args_as_bytes() {
        let args = crate::ElasticModulesForUnidirectionalCompositeArgsMessage {
            endianness: 0,
            number_of_model: 2,
            fibre_content: 0.2,
            e_for_fiber: 100.0,
            nu_for_fiber: 0.3,
            e_for_matrix: 5.0,
            nu_for_matrix: 0.2,
        };
        let bytes = args.into_bytes();

        println!("Args as bytes: {:?}", bytes);

        assert_eq!(bytes[0..=0], [0], "Endianness");
        assert_eq!(bytes[1..=1], [2], "Number of model");
        // bytes[2..=7] is the padding
        assert_eq!(
            bytes[8..=15],
            {
                let mut arr = [154, 153, 153, 153, 153, 153, 201, 63];
                if cfg!(target_endian = "big") {
                    arr.reverse();
                };
                arr
            },
            "Fibre content"
        );
        assert_eq!(bytes[16..=23], [0, 0, 0, 0, 0, 0, 89, 64], "E for fiber");
        assert_eq!(
            bytes[24..=31],
            {
                let mut arr = [51, 51, 51, 51, 51, 51, 211, 63];
                if cfg!(target_endian = "big") {
                    arr.reverse();
                };
                arr
            },
            "Nu for fiber"
        );
        assert_eq!(
            bytes[32..=39],
            {
                let mut arr = [0, 0, 0, 0, 0, 0, 20, 64];
                if cfg!(target_endian = "big") {
                    arr.reverse();
                };
                arr
            },
            "E for matrix"
        );
        assert_eq!(
            bytes[40..=47],
            {
                let mut arr = [154, 153, 153, 153, 153, 153, 201, 63];
                if cfg!(target_endian = "big") {
                    arr.reverse();
                };
                arr
            },
            "Nu for matrix"
        );
    }

    #[test]
    fn check_response() {
        assert_eq!(
            [211, 61, 106, 38, 128, 1, 68, 64],
            u64::to_le_bytes(f64::to_bits(40.01172332942556)),
        );
        assert_eq!(
            [159, 242, 73, 223, 39, 242, 26, 64],
            u64::to_le_bytes(f64::to_bits(6.7364802254566305)),
        );
        assert_eq!(
            [159, 242, 73, 223, 39, 242, 26, 64],
            u64::to_le_bytes(f64::to_bits(6.7364802254566305)),
        );
        assert_eq!(
            [172, 144, 171, 185, 107, 170, 163, 63],
            u64::to_le_bytes(f64::to_bits(0.03840958253366131)),
        );
        assert_eq!(
            [172, 144, 171, 185, 107, 170, 163, 63],
            u64::to_le_bytes(f64::to_bits(0.03840958253366131)),
        );
        assert_eq!(
            [42, 75, 141, 167, 161, 172, 203, 63],
            u64::to_le_bytes(f64::to_bits(0.21620579415556423)),
        );
        assert_eq!(
            [231, 92, 95, 204, 209, 244, 7, 64],
            u64::to_le_bytes(f64::to_bits(2.9945407835581253)),
        );
        assert_eq!(
            [231, 92, 95, 204, 209, 244, 7, 64],
            u64::to_le_bytes(f64::to_bits(2.9945407835581253)),
        );
        assert_eq!(
            [46, 59, 248, 148, 221, 39, 6, 64],
            u64::to_le_bytes(f64::to_bits(2.769465602708258)),
        );
    }

    #[test]
    fn check_args_message_size() {
        assert_eq!(
            core::mem::size_of::<crate::ElasticModulesForUnidirectionalCompositeArgsMessage>(),
            48
        );
    }

    #[test]
    fn check_py_struct_format_string_for_args_message() {
        let s =
            crate::ElasticModulesForUnidirectionalCompositeArgsMessage::to_py_struct_format_string(
            );
        assert_eq!(s, "BBxxxxxxddddd");
    }
}
