use anyhow::{bail, Context};
use sqlx::PgPool;

use super::schema::{CreateUserInput, UpdateUserInput, User};

pub async fn find_all(pool: &PgPool) -> anyhow::Result<Vec<User>> {
    let users = sqlx::query_as!(User, r#"select * from user_"#)
        .fetch_all(pool)
        .await
        // for logging purpose
        .map_err(|err| {
            log::error!(
                "{}",
                format!("failed to perform a query to read user `{}`", err)
            );
            err
        })
        .context("failed to perform a query to read users")?;

    Ok(users)
}
pub async fn find(pool: &PgPool, id: i32) -> anyhow::Result<User> {
    let user = sqlx::query_as!(User, r#"select * from user_ where id = $1"#, id)
        .fetch_one(pool)
        .await;

    match user {
        Ok(user) => Ok(user),
        Err(error) => {
            log::error!(
                "{}",
                format!("failed to perform a query to read user `{}`", error)
            );
            match error {
                sqlx::Error::RowNotFound => {
                    bail!("user not found")
                }
                _ => bail!("unexpected error when performing a query to read user"),
            }
        }
    }
}
pub async fn find_by_name(pool: &PgPool, name: &str) -> anyhow::Result<User> {
    let user = sqlx::query_as!(User, r#"select * from user_ where name = $1"#, name)
        .fetch_one(pool)
        .await;

    match user {
        Ok(user) => Ok(user),
        Err(error) => {
            log::error!(
                "{}",
                format!("failed to perform a query to read user `{}`", error)
            );
            match error {
                sqlx::Error::RowNotFound => {
                    bail!("user not found")
                }
                _ => bail!("unexpected error when performing a query to read user"),
            }
        }
    }
}

pub async fn create(pool: &PgPool, user_input: CreateUserInput) -> anyhow::Result<User> {
    let existing_user = find_by_name(pool, &user_input.name).await;
    if existing_user.is_ok() {
        bail!("a user with same `name` already exists")
    }

    let full_name = if user_input.full_name.is_some() {
        user_input.full_name
    } else {
        None
    };

    let user = sqlx::query_as!(
        User,
        r#"insert into user_ (name, full_name) values ($1, $2) returning *"#,
        &user_input.name,
        full_name
    )
    .fetch_one(pool)
    .await
    // for logging purpose
    .map_err(|err| {
        log::error!(
            "{}",
            format!("failed to perform a query to insert user `{:?}`", err)
        );
        err
    })
    .context("failed to perform a query to insert user")?;

    Ok(user)
}

pub async fn update(pool: &PgPool, user_input: UpdateUserInput) -> anyhow::Result<User> {
    let existing_user = find_by_name(pool, &user_input.name).await;
    if existing_user.is_ok() {
        bail!("a user with same `name` already exists")
    }

    // `COALESCE` takes existing value if the input in None/Null.
    let user = sqlx::query_as!(
        User,
        r#"update user_ set
              id = $1,
              name = $2,
              full_name = COALESCE($3, full_name)
           where id = $1 returning *"#,
        user_input.id,
        user_input.name,
        user_input.full_name,
    )
    .fetch_one(pool)
    .await;

    match user {
        Ok(user) => Ok(user),
        Err(error) => {
            log::error!(
                "{}",
                format!("failed to perform a query to update user `{}`", error)
            );
            match error {
                sqlx::Error::RowNotFound => {
                    bail!("user not found")
                }
                _ => bail!("unexpected error when performing a query to update user"),
            }
        }
    }
}

pub async fn delete(pool: &PgPool, id: i32) -> anyhow::Result<User> {
    let error_message = "failed to perform a query to delete user";

    let is_user_exists = find(pool, id).await;
    if is_user_exists.is_err() {
        bail!("no user with the specified id")
    }

    let user = sqlx::query_as!(User, r#"delete from user_ where id = $1 returning *"#, id)
        .fetch_one(pool)
        .await;

    match user {
        Ok(user) => Ok(user),
        Err(error) => {
            log::error!("{}", format!("{} `{}`", error_message, error));
            match error {
                sqlx::Error::RowNotFound => {
                    bail!("user not found")
                }
                _ => bail!("unexpected error when performing a query to delete user"),
            }
        }
    }
}
