use async_graphql::SimpleObject;

#[derive(Debug, SimpleObject)]
pub struct Health {
    pub status: String,
}
