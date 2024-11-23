use chrono::Utc;
use graphql_client::GraphQLQuery;
use octocrab::{Octocrab, Result};
use serde_json::json;

#[allow(clippy::upper_case_acronyms)]
type URI = String;

#[allow(clippy::upper_case_acronyms)]
type DateTime = chrono::DateTime<Utc>;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "../../graphql/github_schema.graphql",
  query_path = "../../graphql/profile_query.graphql",
  response_derives = "Debug"
)]
pub struct ProfileQuery;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "../../graphql/github_schema.graphql",
  query_path = "../../graphql/repository_query.graphql",
  response_derives = "Debug"
)]
pub struct RepositoryQuery;

#[async_trait::async_trait]
pub trait GraphQLExt {
  async fn get_user_profile(&self) -> Result<profile_query::ProfileQueryViewer>;
  async fn get_graphql_repository(
    &self,
    username: &str,
    repository_name: &str,
  ) -> Result<repository_query::RepositoryQueryRepository>;
}

#[async_trait::async_trait]
impl GraphQLExt for Octocrab {
  async fn get_user_profile(&self) -> Result<profile_query::ProfileQueryViewer> {
    let payload = &ProfileQuery::build_query(profile_query::Variables {});
    let graphql_response = self
      .graphql::<graphql_client::Response<profile_query::ResponseData>>(&json!(payload))
      .await?;

    if let Some(errors) = graphql_response.errors {
      panic!("GraphQL errors occured: {:?}", errors);
    }

    if let Some(profile) = graphql_response.data {
      Ok(profile.viewer)
    } else {
      panic!("GraphQL errors occured")
    }
  }

  async fn get_graphql_repository(
    &self,
    username: &str,
    repository_name: &str,
  ) -> Result<repository_query::RepositoryQueryRepository> {
    let payload = &RepositoryQuery::build_query(repository_query::Variables {
      name: repository_name.to_string(),
      owner: username.to_string(),
    });
    let graphql_response = self
      .graphql::<graphql_client::Response<repository_query::ResponseData>>(&json!(payload))
      .await?;

    if let Some(errors) = graphql_response.errors {
      panic!("GraphQL errors occured: {:?}", errors);
    }

    if let Some(data) = graphql_response.data {
      if let Some(repository) = data.repository {
        Ok(repository)
      } else {
        panic!("GraphQL errors occured")
      }
    } else {
      panic!("GraphQL errors occured")
    }
  }
}
