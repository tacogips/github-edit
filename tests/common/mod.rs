use github_edit::github::client::GitHubClient;
use std::env;

/// Create a GitHub client using the test token from environment variable
pub fn create_test_github_client() -> GitHubClient {
    let token = env::var("GITHUB_EDIT_GITHUB_TOKEN")
        .expect("GITHUB_EDIT_GITHUB_TOKEN environment variable must be set");

    GitHubClient::new(Some(token), None).expect("Failed to create GitHub client")
}
