#![allow(clippy::match_single_binding)]

#[derive(Debug)]
pub enum Error {
    // Other
    Internal,
}

impl std::convert::From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        match err {
            // Other
            Error::Internal => crate::Error::Internal(String::new()),
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

impl std::convert::From<std::num::TryFromIntError> for Error {
    fn from(err: std::num::TryFromIntError) -> Self {
        match err {
            _ => Error::Internal,
        }
    }
}
