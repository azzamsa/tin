use super::Service;
use crate::{db::Queryer, errors::core::Error};

impl Service {
    /// returns true if a username exists. false otherwise
    pub async fn check_username_exists<'c, C: Queryer<'c>>(
        &self,
        db: C,
        name: &str,
    ) -> Result<bool, crate::Error> {
        let find_existing_username_res = self.repo.find_user_by_name(db, name).await;
        match find_existing_username_res {
            Ok(_) => Ok(true),
            Err(Error::UserNotFound) => Ok(false),
            Err(err) => Err(err.into()),
        }
    }
}
