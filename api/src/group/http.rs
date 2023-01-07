use std::ops::DerefMut;

use actix_jwks::JwtPayload;
use actix_web::{get, post, web, HttpResponse, Scope};
use uuid::Uuid;

use crate::command::CommandResponse;
use crate::error::Error;
use crate::AppState;

use super::aggregate::Group;
use super::command::CreateCommand;
use super::projection::Group as ReadGroup;

#[utoipa::path(
    context_path = "/api/groups",
    responses(
        (status = 200, description = "Get groups did not result error", body = [Group]),
    )
)]
#[get("")]
async fn find_all(state: web::Data<AppState>, payload: JwtPayload) -> Result<HttpResponse, Error> {
    let groups = sqlx::query_as!(
        ReadGroup,
        "SELECT * FROM groups WHERE user_id = $1",
        Uuid::parse_str(&payload.subject)?
    )
    .fetch_all(&state.db)
    .await?;

    Ok(HttpResponse::Ok().json(groups))
}

#[utoipa::path(
    context_path = "/api/groups",
    request_body=CreateCommand,
    responses(
        (status = 200, description = "Create group did not result error", body = JsonResponse),
    )
)]
#[post("/create")]
async fn create(
    state: web::Data<AppState>,
    input: web::Json<CreateCommand>,
    payload: JwtPayload,
) -> Result<HttpResponse, Error> {
    let mut producer = state.group_producer.lock().await;

    CommandResponse(state.cmd.send(input.0).await)
        .to_response::<Group>(&state.zone, &state.store, producer.deref_mut(), payload)
        .await
}

pub fn scope() -> Scope {
    web::scope("/groups").service(find_all).service(create)
}
