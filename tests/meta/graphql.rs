mod schema {
    cynic::use_schema!("schema.graphql");
}

#[cynic::schema_for_derives(file = "schema.graphql", module = "schema")]
pub mod queries {
    use serde::Deserialize;

    use super::schema;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct MetaQuery {
        pub meta: Meta,
    }

    #[derive(cynic::QueryFragment, Debug, Deserialize)]
    pub struct Meta {
        pub build: String,
        pub version: String,
    }
}
