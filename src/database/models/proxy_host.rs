use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::database::schema::proxy_hosts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProxyHostModel {
    pub id: i32,
    pub domain: String,
    pub certificate_name: Option<String>,
    pub access_list_id: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::database::schema::proxy_hosts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProxyHostModel {
    pub domain: String,
    pub certificate_name: Option<String>,
    pub access_list_id: Option<i32>,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize, Default)]
#[diesel(table_name = crate::database::schema::proxy_hosts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateProxyHostModel {
    pub domain: Option<String>,
}
