use actix_web::{get, App, HttpServer, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(index,))]
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index).service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
