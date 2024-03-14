use graphql_client::GraphQLQuery;
use supermarket::Identifier;

use crate::internal::scalar::*;

use self::get_member::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/get_member.graphql",
    response_derives = "Debug"
)]
pub struct GetMember;

impl Identifier for GetMemberMember {
    fn identifier(&self) -> String {
        self.id.to_string()
    }
}
