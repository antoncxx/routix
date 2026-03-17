use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::database::schema::proxy_host_upstreams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProxyHostUpstreamModel {
    pub proxy_host_id: i32,
    pub upstream_id: i32,
}
