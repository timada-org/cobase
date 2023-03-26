use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::{openapi, OpenApi};
use utoipa::{IntoParams, ToSchema};

use crate::room;
use crate::warehouse;

use crate::warehouse::WarehouseData;

#[derive(Default, Serialize, Deserialize, Debug, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct QueryArgs {
    #[param(required = false)]
    pub first: Option<u16>,
    #[param(required = false)]
    pub after: Option<String>,
    #[param(required = false)]
    pub last: Option<u16>,
    #[param(required = false)]
    pub before: Option<String>,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[aliases(QueryResultWarehouseData = QueryResult<EdgeWarehouseData>)]
pub struct QueryResult<N> {
    pub edges: Vec<N>,
    pub page_info: PageInfo,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[aliases(EdgeWarehouseData = Edge<WarehouseData>)]
pub struct Edge<N> {
    pub cursor: String,
    pub node: N,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PageInfo {
    pub has_previous_page: bool,
    pub has_next_page: bool,
    pub start_cursor: Option<String>,
    pub end_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CommandResponse {
    #[schema(example = "V1StGXR8_Z5jdHi6B-myT")]
    pub id: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(room::create_room, room::list_rooms, warehouse::import_data, warehouse::list_warehouses_data),
    components(schemas(room::Room, room::CreateRoomInput, warehouse::ImportDataWarehouseInput, WarehouseData, CommandResponse, QueryResultWarehouseData, PageInfo, EdgeWarehouseData)),
    tags(
        (name = "Cobase", description = "Cobase api endpoints.")
    )
)]
pub struct ApiDoc;

#[get("/openapi.json")]
async fn service(api_doc: web::Data<openapi::OpenApi>) -> HttpResponse {
    HttpResponse::Ok().json(api_doc.as_ref())
}
