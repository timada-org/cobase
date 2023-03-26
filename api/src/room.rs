use actix_jwks::JwtPayload;
use actix_web::{get, post, web, HttpResponse, Scope};
use cobase::command::CommandInput;
use cobase::room;
use evento::{CommandError, CommandResponse};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::AppState;

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, ToSchema)]
pub struct Room {
    #[schema(example = "V1StGXR8_Z5jdHi6B-myT")]
    pub id: String,
    #[schema(example = "My room name 1")]
    pub name: String,
    #[schema(example = "a18aac51-6262-4576-8883-7fda0ca72aac")]
    pub user_id: Uuid,
}

#[utoipa::path(
    tag = "cobase",
    context_path = "/api/rooms",
    responses(
        (status = 200, description = "Get rooms did not result error", body = [Room]),
    )
)]
#[get("")]
async fn list_rooms(
    state: web::Data<AppState>,
    payload: JwtPayload,
) -> Result<HttpResponse, CommandError> {
    let rooms = state
        .query
        .send(room::ListRoomsQuery {
            user_id: Uuid::parse_str(&payload.subject)?,
        })
        .await??;

    Ok(HttpResponse::Ok().json(rooms))
}

#[derive(Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoomInput {
    #[schema(example = "My room name 1")]
    pub name: String,
}

#[utoipa::path(
    tag = "cobase",
    context_path = "/api/rooms",
    request_body=CreateRoomInput,
    responses(
        (status = 200, description = "Create room did not result error", body = CommandResponse),
    )
)]
#[post("/create")]
async fn create_room(
    state: web::Data<AppState>,
    input: web::Json<CreateRoomInput>,
    payload: JwtPayload,
) -> HttpResponse {
    CommandResponse(
        state
            .cmd
            .send(CommandInput {
                user_id: payload.subject,
                input: room::CreateCommand { name: input.0.name },
            })
            .await,
    )
    .into()
}

pub fn scope() -> Scope {
    web::scope("/rooms")
        .service(list_rooms)
        .service(create_room)
}
