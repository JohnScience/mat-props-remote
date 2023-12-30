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
