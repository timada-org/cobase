use actix_web::{get, web, HttpResponse};
use utoipa::{openapi, OpenApi};

use crate::group;
use crate::command;

#[derive(OpenApi)]
#[openapi(
    paths(group::create_group, group::list_groups),
    components(schemas(group::Group, group::CreateCommand, command::CommandJsonResponse)),
    tags(
        (name = "Cobase", description = "Cobase api endpoints.")
    )
)]
pub struct ApiDoc;

#[get("/openapi.json")]
async fn service(api_doc: web::Data<openapi::OpenApi>) -> HttpResponse {
    HttpResponse::Ok().json(api_doc.as_ref())
}
