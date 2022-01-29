// db module is public to allow reusability in integration test
pub mod db;
mod health;
pub mod logger;
mod meta;
pub mod routes;
mod user;

use std::env;

#[macro_use]
extern crate diesel;

use anyhow::Context;
use async_graphql::{EmptySubscription, MergedObject, Schema};

use crate::health::resolver::HealthQuery;
use crate::meta::resolver::MetaQuery;
use crate::user::resolver::{UserMutation, UserQuery};

#[derive(MergedObject, Default)]
pub struct Query(HealthQuery, MetaQuery, UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn get_env(name: &str) -> anyhow::Result<String> {
    env::var(&name).context(format!("`{}` is not set", &name))
}
