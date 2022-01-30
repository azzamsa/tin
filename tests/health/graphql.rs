mod schema {
    cynic::use_schema!("schema.graphql");
}

#[cynic::schema_for_derives(file = "schema.graphql", module = "schema")]
pub mod queries {
    use serde::Deserialize;

    use super::schema;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct HealthQuery {
        pub health: Health,
    }

    // `serde::Deserialize` is needed only to Deserialize
    // a response from GraphQL. We use the struct here instead
    // of in `schema.rs` just to avoid duplication.
    // If `serde::Deserialize` changes cynic behavior, we will
    // move it to separate struct in `schema.rs`
    #[derive(cynic::QueryFragment, Debug, Deserialize)]
    pub struct Health {
        pub status: String,
    }
}
