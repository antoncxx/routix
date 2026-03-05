use std::fmt;

#[derive(Debug)]
pub enum JwtError {
    MissingSecret,
    IssueError(jsonwebtoken::errors::Error),
    VerifyError(jsonwebtoken::errors::Error),
    Expired,
}

impl fmt::Display for JwtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSecret => write!(f, "Missing JWT_SECRET environment variable"),
            Self::IssueError(e) => write!(f, "Failed to issue token: {e}"),
            Self::VerifyError(e) => write!(f, "Failed to verify token: {e}"),
            Self::Expired => write!(f, "Token has expired"),
        }
    }
}

impl std::error::Error for JwtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IssueError(e) | Self::VerifyError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for JwtError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        Self::IssueError(e)
    }
}
