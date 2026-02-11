mod schema {
    cynic::use_schema!("schema.graphql");
}

#[cynic::schema_for_derives(file = "schema.graphql", module = "schema")]
pub mod queries {
    use super::schema;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct HealthQuery {
        pub health: Health,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct Health {
        pub status: String,
    }
}
