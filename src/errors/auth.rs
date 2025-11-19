use std::fmt::{self};

#[derive(Debug)]
pub enum Error {
    InvalidPassword,
    InvalidToken,
    ExpiredToken,
    MissingCredentials,
    Unspecified,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPassword => write!(f, "invalid password"),
            Self::InvalidToken => write!(f, "invalid token"),
            Self::ExpiredToken => write!(f, "token has expired"),
            Self::MissingCredentials => write!(f, "credentials are missing"),
            Self::Unspecified => write!(f, "unauthorized"),
        }
    }
}
