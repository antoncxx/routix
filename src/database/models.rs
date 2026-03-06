use chrono::{DateTime, Utc};
use diesel::prelude::*;


#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: String,
    pub scopes: Vec<Option<String>>,
    pub created_at: DateTime<Utc>,
}
