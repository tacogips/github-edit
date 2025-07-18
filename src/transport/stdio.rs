use crate::github::GitHubClient;
use crate::tools::GitEditTools;
use anyhow::Result;
use rmcp::ServiceExt;
use rmcp::transport::stdio;

/// Runs the MCP server in STDIN/STDOUT mode.
///
/// This mode is used when the server is launched as a subprocess by an MCP client,
/// communicating through standard input/output streams.
///
/// # Arguments
/// * `github_token` - Optional GitHub personal access token for API authentication
/// * `_timezone` - Optional timezone for displaying dates (unused after GraphQL removal)
///
/// # Returns
/// * `Result<()>` - Success when server shuts down cleanly, or error
pub async fn run_stdio_server(
    github_token: Option<String>,
    _timezone: Option<String>,
) -> Result<()> {
    // Create GitHub client
    let github_client = GitHubClient::new(github_token, None)?;

    // Create an instance of our GitHub code tools wrapper with the provided token
    let service = GitEditTools::new(github_client);

    // Initialize the service
    service.init().await?;

    // Use the new rust-sdk stdio transport implementation
    let server = service.serve(stdio()).await?;

    server.waiting().await?;
    Ok(())
}
