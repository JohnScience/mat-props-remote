use actix_web::{get, post, App, HttpServer, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod endianness;

use endianness::Endianness;

#[derive(OpenApi)]
#[openapi(paths(index, elastic_modules_for_unidirectional_composite))]
struct ApiDoc;

mod elastic_modules_for_unidirectional_composite_extra {
    use crate::Endianness;
    use bytemuck::AnyBitPattern;
    use futures_util::{Future, StreamExt};
    use std::pin::Pin;

    #[derive(AnyBitPattern, Clone, Copy)]
    #[repr(C)]
    pub(crate) struct ArgsMessage {
        // 0: little endian, 1: big endian
        pub(crate) endianness: u8,
        pub(crate) number_of_model: u8,
        pub(crate) fibre_content: f64,
        pub(crate) e_for_fiber: f64,
        pub(crate) nu_for_fiber: f64,
        pub(crate) e_for_matrix: f64,
        pub(crate) nu_for_matrix: f64,
    }

    impl ArgsMessage {
        const SIZE: usize = std::mem::size_of::<Self>();

        #[cfg(test)]
        #[inline]
        pub(crate) fn into_bytes(self) -> [u8; Self::SIZE] {
            debug_assert!(core::mem::size_of::<Self>() == core::mem::size_of::<[u8; Self::SIZE]>());
            unsafe { std::mem::transmute::<Self, [u8; Self::SIZE]>(self) }
        }

        #[inline]
        pub(crate) fn endianness(&self) -> Option<Endianness> {
            Endianness::try_from_u8(self.endianness)
        }

        pub(crate) fn reorder_bytes(&mut self) {
            let ArgsMessage {
                endianness: _,
                number_of_model: _,
                fibre_content,
                e_for_fiber,
                nu_for_fiber,
                e_for_matrix,
                nu_for_matrix,
            } = self;
            *fibre_content = f64::from_bits(fibre_content.to_bits().swap_bytes());
            *e_for_fiber = f64::from_bits(e_for_fiber.to_bits().swap_bytes());
            *nu_for_fiber = f64::from_bits(nu_for_fiber.to_bits().swap_bytes());
            *e_for_matrix = f64::from_bits(e_for_matrix.to_bits().swap_bytes());
            *nu_for_matrix = f64::from_bits(nu_for_matrix.to_bits().swap_bytes());
        }
    }

    // TODO: an implementation for any type that implements `AnyBitPattern` and suggest it for actix_web
    impl actix_web::FromRequest for ArgsMessage {
        type Error = actix_web::Error;
        type Future = Pin<Box<dyn Future<Output = Result<ArgsMessage, actix_web::Error>>>>;

        fn from_request(
            req: &actix_web::HttpRequest,
            payload: &mut actix_web::dev::Payload,
        ) -> Self::Future {
            let payload = actix_web::web::Payload::from_request(req, payload);
            Box::pin(async move {
                let payload = payload.await;
                match payload {
                    Ok(mut p) => {
                        let mut buf =
                            heapless::AlignedVec::<u8, ArgsMessage, { ArgsMessage::SIZE }>::new();
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
                        let mut args: ArgsMessage = unsafe { buf.transmute_buffer() };
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
}

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
    get,
    request_body = elastic_modules_for_unidirectional_composite_extra::Args,
    responses (
        (status = 200, description = "Computes elastic_modules_for_unidirectional_composite", content_type = "application/octet-stream"),
    )
)]
#[post("/compute/elastic_modules_for_unidirectional_composite")]
async fn elastic_modules_for_unidirectional_composite(
    args: elastic_modules_for_unidirectional_composite_extra::ArgsMessage,
) -> impl Responder {
    let elastic_modules_for_unidirectional_composite_extra::ArgsMessage {
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
    let bytes = if endianness == Endianness::NATIVE {
        let iter = bytemuck::bytes_of(&res).iter().copied();
        actix_web::web::Bytes::from_iter(iter)
    } else {
        let iter = res
            .iter()
            .copied()
            .map(|f| f.to_bits().swap_bytes())
            .flat_map(|u| u.to_ne_bytes());
        actix_web::web::Bytes::from_iter(iter)
    };
    // send as binary data
    actix_web::HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(bytes)
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
        let args = crate::elastic_modules_for_unidirectional_composite_extra::ArgsMessage {
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
}
