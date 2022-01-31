use async_graphql::SimpleObject;

#[derive(Debug, SimpleObject)]
pub struct Meta {
    pub build: String,
    pub version: String,
}
