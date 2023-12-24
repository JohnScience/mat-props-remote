use actix_web::{get, post, App, HttpServer, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(index, elastic_modules_for_unidirectional_composite))]
struct ApiDoc;

mod elastic_modules_for_unidirectional_composite_extra {
    use bytemuck::AnyBitPattern;
    use futures_util::{Future, StreamExt};
    use std::pin::Pin;

    #[derive(AnyBitPattern, Clone, Copy)]
    #[repr(C)]
    pub(crate) struct Args {
        pub(crate) number_of_model: u8,
        pub(crate) fibre_content: f64,
        pub(crate) e_for_fiber: f64,
        pub(crate) nu_for_fiber: f64,
        pub(crate) e_for_matrix: f64,
        pub(crate) nu_for_matrix: f64,
    }

    impl Args {
        const SIZE: usize = std::mem::size_of::<Self>();

        #[cfg(test)]
        pub(crate) fn into_bytes(self) -> [u8; Self::SIZE] {
            unsafe { std::mem::transmute::<Self, [u8; Self::SIZE]>(self) }
        }
    }

    // TODO: write an implementation for any type that implements `AnyBitPattern` and suggest it for actix_web
    impl actix_web::FromRequest for Args {
        type Error = actix_web::Error;
        type Future = Pin<Box<dyn Future<Output = Result<Args, actix_web::Error>>>>;

        fn from_request(
            req: &actix_web::HttpRequest,
            payload: &mut actix_web::dev::Payload,
        ) -> Self::Future {
            let payload = actix_web::web::Payload::from_request(req, payload);
            Box::pin(async move {
                let payload = payload.await;
                match payload {
                    Ok(mut p) => {
                        let mut buf = heapless::Vec::<u8, { Args::SIZE }>::new();
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
                        let Ok(buf) = buf.into_array::<{ Args::SIZE }>() else {
                            return Err(actix_web::error::ErrorBadRequest("Args buffer overflow"));
                        };
                        let args = unsafe { std::mem::transmute::<_, Args>(buf) };
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
    args: elastic_modules_for_unidirectional_composite_extra::Args,
) -> impl Responder {
    let elastic_modules_for_unidirectional_composite_extra::Args {
        number_of_model,
        fibre_content,
        e_for_fiber,
        nu_for_fiber,
        e_for_matrix,
        nu_for_matrix,
    } = args;
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
    let bytes = actix_web::web::Bytes::copy_from_slice(bytemuck::bytes_of(&res));
    // send as binary data
    actix_web::HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(bytes)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
        let args = crate::elastic_modules_for_unidirectional_composite_extra::Args {
            number_of_model: 2,
            fibre_content: 0.2,
            e_for_fiber: 100.0,
            nu_for_fiber: 0.3,
            e_for_matrix: 5.0,
            nu_for_matrix: 0.2,
        };
        let bytes = args.into_bytes();
        assert_eq!(
            bytes,
            [
                2, 0, 0, 0, 0, 0, 0, 0, 154, 153, 153, 153, 153, 153, 201, 63, 0, 0, 0, 0, 0, 0,
                89, 64, 51, 51, 51, 51, 51, 51, 211, 63, 0, 0, 0, 0, 0, 0, 20, 64, 154, 153, 153,
                153, 153, 153, 201, 63
            ]
        )
    }

    #[test]
    fn check_response() {
        let arr: [u8; 72] = [
            211, 61, 106, 38, 128, 1, 68, 64, 159, 242, 73, 223, 39, 242, 26, 64, 159, 242, 73,
            223, 39, 242, 26, 64, 172, 144, 171, 185, 107, 170, 163, 63, 172, 144, 171, 185, 107,
            170, 163, 63, 42, 75, 141, 167, 161, 172, 203, 63, 231, 92, 95, 204, 209, 244, 7, 64,
            231, 92, 95, 204, 209, 244, 7, 64, 46, 59, 248, 148, 221, 39, 6, 64,
        ];
        let res: &[f64; 9] = bytemuck::from_bytes(&arr);
        assert_eq!(
            res,
            &[
                40.01172332942556,
                6.7364802254566305,
                6.7364802254566305,
                0.03840958253366131,
                0.03840958253366131,
                0.21620579415556423,
                2.9945407835581253,
                2.9945407835581253,
                2.769465602708258
            ]
        );
    }
}
