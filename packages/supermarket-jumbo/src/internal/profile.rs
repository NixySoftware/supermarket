use graphql_client::GraphQLQuery;
use supermarket::Identifier;

use self::get_profile::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/loyalty_schema.graphql",
    query_path = "src/graphql/get_profile.graphql",
    response_derives = "Debug"
)]
pub struct GetProfile;

impl Identifier for GetProfileProfile {
    fn identifier(&self) -> String {
        self.customer_id.to_string()
    }
}
