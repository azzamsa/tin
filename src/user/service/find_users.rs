use super::Service;
use crate::{
    errors::{
        core::Error::{
            MissingFirstAndLastPaginationArguments, PassedFirstAndLastPaginationArguments,
        },
        Error,
    },
    relay::Base64Cursor,
    user::model::{UserConnection, UserEdge},
};

impl Service {
    pub async fn find_users(
        &self,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<UserConnection, Error> {
        match (first, last) {
            (None, None) => return Err(MissingFirstAndLastPaginationArguments.into()),
            (Some(_), Some(_)) => return Err(PassedFirstAndLastPaginationArguments.into()),
            (Some(_first), None) => (Some(first), None),
            (None, Some(last)) => (None, Some(last)),
        };

        let (after_, before_) = match (after, before) {
            (None, None) => (None, None),
            (Some(after), Some(before)) => (
                Some(Base64Cursor::decode(&after)?.into()),
                Some(Base64Cursor::decode(&before)?.into()),
            ),
            (Some(after), None) => (Some(Base64Cursor::decode(&after)?.into()), None),
            (None, Some(before)) => (None, Some(Base64Cursor::decode(&before)?.into())),
        };

        let (users, page_info) = self
            .repo
            .find_all_users(&self.db, first, after_, last, before_)
            .await?;

        let user_edges: Vec<UserEdge> = users.into_iter().map(|user| user.into()).collect();
        let user_connection = UserConnection {
            edges: user_edges,
            page_info: page_info.into(),
        };

        Ok(user_connection)
    }
}
