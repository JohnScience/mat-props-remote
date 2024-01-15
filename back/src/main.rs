use actix_web::{get, post, App, HttpServer, Responder};
use dotenv::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod endianness;
mod proto;
use endianness::Endianness;
use proto::{
    ElasticModulesForHoneycombArgsMessage, ElasticModulesForHoneycombResponseMessage,
    ElasticModulesForHoneycombResponseParcel, ElasticModulesForUnidirectionalCompositeArgsMessage,
    ElasticModulesForUnidirectionalCompositeResponseMessage,
    ElasticModulesForUnidirectionalCompositeResponseParcel,
    ThermalConductivityForUnidirectionalCompositeArgsMessage,
    ThermalConductivityForUnidirectionalCompositeResponseMessage,
    ThermalConductivityForUnidirectionalCompositeResponseParcel,
    ThermalExpansionForHoneycombArgsMessage, ThermalExpansionForHoneycombResponseMessage,
    ThermalExpansionForHoneycombResponseParcel,
    ThermalExpansionForUnidirectionalCompositeArgsMessage,
    ThermalExpansionForUnidirectionalCompositeResponseMessage,
    ThermalExpansionForUnidirectionalCompositeResponseParcel,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        index,
        elastic_modules_for_unidirectional_composite,
        elastic_modules_for_honeycomb,
        thermal_conductivity_for_unidirectional_composite,
        thermal_expansion_for_unidirectional_composite,
        thermal_expansion_for_honeycomb,
    ),
    components(schemas(
        ElasticModulesForUnidirectionalCompositeArgsMessage,
        ElasticModulesForUnidirectionalCompositeResponseMessage,
        ElasticModulesForHoneycombArgsMessage,
        ElasticModulesForHoneycombResponseMessage,
        ThermalConductivityForUnidirectionalCompositeArgsMessage,
        ThermalConductivityForUnidirectionalCompositeResponseMessage,
        ThermalExpansionForUnidirectionalCompositeArgsMessage,
        ThermalExpansionForUnidirectionalCompositeResponseMessage,
        ThermalExpansionForHoneycombArgsMessage,
        ThermalExpansionForHoneycombResponseMessage,
    ))
)]
struct ApiDoc;

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
        description = format!(
            "Python struct format string: {:?}. See <https://docs.python.org/3/library/struct.html#format-strings>.\n\n\
            See schema for the order of the fields (but not their sizes).",
            ElasticModulesForUnidirectionalCompositeArgsMessage::py_struct_format_string()
        ),
        content_type = ElasticModulesForUnidirectionalCompositeArgsMessage::content_type(),
        example = ElasticModulesForUnidirectionalCompositeArgsMessage::example_as_serde_big_array,
    ),
    responses (
        (
            status = 200,
            description = format!(
                "Computes elastic_modules_for_unidirectional_composite. \
                Returns the binary representation of [E1, E2, E3, nu12, nu13, nu23, G12, G13, G23] with the requested endianness.\n\n\
                Python struct format string: {:?}. See <https://docs.python.org/3/library/struct.html#format-strings>.",
                ElasticModulesForUnidirectionalCompositeResponseMessage::py_struct_format_string()
            ),
            body = ElasticModulesForUnidirectionalCompositeResponseMessage,
            content_type = ElasticModulesForUnidirectionalCompositeResponseMessage::content_type(),
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
        Ok(r) => r,
        Err(_e) => return actix_web::HttpResponse::InternalServerError().finish(),
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
    actix_web::HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(parcel)
}

#[utoipa::path(
    post,
    request_body(
        content = ElasticModulesForHoneycombArgsMessage,
        description = format!(
            "Python struct format string: {:?}. See <https://docs.python.org/3/library/struct.html#format-strings>.\n\n\
            See schema for the order of the fields (but not their sizes).",
            ElasticModulesForHoneycombArgsMessage::py_struct_format_string()
        ),
        content_type = ElasticModulesForHoneycombArgsMessage::content_type(),
        example = ElasticModulesForHoneycombArgsMessage::example_as_serde_big_array,
    ),
    responses (
        (
            status = 200,
            description = format!(
                "Computes elastic_modules_for_honeycomb. \
                Returns the binary representation of [E1, E2, E3, nu12, nu13, nu23, G12, G13, G23] with the requested endianness.\n\n\
                Python struct format string: {:?}. See <https://docs.python.org/3/library/struct.html#format-strings>.",
                ElasticModulesForHoneycombResponseMessage::py_struct_format_string()
            ),
            body = ElasticModulesForHoneycombResponseMessage,
            content_type = ElasticModulesForHoneycombResponseMessage::content_type(),
        ),
    )
)]
#[post("/compute/elastic_modules_for_honeycomb")]
async fn elastic_modules_for_honeycomb(
    args: ElasticModulesForHoneycombArgsMessage,
) -> impl Responder {
    let ElasticModulesForHoneycombArgsMessage {
        endianness,
        number_of_model,
        l_cell_side_size,
        h_cell_side_size,
        wall_thickness,
        angle,
        e_for_honeycomb,
        nu_for_honeycomb,
    } = args;
    // the extractor validated the endianness, so it's safe to use `from_u8_unchecked`
    let endianness = unsafe { Endianness::from_u8_unchecked(endianness) };
    let res: [f64; 9] = match mat_props::elastic_modules_for_honeycomb(
        number_of_model,
        l_cell_side_size,
        h_cell_side_size,
        wall_thickness,
        angle,
        e_for_honeycomb,
        nu_for_honeycomb,
    ) {
        Ok(r) => r,
        Err(_e) => return actix_web::HttpResponse::InternalServerError().finish(),
    };
    let [e1, e2, e3, nu12, nu13, nu23, g12, g13, g23] = res;
    let message = ElasticModulesForHoneycombResponseMessage {
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
    let parcel = ElasticModulesForHoneycombResponseParcel::new(endianness, message);
    actix_web::HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(parcel)
}

#[utoipa::path(
    post,
    request_body(
        content = ThermalConductivityForUnidirectionalCompositeArgsMessage,
        description = format!(
            "Python struct format string: {:?}. See <https://docs.python.org/3/library/struct.html#format-strings>.\n\n\
            See schema for the order of the fields (but not their sizes).",
            ThermalConductivityForUnidirectionalCompositeArgsMessage::py_struct_format_string()
        ),
        content_type = ThermalConductivityForUnidirectionalCompositeArgsMessage::content_type(),
        example = ThermalConductivityForUnidirectionalCompositeArgsMessage::example_as_serde_big_array,
    ),
    responses (
        (
            status = 200,
            description = format!(
                "Computes thermal_conductivity_for_unidirectional_composite. \
                Returns the binary representation of [K1,K2,K3] with the requested endianness.\n\n\
                Python struct format string: {:?}. See <https://docs.python.org/3/library/struct.html#format-strings>.",
                ThermalConductivityForUnidirectionalCompositeResponseMessage::py_struct_format_string()
            ),
            body = ThermalConductivityForUnidirectionalCompositeResponseMessage,
            content_type = ThermalConductivityForUnidirectionalCompositeResponseMessage::content_type(),
        ),
    )
)]
#[post("/compute/thermal_conductivity_for_unidirectional_composite")]
async fn thermal_conductivity_for_unidirectional_composite(
    args: ThermalConductivityForUnidirectionalCompositeArgsMessage,
) -> impl Responder {
    let ThermalConductivityForUnidirectionalCompositeArgsMessage {
        endianness,
        number_of_model,
        fibre_content,
        k_for_fiber,
        k_for_matrix,
    } = args;
    // the extractor validated the endianness, so it's safe to use `from_u8_unchecked`
    let endianness = unsafe { Endianness::from_u8_unchecked(endianness) };
    let res: [f64; 3] = match mat_props::thermal_conductivity_for_unidirectional_composite(
        number_of_model,
        fibre_content,
        k_for_fiber,
        k_for_matrix,
    ) {
        Ok(r) => r,
        Err(_e) => return actix_web::HttpResponse::InternalServerError().finish(),
    };
    let [k1, k2, k3] = res;
    let message = ThermalConductivityForUnidirectionalCompositeResponseMessage { k1, k2, k3 };
    let parcel =
        ThermalConductivityForUnidirectionalCompositeResponseParcel::new(endianness, message);
    actix_web::HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(parcel)
}

#[utoipa::path(
    post,
    request_body(
        content = ThermalExpansionForUnidirectionalCompositeArgsMessage,
        description = format!(
            "Python struct format string: {:?}. See <https://docs.python.org/3/library/struct.html#format-strings>.\n\n\
            See schema for the order of the fields (but not their sizes).",
            ThermalExpansionForUnidirectionalCompositeArgsMessage::py_struct_format_string()
        ),
        content_type = ThermalExpansionForUnidirectionalCompositeArgsMessage::content_type(),
        example = ThermalExpansionForUnidirectionalCompositeArgsMessage::example_as_serde_big_array,
    ),
    responses (
        (
            status = 200,
            description = format!(
                "Computes thermal_expansion_for_unidirectional_composite. \
                Returns the binary representation of [alpha1,alpha2,alpha3] with the requested endianness.\n\n\
                Python struct format string: {:?}. See <https://docs.python.org/3/library/struct.html#format-strings>.",
                ThermalExpansionForUnidirectionalCompositeResponseMessage::py_struct_format_string()
            ),
            body = ThermalExpansionForUnidirectionalCompositeResponseMessage,
            content_type = ThermalExpansionForUnidirectionalCompositeResponseMessage::content_type(),
        ),
    )
)]
#[post("/compute/thermal_expansion_for_unidirectional_composite")]
async fn thermal_expansion_for_unidirectional_composite(
    args: ThermalExpansionForUnidirectionalCompositeArgsMessage,
) -> impl Responder {
    let ThermalExpansionForUnidirectionalCompositeArgsMessage {
        endianness,
        number_of_model,
        fibre_content,
        e_for_fiber,
        nu_for_fiber,
        alpha_for_fiber,
        e_for_matrix,
        nu_for_matrix,
        alpha_for_matrix,
    } = args;
    // the extractor validated the endianness, so it's safe to use `from_u8_unchecked`
    let endianness = unsafe { Endianness::from_u8_unchecked(endianness) };
    let res: [f64; 3] = match mat_props::thermal_expansion_for_unidirectional_composite(
        number_of_model,
        fibre_content,
        e_for_fiber,
        nu_for_fiber,
        alpha_for_fiber,
        e_for_matrix,
        nu_for_matrix,
        alpha_for_matrix,
    ) {
        Ok(r) => r,
        Err(_e) => return actix_web::HttpResponse::InternalServerError().finish(),
    };
    let [alpha1, alpha2, alpha3] = res;
    let message = ThermalExpansionForUnidirectionalCompositeResponseMessage {
        alpha1,
        alpha2,
        alpha3,
    };
    let parcel = ThermalExpansionForUnidirectionalCompositeResponseParcel::new(endianness, message);
    actix_web::HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(parcel)
}

#[utoipa::path(
    post,
    request_body(
        content = ThermalExpansionForHoneycombArgsMessage,
        description = format!(
            "Python struct format string: {:?}. See <https://docs.python.org/3/library/struct.html#format-strings>.\n\n\
            See schema for the order of the fields (but not their sizes).",
            ThermalExpansionForHoneycombArgsMessage::py_struct_format_string()
        ),
        content_type = ThermalExpansionForHoneycombArgsMessage::content_type(),
        example = ThermalExpansionForHoneycombArgsMessage::example_as_serde_big_array,
    ),
    responses (
        (
            status = 200,
            description = format!(
                "Computes thermal_expansion_for_honeycomb. \
                Returns the binary representation of [alpha1,alpha2,alpha3] with the requested endianness.\n\n\
                Python struct format string: {:?}. See <https://docs.python.org/3/library/struct.html#format-strings>.",
                ThermalExpansionForHoneycombResponseMessage::py_struct_format_string()
            ),
            body = ThermalExpansionForHoneycombResponseMessage,
            content_type = ThermalExpansionForHoneycombResponseMessage::content_type(),
        ),
    )
)]
#[post("/compute/thermal_expansion_for_honeycomb")]
async fn thermal_expansion_for_honeycomb(
    args: ThermalExpansionForHoneycombArgsMessage,
) -> impl Responder {
    let ThermalExpansionForHoneycombArgsMessage {
        endianness,
        number_of_model,
        l_cell_side_size,
        h_cell_side_size,
        _wall_thickness,
        angle,
        alpha_for_honeycomb,
    } = args;
    // the extractor validated the endianness, so it's safe to use `from_u8_unchecked`
    let endianness = unsafe { Endianness::from_u8_unchecked(endianness) };
    let res: [f64; 3] = match mat_props::thermal_expansion_for_honeycomb(
        number_of_model,
        l_cell_side_size,
        h_cell_side_size,
        _wall_thickness,
        angle,
        alpha_for_honeycomb,
    ) {
        Ok(r) => r,
        Err(_e) => return actix_web::HttpResponse::InternalServerError().finish(),
    };
    let [alpha1, alpha2, alpha3] = res;
    let message = ThermalExpansionForHoneycombResponseMessage {
        alpha1,
        alpha2,
        alpha3,
    };
    let parcel = ThermalExpansionForHoneycombResponseParcel::new(endianness, message);
    actix_web::HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "*"))
        .body(parcel)
}

#[post("/api-doc/openapi.json")]
async fn serve_openapi_json() -> impl Responder {
    let json = ApiDoc::openapi().to_pretty_json().unwrap();
    actix_web::HttpResponse::Ok().body(json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().unwrap();
    println!(
        "Endianness: {}.",
        if cfg!(target_endian = "big") {
            "big"
        } else {
            "little"
        }
    );
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(elastic_modules_for_unidirectional_composite)
            .service(elastic_modules_for_honeycomb)
            .service(thermal_conductivity_for_unidirectional_composite)
            .service(thermal_expansion_for_unidirectional_composite)
            .service(thermal_expansion_for_honeycomb)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
            .service(serve_openapi_json)
    })
    .bind((std::env::var("IP_ADDR").unwrap(), 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use crate::proto::ElasticModulesForUnidirectionalCompositeArgsMessage;

    #[test]
    fn see_args_as_bytes() {
        let args = ElasticModulesForUnidirectionalCompositeArgsMessage {
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
}
