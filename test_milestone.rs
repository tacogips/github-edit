use github_edit::github::GitHubClient;
use github_edit::types::repository::{RepositoryId, RepositoryUrl};
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get GitHub token from environment
    let github_token = env::var("GITHUB_EDIT_GITHUB_TOKEN")?;

    // Create GitHub client
    let github_client = GitHubClient::new(Some(github_token), None)?;

    // Parse repository URL
    let repo_url = RepositoryUrl::new("https://github.com/tacogips/gitcodes-mcp-test-1".to_string());
    let repo_id = RepositoryId::parse_url(&repo_url).unwrap();

    println!("Repository ID: {}/{}", repo_id.owner().as_str(), repo_id.repo_name().as_str());

    // Test the milestone creation
    let result = github_client.create_milestone(
        &repo_id,
        "Test Milestone via Test Code",
        Some("A test milestone created using test code"),
        None,
        None,
    ).await;

    match result {
        Ok(milestone) => println!("Created milestone: {:?}", milestone),
        Err(e) => eprintln!("Error creating milestone: {:?}", e),
    }

    Ok(())
}