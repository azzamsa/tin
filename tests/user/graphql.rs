mod schema {
    cynic::use_schema!("tests/schema.graphql");
}

#[cynic::schema_for_derives(file = "tests/schema.graphql", module = "schema")]
pub mod queries {
    use super::schema;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct UsersQuery {
        pub users: Vec<User>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct User {
        pub id: Option<Uuid>,
        pub name: String,
        pub full_name: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", argument_struct = "ReadUserArguments")]
    pub struct UserQuery {
        #[arguments(id = &args.id)]
        pub user: User,
    }

    // All sturct must be inline
    #[derive(cynic::FragmentArguments, Debug)]
    pub struct ReadUserArguments {
        pub id: Uuid,
    }

    // All sturct must be inline
    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct Uuid(pub String);
}

#[cynic::schema_for_derives(file = "tests/schema.graphql", module = "schema")]
pub mod add {
    use super::schema;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", argument_struct = "CreateUserInput")]
    pub struct UserMutation {
        #[arguments(input =
              CreateUserInput {
                 name: args.name.clone(),
                 full_name: args.full_name.clone(),
            }
        )]
        pub create_user: User,
    }

    #[derive(cynic::InputObject, cynic::FragmentArguments, Debug)]
    pub struct CreateUserInput {
        pub name: String,
        pub full_name: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct User {
        pub id: Option<Uuid>,
        pub name: String,
        pub full_name: Option<String>,
    }

    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct Uuid(pub String);
}

#[cynic::schema_for_derives(file = "tests/schema.graphql", module = "schema")]
pub mod update {
    use super::schema;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", argument_struct = "UpdateUserInput")]
    pub struct UserMutation {
        #[arguments(input =
                    UpdateUserInput {
                        id: args.id.clone(),
                        name: args.name.clone(),
                        full_name: args.full_name.clone(),
            }
        )]
        pub update_user: User,
    }

    #[derive(cynic::InputObject, cynic::FragmentArguments, Debug)]
    pub struct UpdateUserInput {
        pub id: Uuid,
        pub name: String,
        pub full_name: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct User {
        pub id: Option<Uuid>,
        pub name: String,
        pub full_name: Option<String>,
    }
    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct Uuid(pub String);
}

#[cynic::schema_for_derives(file = "tests/schema.graphql", module = "schema")]
pub mod delete {
    use super::schema;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", argument_struct = "DeleteUserArguments")]
    pub struct UserMutation {
        #[arguments(id = &args.id)]
        pub delete_user: User,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct DeleteUserArguments {
        pub id: Uuid,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct User {
        pub id: Option<Uuid>,
        pub name: String,
        pub full_name: Option<String>,
    }
    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct Uuid(pub String);
}
