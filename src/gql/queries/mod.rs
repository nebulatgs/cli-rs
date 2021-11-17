use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/gql/schema.graphql",
	query_path = "src/gql/queries/strings/GetUser.graphql"
)]
pub struct GetUser;
