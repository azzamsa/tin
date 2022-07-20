use super::Service;
use crate::{
    errors::Error,
    user::{entities::User, service::PageInfo},
};

impl Service {
    pub async fn find_users(
        &self,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<(Vec<User>, PageInfo), Error> {
        let (users, page_info) = self
            .repo
            .find_all_users(&self.db, after, before, first, last)
            .await?;

        Ok((users, page_info))
    }
}
