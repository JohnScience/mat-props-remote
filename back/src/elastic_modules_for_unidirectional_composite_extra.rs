use crate::Endianness;
use actix_web::body::MessageBody;
use bytemuck::{Pod, Zeroable};
use futures_util::{Future, StreamExt};
use std::pin::Pin;
use utoipa::ToSchema;

pub(crate) use crate::proto::ElasticModulesForUnidirectionalCompositeArgsMessage;

#[derive(Clone, Copy, ToSchema, Pod, Zeroable)]
#[repr(C)]
#[schema(example = ElasticModulesForUnidirectionalCompositeResponseMessage::example_as_serde_big_array)]
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

impl ElasticModulesForUnidirectionalCompositeResponseMessage {
    pub(crate) const SIZE: usize = core::mem::size_of::<Self>();

    pub(crate) const fn content_type() -> &'static str {
        "application/x.elastic-modules-for-unidirectional-composite-response-message"
    }

    pub(crate) const fn py_struct_format_string() -> &'static str {
        "ddddddddd"
    }

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

    pub(crate) const fn example_as_array() -> [u8; Self::SIZE] {
        unsafe { core::mem::transmute(Self::example()) }
    }

    pub(crate) fn example_as_serde_big_array() -> serde_big_array::Array<u8, { Self::SIZE }> {
        serde_big_array::Array(Self::example_as_array())
    }
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
