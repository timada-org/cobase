use actix_web::{get, web, HttpResponse};
use utoipa::{openapi, OpenApi};

use crate::group;

#[derive(OpenApi)]
#[openapi(
    paths(group::create, group::find_all),
    components(schemas(group::Group, group::CreateCommand))
)]
pub struct ApiDoc;

#[get("/openapi.json")]
async fn service(api_doc: web::Data<openapi::OpenApi>) -> HttpResponse {
    HttpResponse::Ok().json(api_doc.as_ref())
}
