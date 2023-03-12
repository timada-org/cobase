use actix_jwks::JwtPayload;
use actix_web::{get, post, web, HttpResponse, Scope};
use cobase::command::CommandInput;
use cobase::room::{CreateCommand, ListRoomsQuery};
use evento::{CommandError, CommandResponse};
use uuid::Uuid;

use crate::AppState;

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
        .send(ListRoomsQuery {
            user_id: Uuid::parse_str(&payload.subject)?,
        })
        .await??;

    Ok(HttpResponse::Ok().json(rooms))
}

#[utoipa::path(
    tag = "cobase",
    context_path = "/api/rooms",
    request_body=CreateCommand,
    responses(
        (status = 200, description = "Create room did not result error", body = CommandJsonResponse),
    )
)]
#[post("/create")]
async fn create_room(
    state: web::Data<AppState>,
    input: web::Json<CreateCommand>,
    payload: JwtPayload,
) -> HttpResponse {
    CommandResponse(
        state
            .cmd
            .send(CommandInput {
                user_id: payload.subject,
                input: input.0,
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
