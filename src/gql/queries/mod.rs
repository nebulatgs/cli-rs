use graphql_client::GraphQLQuery;
type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/gql/schema.graphql",
	query_path = "src/gql/queries/strings/GetUser.graphql",
	response_derives = "Debug"
)]
pub struct GetUser;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/gql/schema.graphql",
	query_path = "src/gql/queries/strings/GetProjects.graphql",
	response_derives = "Debug"
)]
pub struct GetProjects;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/gql/schema.graphql",
	query_path = "src/gql/queries/strings/GetProject.graphql",
	response_derives = "Debug"
)]
pub struct GetProject;
