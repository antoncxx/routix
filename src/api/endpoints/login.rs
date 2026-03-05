use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::context::Context;

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
    // TODO: verify credentials against DB
    // TODO: look up user roles

    let _ = body.password;

    match ctx.jwt.issue(&body.username, vec![]) {
        Ok(jwt) => (StatusCode::OK, Json(LoginResponseBody { jwt })).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
