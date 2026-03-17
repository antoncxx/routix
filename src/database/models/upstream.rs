use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::database::schema::upstreams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpstreamModel {
    pub id: i32,
    pub name: String,
    pub schema: String,
    pub host: String,
    pub port: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::database::schema::upstreams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUpstreamModel {
    pub name: String,
    pub schema: String,
    pub host: String,
    pub port: i32,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize, Default)]
#[diesel(table_name = crate::database::schema::upstreams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateUpstreamModel {
    pub name: Option<String>,
    pub schema: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
}
