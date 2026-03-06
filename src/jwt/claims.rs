use serde::{Deserialize, Serialize};

use crate::{roles::UserRole, scopes::UserScope};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
    pub iat: u64,
    pub scopes: Vec<UserScope>,
    pub role: UserRole,
}

impl Claims {
    // pub fn has_scope(&self, scope: UserScope) -> bool {
    //     self.scopes.contains(&scope)
    // }

    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }
}
