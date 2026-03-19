use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::database::schema::access_lists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccessListModel {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Serialize, Deserialize, Default)]
#[diesel(table_name = crate::database::schema::access_lists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAccessListModel {
    pub name: String,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize, Default)]
#[diesel(table_name = crate::database::schema::access_lists)]
pub struct UpdateAccessListModel {
    pub name: Option<String>,
}
