use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::database::schema::certificates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CertificateModel {
    pub id: i32,
    pub name: String,
    pub type_: String,
    pub certificate: String,
    pub private_key: String,
    pub dns_provider: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Serialize, Deserialize, Default)]
#[diesel(table_name = crate::database::schema::certificates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCertificateModel {
    pub type_: String,
    pub name: String,
    pub certificate: String,
    pub private_key: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub dns_provider: Option<String>,
}
