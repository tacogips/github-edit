use crate::github::error::ApiRetryableError;

use anyhow::Result;
use octocrab::Octocrab;
use tokio::time::Duration;
use tokio::time::sleep;

/// Default maximum number of retry attempts for API operations
pub const DEFAULT_MAX_RETRY_COUNT: u32 = 15;

#[derive(Clone)]
pub struct GitHubClient {
    pub(crate) client: octocrab::Octocrab,
    pub(crate) token: Option<String>,
}

impl GitHubClient {
    pub fn new(token: Option<String>, _timeout: Option<Duration>) -> Result<Self> {
        let mut builder = Octocrab::builder();

        if let Some(ref token_str) = token {
            builder = builder.personal_token(token_str.clone());
        }

        let client = builder.build()?;
        Ok(GitHubClient { client, token })
    }

    pub fn octocrab(&self) -> &Octocrab {
        &self.client
    }
}

pub(crate) async fn retry_with_backoff<F, Fut, T>(
    operation_name: &str,
    max_retry_count: Option<u32>,
    execute_operation: F,
) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = std::result::Result<T, ApiRetryableError>>,
{
    let mut attempt = 0;
    let max_retries = max_retry_count.unwrap_or(DEFAULT_MAX_RETRY_COUNT);

    loop {
        match execute_operation().await {
            Ok(result) => {
                tracing::debug!(
                    "Operation {} succeeded on attempt {}",
                    operation_name,
                    attempt + 1
                );
                return Ok(result);
            }
            Err(e) => {
                tracing::warn!(
                    "Operation {} failed on attempt {}: {}",
                    operation_name,
                    attempt + 1,
                    e,
                );

                // Check if this is a non-retryable error
                match &e {
                    ApiRetryableError::NonRetryable(_) => {
                        tracing::debug!(
                            "Operation {} failed with non-retryable error, not retrying: {}",
                            operation_name,
                            e
                        );
                        return Err(anyhow::anyhow!(
                            "Operation {} failed: {}",
                            operation_name,
                            e
                        ));
                    }
                    ApiRetryableError::RateLimit => {
                        tracing::debug!(
                            "Operation {} hit rate limit, will retry with backoff",
                            operation_name
                        );
                    }
                    ApiRetryableError::Retryable(_) => {
                        tracing::debug!(
                            "Operation {} failed with retryable error, will retry",
                            operation_name
                        );
                    }
                }

                if attempt >= max_retries {
                    return Err(anyhow::anyhow!(
                        "Operation {} failed after {} attempts: {}",
                        operation_name,
                        attempt + 1,
                        e
                    ));
                }

                let delay = Duration::from_millis(100 * (1 << attempt));
                tracing::debug!(
                    "Retrying operation {} after delay: {:?}",
                    operation_name,
                    delay
                );

                sleep(delay).await;
                attempt += 1;
            }
        }
    }
}
