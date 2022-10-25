#![allow(clippy::match_single_binding)]

use sqlx;

#[derive(Debug)]
pub enum Error {
    // Other
    Internal,
    MissingFirstAndLastPaginationArguments,
    PassedFirstAndLastPaginationArguments,

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
            Error::MissingFirstAndLastPaginationArguments => crate::Error::InvalidArgument(
                "You must provide a `first` or `last` value to properly paginate the entity."
                    .to_string(),
            ),
            Error::PassedFirstAndLastPaginationArguments => crate::Error::InvalidArgument(
                "Passing both `first` and `last` for pagination is not supported.".to_string(),
            ),
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
