use std::fmt;

#[derive(Debug)]
pub enum RepositoryError {
    Connection(deadpool_diesel::PoolError),
    Query(diesel::result::Error),
    Interact(deadpool_diesel::InteractError),
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Connection(e) => write!(f, "Database connection error: {e}"),
            Self::Query(e) => write!(f, "Query error: {e}"),
            Self::Interact(e) => write!(f, "Interaction error: {e}"),
            Self::Other(e) => write!(f, "Error: {e}"),
        }
    }
}

impl std::error::Error for RepositoryError {}

impl From<deadpool_diesel::PoolError> for RepositoryError {
    fn from(e: deadpool_diesel::PoolError) -> Self {
        Self::Connection(e)
    }
}

impl From<diesel::result::Error> for RepositoryError {
    fn from(e: diesel::result::Error) -> Self {
        Self::Query(e)
    }
}

impl From<deadpool_diesel::InteractError> for RepositoryError {
    fn from(e: deadpool_diesel::InteractError) -> Self {
        Self::Interact(e)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for RepositoryError {
    fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::Other(e)
    }
}

impl RepositoryError {
    pub fn is_unique_violation(&self) -> bool {
        use diesel::result::{DatabaseErrorKind, Error};
        matches!(
            self,
            RepositoryError::Query(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _,))
        )
    }
}
