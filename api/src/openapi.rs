use actix_web::{get, web, HttpResponse};
use utoipa::{openapi, OpenApi};

use crate::group;
use crate::command;

#[derive(OpenApi)]
#[openapi(
    paths(group::create, group::find_all),
    components(schemas(group::Group, group::CreateCommand, command::JsonResponse))
)]
pub struct ApiDoc;

#[get("/openapi.json")]
async fn service(api_doc: web::Data<openapi::OpenApi>) -> HttpResponse {
    HttpResponse::Ok().json(api_doc.as_ref())
}
