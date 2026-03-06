use crate::{context::Context, repos::UsersRepository, roles::UserRole, scopes::UserScope};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct LoginRequestbody {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponseBody {
    pub jwt: String,
}

pub async fn login(
    State(ctx): State<Context>,
    Json(body): Json<LoginRequestbody>,
) -> impl IntoResponse {
    let user = match UsersRepository::find_by_username(&body.username, &ctx.database).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let Ok(valid) = bcrypt::verify(&body.password, &user.password) else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    if !valid {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let Ok(role) = UserRole::from_str(&user.role) else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let scopes = user
        .scopes
        .into_iter()
        .flatten()
        .filter_map(|s| UserScope::from_str(&s).ok())
        .collect::<Vec<_>>();

    match ctx.jwt.issue(&body.username, role, scopes) {
        Ok(jwt) => (StatusCode::OK, Json(LoginResponseBody { jwt })).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
