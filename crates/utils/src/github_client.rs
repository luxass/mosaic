use std::collections::HashMap;
use async_trait::async_trait;
use chrono::Utc;
use graphql_client::GraphQLQuery;
use mockall::mock;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use reqwest::Method;
use crate::{models, AppError};
use crate::error::GitHubErrorBody;
use crate::models::events::Event;
use crate::models::GitHubContentObject;

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

#[derive(Clone, Debug)]
pub struct GitHubClient {
    client: reqwest::Client,
}

impl GitHubClient {
    pub fn new(token: &str) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("token {}", token)).unwrap(),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static("mosaic"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client })
    }
}


#[async_trait]
pub trait GitHubClientTrait {

    async fn get_user_events(&self, username: &str) -> Result<Vec<models::events::Event>, AppError>;

    async fn get_user_profile(&self) -> Result<profile_query::ProfileQueryViewer, AppError>;

    async fn get_repository(
        &self,
        username: &str,
        repository_name: &str,
    ) -> Result<repository_query::RepositoryQueryRepository, AppError>;

    async fn get_languages(
        &self,
        username: &str,
        repository_name: &str,
    ) -> Result<HashMap<String, i32>, AppError>;

    async fn get_content_by_path(
        &self,
        username: &str,
        repository_name: &str,
        path: &str,
    ) -> Result<GitHubContentObject, AppError>;
}

#[async_trait]
impl GitHubClientTrait for GitHubClient {
    async fn get_user_events(&self, username: &str) -> Result<Vec<Event>, AppError> {
        let url = format!(
            "https://api.github.com/users/{}/events?per_page=100&page=1",
            username
        );
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let events = response.json::<Vec<models::events::Event>>().await?;
            Ok(events)
        } else {
            // TODO: fix this
            let _status = response.status();
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            let github_error = match serde_json::from_str::<GitHubErrorBody>(&error_body) {
                Ok(parsed_error) => parsed_error,
                Err(_) => GitHubErrorBody {
                    documentation_url: None,
                    errors: None,
                    message: error_body.clone(),
                },
            };

            Err(AppError::GitHubError(GitHubErrorBody {
                documentation_url: github_error.documentation_url,
                errors: github_error.errors,
                message: github_error.message,
            }))
        }
    }

    async fn get_user_profile(&self) -> Result<profile_query::ProfileQueryViewer, AppError> {
        let response = self
            .client
            .post("https://api.github.com/graphql")
            .json(&ProfileQuery::build_query(profile_query::Variables {}))
            .send()
            .await?;

        if !response.status().is_success() {
            tracing::error!("Failed to fetch user profile: {:?}", response);
            return Err(AppError::GitHubError(GitHubErrorBody {
                documentation_url: None,
                errors: None,
                message: "Failed to fetch user profile".to_string(),
            }));
        }

        let query_response = response
            .json::<graphql_client::Response<profile_query::ResponseData>>()
            .await?;
        if let Some(errors) = query_response.errors {
            return Err(AppError::GitHubError(GitHubErrorBody {
                documentation_url: None,
                errors: Some(errors.into_iter().map(|e| serde_json::json!(e)).collect()),
                message: "Failed to fetch user profile".to_string(),
            }));
        }

        if let Some(profile) = query_response.data {
            return Ok(profile.viewer);
        }

        Err(AppError::GitHubError(GitHubErrorBody {
            documentation_url: None,
            errors: None,
            message: "Failed to fetch user profile".to_string(),
        }))
    }

    async fn get_repository(&self, username: &str, repository_name: &str) -> Result<repository_query::RepositoryQueryRepository, AppError> {
        let response = self
            .client
            .post("https://api.github.com/graphql")
            .json(&RepositoryQuery::build_query(repository_query::Variables {
                name: repository_name.to_string(),
                owner: username.to_string(),
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            tracing::error!("Failed to fetch repository: {:?}", response);
            return Err(AppError::GitHubError(GitHubErrorBody {
                documentation_url: None,
                errors: None,
                message: "Failed to fetch repository".to_string(),
            }));
        }

        let query_response = response
            .json::<graphql_client::Response<repository_query::ResponseData>>()
            .await?;
        if let Some(errors) = query_response.errors {
            return Err(AppError::GitHubError(GitHubErrorBody {
                documentation_url: None,
                errors: Some(errors.into_iter().map(|e| serde_json::json!(e)).collect()),
                message: "Failed to fetch user profile".to_string(),
            }));
        }

        if let Some(data) = query_response.data {
            if let Some(repository) = data.repository {
                return Ok(repository);
            }
        }

        Err(AppError::GitHubError(GitHubErrorBody {
            documentation_url: None,
            errors: None,
            message: "Failed to fetch repository".to_string(),
        }))
    }

    async fn get_languages(&self, username: &str, repository_name: &str) -> Result<HashMap<String, i32>, AppError> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/languages",
            username, repository_name
        );
        let response = self
            .client
            .request(Method::GET, &url)
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            Err(AppError::GitHubError(GitHubErrorBody {
                documentation_url: None,
                errors: None,
                message: format!(
                    "GitHub API error: status = {}, message = {}",
                    status, error_body
                ),
            }))
        } else {
            let languages = response.json::<HashMap<String, i32>>().await?;
            Ok(languages)
        }
    }

    async fn get_content_by_path(&self, username: &str, repository_name: &str, path: &str) -> Result<GitHubContentObject, AppError> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            username, repository_name, path
        );

        tracing::debug!("requesting github content from {}", url);

        let response = self
            .client
            .request(Method::GET, &url)
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            Err(AppError::GitHubError(GitHubErrorBody {
                documentation_url: None,
                errors: None,
                message: format!(
                    "GitHub API error: status = {}, message = {}",
                    status, error_body
                ),
            }))
        } else {
            let content = response
                .json::<GitHubContentObject>()
                .await
                .map_err(|err| {
                    tracing::error!("Error parsing GitHub content response: {:?}", err);
                    AppError::ParseConfigError(err.to_string())
                })?;

            Ok(content)
        }
    }
}

mock! {
    pub GitHubClient {}

    #[async_trait]
    impl GitHubClientTrait for GitHubClient {
        async fn get_user_events(
            &self,
            username: &str,
        ) -> Result<Vec<models::events::Event>, AppError>;

        async fn get_user_profile(&self) -> Result<profile_query::ProfileQueryViewer, AppError>;

        async fn get_repository(
            &self,
            username: &str,
            repository_name: &str,
        ) -> Result<repository_query::RepositoryQueryRepository, AppError>;

        async fn get_languages(
            &self,
            username: &str,
            repository_name: &str,
        ) -> Result<HashMap<String, i32>, AppError>;

        async fn get_content_by_path(
            &self,
            username: &str,
            repository_name: &str,
            path: &str,
        ) -> Result<GitHubContentObject, AppError>;
    }
}


#[cfg(test)]
mod tests {
    use mockall::predicate::eq;
    use url::Url;
    use crate::github_client::profile_query::{ProfileQueryViewerRepositories, ProfileQueryViewerRepositoriesPageInfo};
    use crate::models::events::{Actor, Repository};
    use super::*;

    #[tokio::test]
    async fn get_user_events() {
        let mut mock_client = MockGitHubClient::new();

        mock_client.expect_get_user_events()
            .with(eq("octocat"))
            .returning(|_| {
                Ok(vec![Event {
                    id: "42566153650".to_string(),
                    r#type: "PushEvent".to_string(),
                    actor: Actor {
                        id: 40726067,
                        login: "luxass".to_string(),
                        display_login: "luxass".to_string(),
                        gravatar_id: "".to_string(),
                        url: Url::parse("https://api.github.com/users/luxass").unwrap(),
                        avatar_url: Url::parse("https://avatars.githubusercontent.com/u/40726067?").unwrap(),
                    },
                    repo: Repository {
                        id: 692361166,
                        name: "luxass/mosaic".to_string(),
                        url: Url::parse("https://api.github.com/repos/luxass/mosaic").unwrap(),
                    },
                    public: true,
                    created_at: Default::default(),
                    payload: None,
                    org: None,
                }])
            });

        let result = mock_client.get_user_events("octocat").await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn get_user_events_empty() {
        let mut mock_client = MockGitHubClient::new();

        mock_client.expect_get_user_events()
            .with(eq("octocat"))
            .returning(|_| {
                Ok(vec![])
            });

        let result = mock_client.get_user_events("octocat").await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_languages() {
        let mut mock_client = MockGitHubClient::new();

        // Setup expected behavior
        mock_client.expect_get_languages()
            .with(eq("octocat"), eq("Hello-World"))
            .returning(|_, _| {
                let mut languages = HashMap::new();
                languages.insert("Rust".to_string(), 100);
                Ok(languages)
            });

        // Call the method
        let result = mock_client.get_languages("octocat", "Hello-World").await;

        // Assert the result
        assert!(result.is_ok());
        let languages = result.unwrap();
        assert_eq!(languages.get("Rust"), Some(&100));
    }

    #[tokio::test]
    async fn test_get_user_profile() {
        let mut mock_client = MockGitHubClient::new();

        // Setup expected behavior
        mock_client.expect_get_user_profile()
            .returning(|| {
                Ok(profile_query::ProfileQueryViewer {
                    // Populate profile fields
                    repositories: ProfileQueryViewerRepositories {
                        total_count: 0,
                        nodes: None,
                        page_info: ProfileQueryViewerRepositoriesPageInfo { end_cursor: None, has_next_page: false },
                    },
                })
            });

        // Call the method
        let result = mock_client.get_user_profile().await;

        // Assert the result
        assert!(result.is_ok());
    }
}