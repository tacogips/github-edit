//! Repository-related tool definitions for GitHub repository operations
//!
//! This module contains MCP tool implementations for managing GitHub repositories,
//! including milestone creation and repository management operations.
//!
//! Note: This module does not contain any delete operations for safety reasons.

use chrono::{DateTime, Utc};
use rmcp::{Error as McpError, model::*};

use crate::github::GitHubClient;
use crate::tools::functions::repository;
use crate::types::milestone::MilestoneState;
use crate::types::repository::{RepositoryId, RepositoryUrl};

/// Repository-related tool implementations
pub struct RepositoryTools;

impl RepositoryTools {
    /// Create a new label in a repository
    pub async fn create_label(
        github_client: &GitHubClient,
        repository_url: String,
        name: String,
        color: Option<String>,
        description: Option<String>,
    ) -> Result<CallToolResult, McpError> {
        let repo_id =
            RepositoryId::parse_url(&RepositoryUrl(repository_url.clone())).map_err(|e| {
                McpError::invalid_request(format!("Invalid repository URL: {}", e), None)
            })?;

        match repository::create_label(
            github_client,
            &repo_id,
            &name,
            color.as_deref(),
            description.as_deref(),
        )
        .await
        {
            Ok(label) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Created label '{}' with color '{}' in repository {}",
                    label.name,
                    label.color(),
                    repository_url
                ))],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to create label: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    /// Update an existing label in a repository
    pub async fn update_label(
        github_client: &GitHubClient,
        repository_url: String,
        old_name: String,
        new_name: Option<String>,
        color: Option<String>,
        description: Option<String>,
    ) -> Result<CallToolResult, McpError> {
        let repo_id =
            RepositoryId::parse_url(&RepositoryUrl(repository_url.clone())).map_err(|e| {
                McpError::invalid_request(format!("Invalid repository URL: {}", e), None)
            })?;

        match repository::update_label(
            github_client,
            &repo_id,
            &old_name,
            new_name.as_deref(),
            color.as_deref(),
            description.as_deref(),
        )
        .await
        {
            Ok(label) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Updated label '{}' with color '{}' in repository {}",
                    label.name,
                    label.color(),
                    repository_url
                ))],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to update label: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    /// Create a new milestone in a repository
    pub async fn create_milestone(
        github_client: &GitHubClient,
        repository_url: String,
        title: String,
        description: Option<String>,
        due_on: Option<String>,
        state: Option<String>,
    ) -> Result<CallToolResult, McpError> {
        let repo_id =
            RepositoryId::parse_url(&RepositoryUrl(repository_url.clone())).map_err(|e| {
                McpError::invalid_request(format!("Invalid repository URL: {}", e), None)
            })?;

        let due_date = match due_on {
            Some(date_str) => {
                let parsed_date = DateTime::parse_from_rfc3339(&date_str).map_err(|e| {
                    McpError::invalid_request(format!("Invalid date format: {}", e), None)
                })?;
                Some(parsed_date.with_timezone(&Utc))
            }
            None => None,
        };

        let milestone_state = match state {
            Some(state_str) => {
                let state = match state_str.to_lowercase().as_str() {
                    "open" => MilestoneState::Open,
                    "closed" => MilestoneState::Closed,
                    _ => {
                        return Err(McpError::invalid_request(
                            "State must be 'open' or 'closed'".to_string(),
                            None,
                        ));
                    }
                };
                Some(state)
            }
            None => None,
        };

        match repository::create_milestone(
            github_client,
            &repo_id,
            &title,
            description.as_deref(),
            due_date,
            milestone_state,
        )
        .await
        {
            Ok(milestone) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Created milestone '{}' with ID {} in repository {}",
                    milestone.title, milestone.id.0, repository_url
                ))],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to create milestone: {}", e))],
                is_error: Some(true),
            }),
        }
    }
}
