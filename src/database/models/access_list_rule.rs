use super::AccessListModel;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::database::schema::access_list_rules)]
#[diesel(belongs_to(AccessListModel, foreign_key = access_list_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccessListRuleModel {
    pub id: i32,
    pub access_list_id: i32,
    pub action: String,
    pub address: String,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Serialize, Deserialize, Default, Clone)]
#[diesel(table_name = crate::database::schema::access_list_rules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAccessListRuleModel {
    pub access_list_id: i32,
    pub action: String,
    pub address: String,
    pub sort_order: i32,
}
