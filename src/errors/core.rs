#![allow(clippy::match_single_binding)]

use sqlx;

#[derive(Debug)]
pub enum Error {
    // Other
    Internal,

    // User
    UserNotFound,
    UsernameAlreadyExists,
}

impl std::convert::From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        match err {
            // Users
            Error::UserNotFound => crate::Error::NotFound(String::from("user not found")),
            Error::UsernameAlreadyExists => {
                crate::Error::AlreadyExists(String::from("username is already in use"))
            }

            // Other
            Error::Internal => crate::Error::Internal(String::new()),
        }
    }
}

impl std::convert::From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            // Not found error should be catched manually
            _ => Error::Internal,
        }
    }
}

impl std::convert::From<async_graphql::Error> for Error {
    fn from(err: async_graphql::Error) -> Self {
        match err {
            _ => Error::Internal,
        }
    }
}
