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

impl UpdateAccessListRequest {
    pub fn has_changes(&self) -> bool {
        self.name.is_some()
    }
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

    let _ = if body.has_changes() {
        let model = UpdateAccessListModel { name: body.name };

        match AccessListsRepository::update(id, model, &ctx.database).await {
            Ok(model) => model,
            Err(e) if e.is_unique_violation() => return StatusCode::CONFLICT.into_response(),
            Err(e) if e.is_not_found() => return StatusCode::NOT_FOUND.into_response(),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    } else {
        match AccessListsRepository::fetch(id, &ctx.database).await {
            Ok(model) => model,
            Err(e) if e.is_not_found() => return StatusCode::NOT_FOUND.into_response(),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    };

    let (_1, _2) = if let Some(rules) = body.rules {
        let rules = rules
            .into_iter()
            .enumerate()
            .map(|(i, r)| NewAccessListRuleModel {
                access_list_id: id,
                action: r.action,
                address: r.address,
                sort_order: i as i32,
            })
            .collect();

        match AccessListsRepository::set_rules(id, rules, &ctx.database).await {
            Ok((access_list, rules)) => (access_list, rules),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    } else {
        match AccessListsRepository::fetch(id, &ctx.database).await {
            Ok((access_list, rules)) => (access_list, rules),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    };

    StatusCode::OK.into_response()
}

// @TODO: refactor
// idiocracy in from of my eyes
