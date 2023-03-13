use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa::{openapi, OpenApi};

use crate::room;
use crate::warehouse;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CommandResponse {
    #[schema(example = "V1StGXR8_Z5jdHi6B-myT")]
    pub id: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(room::create_room, room::list_rooms, warehouse::import_data),
    components(schemas(room::Room, room::CreateInput, warehouse::ImportDataInput, CommandResponse)),
    tags(
        (name = "Cobase", description = "Cobase api endpoints.")
    )
)]
pub struct ApiDoc;

#[get("/openapi.json")]
async fn service(api_doc: web::Data<openapi::OpenApi>) -> HttpResponse {
    HttpResponse::Ok().json(api_doc.as_ref())
}
