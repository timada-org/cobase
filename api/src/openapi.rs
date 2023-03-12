use actix_web::{get, web, HttpResponse};
use cobase::command;
use utoipa::{openapi, OpenApi};

use crate::room;

#[derive(OpenApi)]
#[openapi(
    paths(room::create_room, room::list_rooms),
    components(schemas(cobase::room::Room, cobase::room::CreateCommand, command::CommandJsonResponse)),
    tags(
        (name = "Cobase", description = "Cobase api endpoints.")
    )
)]
pub struct ApiDoc;

#[get("/openapi.json")]
async fn service(api_doc: web::Data<openapi::OpenApi>) -> HttpResponse {
    HttpResponse::Ok().json(api_doc.as_ref())
}
