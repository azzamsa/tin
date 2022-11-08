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
