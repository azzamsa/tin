use anyhow::{bail, Context};
use diesel::prelude::*;
use diesel::result::Error::NotFound;

use crate::diesel::RunQueryDsl;

use crate::db::DbPool;

use super::schema::{CreateUserInput, UpdateUserInput, User};
use crate::db::schema::user_ as user;

pub fn find_all(pool: &DbPool) -> anyhow::Result<Vec<User>> {
    let users = user::table
        .load::<User>(&pool.get()?)
        .context("failed to perform a query to read users")?;

    Ok(users)
}
pub fn find(pool: &DbPool, id: i32) -> anyhow::Result<User> {
    let row = user::table
        .filter(user::id.eq(id))
        .first::<User>(&pool.get()?);

    match row {
        Ok(user) => Ok(user),
        Err(error) => {
            log::error!(
                "{}",
                format!("failed to perform a query to find user `{}`", error)
            );
            match error {
                NotFound => {
                    bail!("user not found")
                }
                _ => bail!("unexpected error when performing a query to find user"),
            }
        }
    }
}
pub fn find_by_name(pool: &DbPool, name: &str) -> anyhow::Result<User> {
    let user = user::table
        .filter(user::name.eq(name))
        .first::<User>(&pool.get()?)
        .context("failed to perform a query to read users")?;

    Ok(user)
}

pub fn create(pool: &DbPool, user_input: CreateUserInput) -> anyhow::Result<User> {
    let error_message = "failed to perform a query to insert user";

    let is_user_exists = find_by_name(pool, &user_input.name);
    if is_user_exists.is_ok() {
        bail!("a user with same `name` already exists")
    }

    let user = diesel::insert_into(user::table)
        .values(user_input)
        .get_result::<User>(&pool.get()?)
        // for logging purpose
        .map_err(|err| {
            log::error!("{}", format!("{} `{:?}`", error_message, err));
            err
        })
        .context("{error_message}")?;

    Ok(user)
}

pub fn update(pool: &DbPool, user_input: UpdateUserInput) -> anyhow::Result<User> {
    let error_message = "failed to perform a query to update user";

    let is_user_exists = find(pool, user_input.id);
    if is_user_exists.is_err() {
        bail!("no user with the specified id")
    }

    let user = diesel::update(user::table)
        .filter(user::id.eq(user_input.id))
        .set(user_input)
        .get_result::<User>(&pool.get()?)
        .map_err(|err| {
            log::error!("{}", format!("{} `{:?}`", error_message, err));
            err
        })
        .context("{error_message}")?;

    Ok(user)
}

pub fn delete(pool: &DbPool, id: i32) -> anyhow::Result<User> {
    let error_message = "failed to perform a query to delete user";

    let is_user_exists = find(pool, id);
    if is_user_exists.is_err() {
        bail!("no user with the specified id")
    }

    let user = diesel::delete(user::table)
        .filter(user::id.eq(id))
        .get_result::<User>(&pool.get()?)
        .map_err(|err| {
            log::error!("{}", format!("{} `{:?}`", error_message, err));
            err
        })
        .context("{error_message}")?;

    Ok(user)
}
