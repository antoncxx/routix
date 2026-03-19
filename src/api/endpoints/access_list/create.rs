use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api::endpoints::utils::{validate_access_list_action, validate_address};
use crate::database::models::{NewAccessListModel, NewAccessListRuleModel};
use crate::{context::Context, database::repos::AccessListsRepository};

#[derive(Deserialize, Validate, Serialize)]
pub struct CreateAccessListRuleRequest {
    #[validate(custom(function = "validate_access_list_action"))]
    pub action: String,
    #[validate(custom(function = "validate_address"))]
    pub address: String,
}

#[derive(Deserialize, Validate)]
pub struct CreateAccessListRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1), nested)]
    pub rules: Vec<CreateAccessListRuleRequest>,
}

pub async fn create(
    State(ctx): State<Context>,
    Json(body): Json<CreateAccessListRequest>,
) -> impl IntoResponse {
    if body.validate().is_err() {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    for rule in &body.rules {
        if !matches!(rule.action.as_str(), "allow" | "deny") {
            return StatusCode::UNPROCESSABLE_ENTITY.into_response();
        }
    }

    let model = NewAccessListModel { name: body.name };

    let rules: Vec<NewAccessListRuleModel> = body
        .rules
        .into_iter()
        .enumerate()
        .map(|(i, r)| NewAccessListRuleModel {
            access_list_id: 0, // stamped in the repository
            action: r.action,
            address: r.address,
            sort_order: i as i32,
        })
        .collect();

    match AccessListsRepository::create(model, rules, &ctx.database).await {
        Ok(result) => Json(result).into_response(),
        Err(e) if e.is_unique_violation() => StatusCode::CONFLICT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
