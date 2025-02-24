mod schema {
    cynic::use_schema!("tests/schema.graphql");
}

#[cynic::schema_for_derives(file = "tests/schema.graphql", module = "schema")]
pub mod queries {
    use super::schema;

    #[derive(cynic::Scalar, Debug, Clone)]
    #[cynic(graphql_type = "UUID")]
    pub struct Uuid(pub String);

    #[derive(cynic::QueryFragment, Debug)]
    pub struct User {
        pub id: Uuid,
        pub name: String,
        pub full_name: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "ReadUsersArguments")]
    pub struct UsersQuery {
        #[arguments(first: $first, after : $after, last : $last, before : $before)]
        pub users: UserConnection,
    }

    // All struct must be inline
    #[derive(cynic::QueryVariables, Debug)]
    pub struct ReadUsersArguments {
        pub first: Option<i32>,
        pub after: Option<String>,
        pub last: Option<i32>,
        pub before: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct UserConnection {
        pub total_count: i32,
        pub edges: Vec<UserEdge>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct UserEdge {
        pub node: User,
        pub cursor: String,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "ReadUserArguments")]
    #[allow(dead_code)]
    pub struct UserQuery {
        #[arguments(id : $id)]
        pub user: User,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct ReadUserArguments {
        pub id: Uuid,
    }
}

#[cynic::schema_for_derives(file = "tests/schema.graphql", module = "schema")]
pub mod mutations {
    use super::{queries, schema};

    #[derive(cynic::QueryVariables, Debug)]
    pub struct CreateUserInput {
        pub name: String,
        pub email: String,
        pub full_name: Option<String>,
    }

    // This must exist alongside `CreateUserInput`
    // since the latter doesn't have an `id` field.
    #[derive(cynic::QueryVariables, Debug)]
    pub struct UpdateUserInput {
        pub id: queries::Uuid,
        pub name: String,
        pub email: String,
        pub full_name: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "CreateUserInput")]
    pub struct CreateUser {
        #[arguments(input : {
            name: $name,
            email: $email,
            fullName: $full_name,
        })]
        #[allow(dead_code)]
        pub create_user: queries::User,
    }

    // This must exist alongside `CreateUser`
    // since the latter doesn't have an `id` field.
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "UpdateUserInput")]
    pub struct UpdateUser {
        #[arguments(input : {
            id: $id,
            name: $name,
            email: $email,
            fullName: $full_name,
        })]
        #[allow(dead_code)]
        pub update_user: queries::User,
    }

    #[derive(cynic::QueryVariables, Debug)]
    pub struct DeleteUserArguments {
        pub id: queries::Uuid,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "DeleteUserArguments")]
    pub struct DeleteUser {
        #[arguments(id: $id)]
        #[allow(dead_code)]
        pub delete_user: queries::User,
    }
}

// #[cynic::schema_for_derives(file = "tests/schema.graphql", module = "schema")]
// pub mod update {
//     use super::{core, schema};
//
//     #[derive(cynic::QueryVariables, Debug)]
//     pub struct UpdateUserInput {
//         pub id: core::Uuid,
//         pub name: String,
//         pub email: String,
//         pub full_name: Option<String>,
//     }
//

// }
//
// #[cynic::schema_for_derives(file = "tests/schema.graphql", module = "schema")]
// pub mod delete {
//     use super::{core, schema};
//
//     #[derive(cynic::QueryVariables, Debug)]
//     pub struct DeleteUserArguments {
//         pub id: core::Uuid,
//     }
//

// }
