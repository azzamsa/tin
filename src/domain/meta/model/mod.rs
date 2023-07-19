use async_graphql::SimpleObject;
use frunk::LabelledGeneric;

#[derive(Debug, SimpleObject, LabelledGeneric)]
pub struct Meta {
    pub build: String,
    pub version: String,
}
