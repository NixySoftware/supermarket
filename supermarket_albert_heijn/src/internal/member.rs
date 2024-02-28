use graphql_client::GraphQLQuery;

use crate::internal::scalar::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/get_member.graphql",
    response_derives = "Debug"
)]
pub struct GetMember;
