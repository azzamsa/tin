use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};

use crate::{health::resolver::HealthQuery, meta::resolver::MetaQuery};

#[derive(MergedObject, Default)]
pub struct Query(MetaQuery, HealthQuery);

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;
