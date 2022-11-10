use actix_web::{get, web, HttpResponse};
use utoipa::{OpenApi, openapi};

use crate::group;

#[derive(OpenApi)]
#[openapi(
    paths(
        group::create,
    ),
    components(
        schemas(group::Group, group::CreateCommand)
    )
)]
pub struct ApiDoc;

#[get("/openapi.json")]
async fn service(api_doc: web::Data<openapi::OpenApi>) -> HttpResponse {
    HttpResponse::Ok().json(api_doc.as_ref())
}
