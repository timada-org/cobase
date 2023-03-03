use actix_jwks::JwtPayload;
use actix_web::{get, post, web, HttpResponse, Scope};
use cobase::command::CommandInput;
use cobase::group::{CreateCommand, Group, ListGroupsQuery};
use evento::{CommandError, CommandResponse};
use uuid::Uuid;

use crate::AppState;

#[utoipa::path(
    tag = "cobase",
    context_path = "/api/groups",
    responses(
        (status = 200, description = "Get groups did not result error", body = [Group]),
    )
)]
#[get("")]
async fn list_groups(
    state: web::Data<AppState>,
    payload: JwtPayload,
) -> Result<HttpResponse, CommandError> {
    let groups = state
        .query
        .send(ListGroupsQuery {
            user_id: Uuid::parse_str(&payload.subject)?,
        })
        .await??;

    Ok(HttpResponse::Ok().json(groups))
}

#[utoipa::path(
    tag = "cobase",
    context_path = "/api/groups",
    request_body=CreateCommand,
    responses(
        (status = 200, description = "Create group did not result error", body = CommandJsonResponse),
    )
)]
#[post("/create")]
async fn create_group(
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
    .to_response::<Group, _>(&state.publisher)
    .await
}

pub fn scope() -> Scope {
    web::scope("/groups")
        .service(list_groups)
        .service(create_group)
}
