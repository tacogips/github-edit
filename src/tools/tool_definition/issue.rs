//! Issue-related tool definitions for GitHub repository operations
//!
//! This module contains MCP tool implementations for managing GitHub issues,
//! including creation, modification, comment management, and state updates.
//!
//! Note: Delete operations for issues and comments have been removed for safety reasons.

use crate::github::GitHubClient;
use crate::tools::functions;
use crate::types::User;
use crate::types::issue::{IssueCommentNumber, IssueNumber, IssueState};
use crate::types::label::Label;
use crate::types::repository::{MilestoneNumber, RepositoryId, RepositoryUrl};

use rmcp::{Error as McpError, model::*};

/// Issue management tools implementation
pub struct IssueTools;

impl IssueTools {
    pub async fn create_issue(
        github_client: &GitHubClient,
        repository_url: String,
        title: String,
        body: Option<String>,
        assignees: Option<Vec<String>>,
        labels: Option<Vec<String>>,
        milestone_number: Option<u64>,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;

        let user_assignees: Option<Vec<User>> = assignees.map(|a| {
            a.into_iter()
                .map(|username| User::new(username, None))
                .collect()
        });
        let label_objects: Option<Vec<Label>> =
            labels.map(|l| l.into_iter().map(|name| Label::from(name)).collect());
        let milestone: Option<MilestoneNumber> = milestone_number.map(MilestoneNumber::new);

        match functions::issue::create_issue(
            github_client,
            &repo_id,
            &title,
            body.as_deref(),
            user_assignees.as_deref(),
            label_objects.as_deref(),
            milestone,
        )
        .await
        {
            Ok(issue) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Issue created successfully: #{}\nTitle: {}\nState: {:?}",
                    issue.issue_id.number, issue.title, issue.state
                ))],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to create issue: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn add_comment_to_issue(
        github_client: &GitHubClient,
        repository_url: String,
        issue_number: IssueNumber,
        body: String,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let issue_num = issue_number;

        match functions::issue::add_comment(github_client, &repo_id, issue_num, &body).await {
            Ok(comment_number) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Comment added successfully: #{}",
                    comment_number
                ))],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to add comment: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn edit_comment_on_issue(
        github_client: &GitHubClient,
        repository_url: String,
        issue_number: IssueNumber,
        comment_number: IssueCommentNumber,
        body: String,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let issue_num = issue_number;
        let comment_num = comment_number;

        match functions::issue::edit_comment(github_client, &repo_id, issue_num, comment_num, &body)
            .await
        {
            Ok(_) => Ok(CallToolResult {
                content: vec![Content::text("Comment edited successfully".to_string())],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to edit comment: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn edit_issue_title(
        github_client: &GitHubClient,
        repository_url: String,
        issue_number: IssueNumber,
        title: String,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let issue_num = issue_number;

        match functions::issue::edit_title(github_client, &repo_id, issue_num, &title).await {
            Ok(_) => Ok(CallToolResult {
                content: vec![Content::text("Issue title edited successfully".to_string())],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to edit title: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn edit_issue_body(
        github_client: &GitHubClient,
        repository_url: String,
        issue_number: IssueNumber,
        body: String,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let issue_num = issue_number;

        match functions::issue::edit_body(github_client, &repo_id, issue_num, &body).await {
            Ok(_) => Ok(CallToolResult {
                content: vec![Content::text("Issue body edited successfully".to_string())],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to edit body: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn update_issue_state(
        github_client: &GitHubClient,
        repository_url: String,
        issue_number: IssueNumber,
        state: String,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let issue_num = issue_number;
        let issue_state = match state.to_lowercase().as_str() {
            "open" => IssueState::Open,
            "closed" => IssueState::Closed,
            _ => {
                return Ok(CallToolResult {
                    content: vec![Content::text(
                        "State must be 'open' or 'closed'".to_string(),
                    )],
                    is_error: Some(true),
                });
            }
        };

        match functions::issue::update_state(github_client, &repo_id, issue_num, issue_state).await
        {
            Ok(_) => Ok(CallToolResult {
                content: vec![Content::text(
                    "Issue state updated successfully".to_string(),
                )],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to update state: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn add_assignees_to_issue(
        github_client: &GitHubClient,
        repository_url: String,
        issue_number: IssueNumber,
        new_assignees: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let issue_num = issue_number;

        match functions::issue::add_assignees(github_client, &repo_id, issue_num, &new_assignees)
            .await
        {
            Ok((added, skipped)) => {
                let mut result = vec![];
                if !added.is_empty() {
                    result.push(format!("Added assignees: {}", added.join(", ")));
                }
                if !skipped.is_empty() {
                    result.push(format!(
                        "Skipped (already assigned): {}",
                        skipped.join(", ")
                    ));
                }
                Ok(CallToolResult {
                    content: vec![Content::text(if result.is_empty() {
                        "No changes made to assignees".to_string()
                    } else {
                        result.join("; ")
                    })],
                    is_error: Some(false),
                })
            }
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to add assignees: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn remove_assignees_from_issue(
        github_client: &GitHubClient,
        repository_url: String,
        issue_number: IssueNumber,
        assignees: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let issue_num = issue_number;

        match functions::issue::remove_assignees(github_client, &repo_id, issue_num, &assignees)
            .await
        {
            Ok((removed, skipped)) => {
                let mut result = vec![];
                if !removed.is_empty() {
                    result.push(format!("Removed assignees: {}", removed.join(", ")));
                }
                if !skipped.is_empty() {
                    result.push(format!("Skipped (not assigned): {}", skipped.join(", ")));
                }
                Ok(CallToolResult {
                    content: vec![Content::text(if result.is_empty() {
                        "No changes made to assignees".to_string()
                    } else {
                        result.join("; ")
                    })],
                    is_error: Some(false),
                })
            }
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to remove assignees: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn remove_labels_from_issue(
        github_client: &GitHubClient,
        repository_url: String,
        issue_number: IssueNumber,
        labels: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let issue_num = issue_number;
        let label_objects: Vec<Label> = labels.into_iter().map(|name| Label::from(name)).collect();

        match functions::issue::remove_labels(github_client, &repo_id, issue_num, &label_objects)
            .await
        {
            Ok((removed, skipped)) => {
                let mut result = vec![];
                if !removed.is_empty() {
                    result.push(format!(
                        "Removed labels: {}",
                        removed
                            .iter()
                            .map(|l| l.name.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
                if !skipped.is_empty() {
                    result.push(format!(
                        "Skipped (not assigned): {}",
                        skipped
                            .iter()
                            .map(|l| l.name.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
                Ok(CallToolResult {
                    content: vec![Content::text(if result.is_empty() {
                        "No changes made to labels".to_string()
                    } else {
                        result.join("; ")
                    })],
                    is_error: Some(false),
                })
            }
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to remove labels: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn add_labels_to_issue(
        github_client: &GitHubClient,
        repository_url: String,
        issue_number: IssueNumber,
        labels: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let issue_num = issue_number;
        let label_objects: Vec<Label> = labels.into_iter().map(|name| Label::from(name)).collect();

        match functions::issue::add_labels(github_client, &repo_id, issue_num, &label_objects).await
        {
            Ok((added_labels, skipped_labels)) => {
                let mut result = vec![];
                if !added_labels.is_empty() {
                    result.push(format!(
                        "Added labels: {}",
                        added_labels
                            .iter()
                            .map(|l| l.name.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
                if !skipped_labels.is_empty() {
                    result.push(format!(
                        "Skipped (already assigned): {}",
                        skipped_labels
                            .iter()
                            .map(|l| l.name.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
                Ok(CallToolResult {
                    content: vec![Content::text(if result.is_empty() {
                        "No changes made to labels".to_string()
                    } else {
                        result.join("; ")
                    })],
                    is_error: Some(false),
                })
            }
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to add labels: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn add_milestone_to_issue(
        github_client: &GitHubClient,
        repository_url: String,
        issue_number: IssueNumber,
        milestone_number: u64,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let issue_num = issue_number;
        let milestone = MilestoneNumber::new(milestone_number);

        match functions::issue::set_milestone(github_client, &repo_id, issue_num, milestone).await {
            Ok(_) => Ok(CallToolResult {
                content: vec![Content::text("Milestone added successfully".to_string())],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to add milestone: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn remove_milestone_from_issue(
        github_client: &GitHubClient,
        repository_url: String,
        issue_number: IssueNumber,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let issue_num = issue_number;

        match functions::issue::remove_milestone(github_client, &repo_id, issue_num).await {
            Ok(_) => Ok(CallToolResult {
                content: vec![Content::text("Milestone removed successfully".to_string())],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to remove milestone: {}", e))],
                is_error: Some(true),
            }),
        }
    }
}
