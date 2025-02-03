use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/get_search_suggestions.graphql",
    response_derives = "Debug"
)]
pub struct GetSearchSuggestions;
