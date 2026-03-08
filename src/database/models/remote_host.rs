use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::database::schema::proxy_hosts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProxyHostModel {
    pub id: i32,
    pub domain: String,
    pub forward_host: String,
    pub forward_port: i32,
    pub certificate_name: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Serialize, Deserialize, Default)]
#[diesel(table_name = crate::database::schema::proxy_hosts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProxyHostModel {
    pub domain: String,
    pub forward_host: String,
    pub forward_port: i32,
    pub certificate_name: Option<String>,
}
