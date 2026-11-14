use crate::{github::GitHubClient, tools::GitEditTools};
use anyhow::Result;
use rmcp::transport::sse_server::SseServer;
use std::net::SocketAddr;

pub struct SseServerApp {
    bind_addr: SocketAddr,
    github_token: Option<String>,
    timezone: Option<String>,
}

impl SseServerApp {
    /// Creates a new SSE server application instance.
    ///
    /// # Arguments
    ///
    /// * `bind_addr` - The socket address to bind the server to
    /// * `github_token` - Optional GitHub personal access token for API authentication
    ///
    /// # Returns
    ///
    /// Returns a new SseServerApp instance.
    pub fn new(
        bind_addr: SocketAddr,
        github_token: Option<String>,
        timezone: Option<String>,
    ) -> Self {
        Self {
            bind_addr,
            github_token,
            timezone,
        }
    }

    /// Starts the SSE server and serves GitInsightTools over Server-Sent Events.
    ///
    /// This method starts the server and waits for a Ctrl+C signal to shutdown gracefully.
    ///
    /// # Returns
    ///
    /// Returns Ok(()) when the server shuts down gracefully.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The server fails to bind to the specified address
    /// - The server encounters an error during operation
    pub async fn serve(self) -> Result<()> {
        // Initialize the service before starting the server
        tracing::info!("Initializing GitInsight service before starting SSE server...");
        let github_client = GitHubClient::new(self.github_token.clone(), None)?;
        let init_service = GitEditTools::new(github_client);
        init_service.init().await?;
        tracing::info!("GitInsight service initialization complete");

        let sse_server = SseServer::serve(self.bind_addr).await?;
        let github_token = self.github_token.clone();
        let _timezone = self.timezone.clone();
        let cancellation_token = sse_server.with_service(move || {
            let github_client = GitHubClient::new(github_token.clone(), None).unwrap();
            GitEditTools::new(github_client)
        });

        // Wait for Ctrl+C signal to gracefully shutdown
        tokio::signal::ctrl_c().await?;

        // Cancel the server
        cancellation_token.cancel();

        tracing::info!("SSE server shutdown complete");
        Ok(())
    }
}
