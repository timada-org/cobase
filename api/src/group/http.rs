use std::ops::DerefMut;

use actix_jwks::JwtPayload;
use actix_web::{get, post, web, HttpResponse, Scope};
use futures::TryStreamExt;
use mongodb::bson::doc;

use crate::command::CommandResponse;
use crate::error::Error;
use crate::AppState;

use super::aggregate::Group;
use super::command::CreateCommand;

#[utoipa::path(
    context_path = "/groups",
    responses(
        (status = 200, description = "Get groups did not result error", body = [Vec<Group>]),
    )
)]
#[get("")]
async fn find_all(state: web::Data<AppState>, payload: JwtPayload) -> Result<HttpResponse, Error> {
    let collection = state.read_db.collection::<Group>("groups");
    let mut cursor = collection
        .find(doc! {"user_id": payload.subject}, None)
        .await?;
    let mut groups = Vec::new();
    while let Some(group) = cursor.try_next().await? {
        groups.push(group);
    }

    Ok(HttpResponse::Ok().json(groups))
}

#[utoipa::path(
    context_path = "/groups",
    request_body=CreateCommand,
    responses(
        (status = 200, description = "Create group did not result error", body = [Group]),
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
