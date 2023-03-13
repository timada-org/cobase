use actix_jwks::JwtPayload;
use actix_web::{post, web, HttpResponse, Scope};
use cobase::command::CommandInput;
use cobase::warehouse;
use evento::CommandResponse;
use serde::Deserialize;
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};

use crate::AppState;

#[derive(Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ImportDataInput {
    #[schema(value_type = Vec<Object>)]
    pub data: Vec<Value>,
}

#[utoipa::path(
    tag = "cobase",
    context_path = "/api/warehouses",
    request_body=ImportDataInput,
    responses(
        (status = 200, description = "Import data to wharehouse did not result error", body = CommandResponse),
    )
)]
#[post("/import-data")]
async fn import_data(
    state: web::Data<AppState>,
    input: web::Json<ImportDataInput>,
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
    web::scope("/warehouses").service(import_data)
}
