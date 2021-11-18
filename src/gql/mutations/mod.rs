use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/gql/schema.graphql",
	query_path = "src/gql/mutations/strings/ConsumeLoginSession.graphql",
	response_derives = "Debug"
)]
pub struct ConsumeLoginSession;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/gql/schema.graphql",
	query_path = "src/gql/mutations/strings/CreateLoginSession.graphql",
	response_derives = "Debug"
)]
pub struct CreateLoginSession;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/gql/schema.graphql",
	query_path = "src/gql/mutations/strings/Logout.graphql",
	response_derives = "Debug"
)]
pub struct Logout;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/gql/schema.graphql",
	query_path = "src/gql/mutations/strings/DeleteProject.graphql",
	response_derives = "Debug"
)]
pub struct DeleteProject;
