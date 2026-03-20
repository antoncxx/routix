use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api::endpoints::utils::{validate_access_list_action, validate_address};
use crate::context::Context;
use crate::database::models::{NewAccessListRuleModel, UpdateAccessListModel};
use crate::database::repos::AccessListsRepository;

#[derive(Deserialize, Validate, Serialize)]
pub struct UpdateAccessListRuleRequest {
    #[validate(custom(function = "validate_access_list_action"))]
    pub action: String,
    #[validate(custom(function = "validate_address"))]
    pub address: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateAccessListRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    #[validate(length(min = 1), nested)]
    pub rules: Option<Vec<UpdateAccessListRuleRequest>>,
}

pub async fn update(
    State(ctx): State<Context>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateAccessListRequest>,
) -> impl IntoResponse {
    if body.validate().is_err() {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    if let Some(ref rules) = body.rules
        && rules.is_empty()
    {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    }

    let model = body
        .name
        .map(|name| UpdateAccessListModel { name: Some(name) });

    let rules = body.rules.map(|rules| {
        rules
            .into_iter()
            .enumerate()
            .map(|(i, r)| {
                Ok(NewAccessListRuleModel {
                    access_list_id: id,
                    action: r.action,
                    address: r.address,
                    sort_order: i32::try_from(i).map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?,
                })
            })
            .collect::<Result<Vec<_>, StatusCode>>()
    });

    let rules = match rules.transpose() {
        Ok(rules) => rules,
        Err(status) => return status.into_response(),
    };

    let data = match AccessListsRepository::update_full(id, model, rules, &ctx.database).await {
        Ok(data) => data,
        Err(e) if e.is_not_found() => return StatusCode::NOT_FOUND.into_response(),
        Err(e) if e.is_unique_violation() => return StatusCode::CONFLICT.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let () = ctx.hosts_manager.update_access_list(&data.0, &data.1).await;

    StatusCode::OK.into_response()
}
