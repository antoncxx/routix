use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use validator::Validate;

use crate::context::Context;
use crate::database::models::NewUserModel;
use crate::database::repos::UsersRepository;
use crate::roles::UserRole;
use crate::scopes::UserScope;

static USERNAME_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap());

#[derive(Deserialize, Validate)]
pub struct CreateUserRequestBody {
    #[validate(length(min = 3, max = 100), regex(path = *USERNAME_REGEX))]
    username: String,
    #[validate(length(min = 8, max = 255))]
    password: String,
    role: UserRole,
    scopes: Vec<UserScope>,
}

pub async fn create(
    State(ctx): State<Context>,
    Json(body): Json<CreateUserRequestBody>,
) -> impl IntoResponse {
    if body.validate().is_err() {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    let Ok(hashed) = bcrypt::hash(&body.password, bcrypt::DEFAULT_COST) else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let model = NewUserModel {
        username: body.username,
        password: hashed,
        role: body.role.to_string(),
        scopes: body.scopes.iter().map(|s| Some(s.to_string())).collect(),
    };

    match UsersRepository::create(model, &ctx.database).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) if e.is_unique_violation() => StatusCode::CONFLICT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
