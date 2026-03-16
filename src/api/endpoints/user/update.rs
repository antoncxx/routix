use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use validator::Validate;

use crate::context::Context;
use crate::database::models::UpdateUserModel;
use crate::database::repos::UsersRepository;
use crate::roles::UserRole;
use crate::scopes::UserScope;

#[derive(Deserialize, Validate)]
pub struct UpdateUserRequestBody {
    #[validate(length(min = 8, max = 255))]
    password: Option<String>,
    role: Option<UserRole>,
    scopes: Option<Vec<UserScope>>,
}

pub async fn update(
    State(ctx): State<Context>,
    Path(username): Path<String>,
    Json(body): Json<UpdateUserRequestBody>,
) -> impl IntoResponse {
    if body.validate().is_err() {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    let password = match body.password {
        Some(ref p) => match bcrypt::hash(p, bcrypt::DEFAULT_COST) {
            Ok(hashed) => Some(hashed),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
        None => None,
    };

    let model = UpdateUserModel {
        password,
        role: body.role.map(|r| r.to_string()),
        scopes: body
            .scopes
            .map(|s| s.iter().map(|s| Some(s.to_string())).collect()),
    };

    match UsersRepository::update(&username, model, &ctx.database).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) if e.is_not_found() => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
