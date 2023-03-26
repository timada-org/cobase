use std::collections::HashMap;

use actix_jwks::JwtPayload;
use actix_web::{get, post, web, HttpResponse, Scope};
use chrono::{DateTime, Utc};
use cobase::command::CommandInput;
use cobase::warehouse;
use evento::{query::QueryArgs, CommandError, CommandResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::AppState;

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, ToSchema)]
pub struct WarehouseData {
    #[schema(example = "V1StGXR8_Z5jdHi6B-myT")]
    pub id: String,
    #[schema(example = "your-custom-key")]
    pub key: String,
    #[schema(value_type = Vec<Object>, example = "{\"name\": \"My name\"}")]
    pub data: Value,
    #[schema(value_type = String, example = "2023-03-26T02:57:08.590084Z")]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = Option<String>, example = "2023-03-26T02:57:08.590084Z")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[utoipa::path(
    tag = "cobase",
    context_path = "/api/warehouses",
    params(
        crate::openapi::QueryArgs
    ),
    responses(
        (status = 200, description = "Get warehouse data did not result error", body = QueryResultWarehouseData),
    )
)]
#[get("/data")]
async fn list_warehouses_data(
    state: web::Data<AppState>,
    payload: JwtPayload,
    query_args: web::Query<QueryArgs>,
) -> Result<HttpResponse, CommandError> {
    let rows = state
        .query
        .send(warehouse::ListWarehouseDataQuery {
            user_id: Uuid::parse_str(&payload.subject)?,
            query_args: query_args.0,
        })
        .await??;

    Ok(HttpResponse::Ok().json(rows))
}

#[derive(Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ImportDataWarehouseInput {
    #[schema(value_type = Vec<Object>, example = "[{ \"_id\": 1, \"name\": \"john doe\" }]")]
    pub data: Vec<HashMap<String, Value>>,
}

#[utoipa::path(
    tag = "cobase",
    context_path = "/api/warehouses",
    request_body=ImportDataWarehouseInput,
    responses(
        (status = 200, description = "Import data to wharehouse did not result error", body = CommandResponse),
    )
)]
#[post("/import-data")]
async fn import_data(
    state: web::Data<AppState>,
    input: web::Json<ImportDataWarehouseInput>,
    payload: JwtPayload,
) -> HttpResponse {
    CommandResponse(
        state
            .cmd
            .send(CommandInput {
                user_id: payload.subject,
                input: warehouse::ImportDataCommand { data: input.0.data },
            })
            .await,
    )
    .into()
}

pub fn scope() -> Scope {
    web::scope("/warehouses")
        .service(list_warehouses_data)
        .service(import_data)
}
