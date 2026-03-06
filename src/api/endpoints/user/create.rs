use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;

use crate::context::Context;
use crate::database::models::NewUserModel;
use crate::repos::UsersRepository;
use crate::roles::UserRole;
use crate::scopes::UserScope;

#[derive(Deserialize)]
pub struct CreateUserRequestBody {
    username: String,
    password: String,
    role: UserRole,
    scopes: Vec<UserScope>,
}

pub async fn create(
    State(ctx): State<Context>,
    Json(body): Json<CreateUserRequestBody>,
) -> impl IntoResponse {
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
