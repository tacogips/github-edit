//! Pull request related tool definitions for GitHub repository operations
//!
//! This module contains MCP tool implementations for managing GitHub pull requests,
//! including creation, modification, comment management, and metadata updates.
//!
//! Note: Delete operations for pull request comments have been removed for safety reasons.

use crate::github::GitHubClient;
use crate::tools::functions;
use crate::types::label::Label;
use crate::types::pull_request::{Branch, PullRequestCommentNumber, PullRequestNumber};
use crate::types::repository::{MilestoneNumber, RepositoryId, RepositoryUrl};

use rmcp::{Error as McpError, model::*};

/// Pull request management tools implementation
pub struct PullRequestTools;

impl PullRequestTools {
    pub async fn create_pull_request(
        github_client: &GitHubClient,
        repository_url: String,
        title: String,
        head_branch: String,
        base_branch: String,
        body: Option<String>,
        draft: Option<bool>,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let head = Branch::new(head_branch);
        let base = Branch::new(base_branch);

        match functions::pull_request::create_pull_request(
            github_client,
            &repo_id,
            &title,
            &head,
            &base,
            body.as_deref(),
            draft,
        )
        .await
        {
            Ok(pr) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Pull request created successfully: #{}\nTitle: {}\nStatus: {:?}",
                    pr.pull_request_id.number, pr.title, pr.state
                ))],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Failed to create pull request: {}",
                    e
                ))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn add_comment_to_pull_request(
        github_client: &GitHubClient,
        repository_url: String,
        pr_number: u64,
        body: String,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let pr_num = PullRequestNumber::new(pr_number as u32);

        match functions::pull_request::add_comment(github_client, &repo_id, pr_num, &body).await {
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

    pub async fn edit_comment_on_pull_request(
        github_client: &GitHubClient,
        repository_url: String,
        pr_number: u64,
        comment_number: PullRequestCommentNumber,
        body: String,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let pr_num = PullRequestNumber::new(pr_number as u32);
        let comment_num = comment_number;

        match functions::pull_request::edit_comment(
            github_client,
            &repo_id,
            pr_num,
            comment_num,
            &body,
        )
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

    pub async fn close_pull_request(
        github_client: &GitHubClient,
        repository_url: String,
        pr_number: u64,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let pr_num = PullRequestNumber::new(pr_number as u32);

        match functions::pull_request::close_pull_request(github_client, &repo_id, pr_num).await {
            Ok(_) => Ok(CallToolResult {
                content: vec![Content::text(
                    "Pull request closed successfully".to_string(),
                )],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Failed to close pull request: {}",
                    e
                ))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn edit_pull_request_title(
        github_client: &GitHubClient,
        repository_url: String,
        pr_number: u64,
        title: String,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let pr_num = PullRequestNumber::new(pr_number as u32);

        match functions::pull_request::edit_title(github_client, &repo_id, pr_num, &title).await {
            Ok(_) => Ok(CallToolResult {
                content: vec![Content::text(
                    "Pull request title edited successfully".to_string(),
                )],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to edit title: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn edit_pull_request_body(
        github_client: &GitHubClient,
        repository_url: String,
        pr_number: u64,
        body: String,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let pr_num = PullRequestNumber::new(pr_number as u32);

        match functions::pull_request::edit_body(github_client, &repo_id, pr_num, &body).await {
            Ok(_) => Ok(CallToolResult {
                content: vec![Content::text(
                    "Pull request body edited successfully".to_string(),
                )],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to edit body: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn add_assignees_to_pull_request(
        github_client: &GitHubClient,
        repository_url: String,
        pr_number: u64,
        new_assignees: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let pr_num = PullRequestNumber::new(pr_number as u32);

        match functions::pull_request::add_assignees(
            github_client,
            &repo_id,
            pr_num,
            &new_assignees,
        )
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

    pub async fn remove_assignees_from_pull_request(
        github_client: &GitHubClient,
        repository_url: String,
        pr_number: u64,
        assignees: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let pr_num = PullRequestNumber::new(pr_number as u32);

        match functions::pull_request::remove_assignees(github_client, &repo_id, pr_num, &assignees)
            .await
        {
            Ok(_) => Ok(CallToolResult {
                content: vec![Content::text("Assignees removed successfully".to_string())],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to remove assignees: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn add_requested_reviewers_to_pull_request(
        github_client: &GitHubClient,
        repository_url: String,
        pr_number: u64,
        new_reviewers: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let pr_num = PullRequestNumber::new(pr_number as u32);

        match functions::pull_request::add_requested_reviewers(
            github_client,
            &repo_id,
            pr_num,
            &new_reviewers,
        )
        .await
        {
            Ok((added, skipped)) => {
                let mut result = vec![];
                if !added.is_empty() {
                    result.push(format!("Added reviewers: {}", added.join(", ")));
                }
                if !skipped.is_empty() {
                    result.push(format!(
                        "Skipped (already requested): {}",
                        skipped.join(", ")
                    ));
                }
                Ok(CallToolResult {
                    content: vec![Content::text(if result.is_empty() {
                        "No changes made to reviewers".to_string()
                    } else {
                        result.join("; ")
                    })],
                    is_error: Some(false),
                })
            }
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to add reviewers: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn add_labels_to_pull_request(
        github_client: &GitHubClient,
        repository_url: String,
        pr_number: u64,
        labels: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let pr_num = PullRequestNumber::new(pr_number as u32);
        let label_objects: Vec<Label> = labels.into_iter().map(|name| Label::from(name)).collect();

        match functions::pull_request::add_labels(github_client, &repo_id, pr_num, &label_objects)
            .await
        {
            Ok(_) => Ok(CallToolResult {
                content: vec![Content::text("Labels added successfully".to_string())],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to add labels: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn remove_labels_from_pull_request(
        github_client: &GitHubClient,
        repository_url: String,
        pr_number: u64,
        labels: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let pr_num = PullRequestNumber::new(pr_number as u32);
        let label_objects: Vec<Label> = labels.into_iter().map(|name| Label::from(name)).collect();

        match functions::pull_request::remove_labels(
            github_client,
            &repo_id,
            pr_num,
            &label_objects,
        )
        .await
        {
            Ok(_) => Ok(CallToolResult {
                content: vec![Content::text("Labels removed successfully".to_string())],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!("Failed to remove labels: {}", e))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn add_milestone_to_pull_request(
        github_client: &GitHubClient,
        repository_url: String,
        pr_number: u64,
        milestone_number: u64,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let pr_num = PullRequestNumber::new(pr_number as u32);
        let milestone = MilestoneNumber::new(milestone_number);

        match functions::pull_request::add_milestone(github_client, &repo_id, pr_num, milestone)
            .await
        {
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

    pub async fn remove_milestone_from_pull_request(
        github_client: &GitHubClient,
        repository_url: String,
        pr_number: u64,
    ) -> Result<CallToolResult, McpError> {
        let repo_id = RepositoryId::parse_url(&RepositoryUrl(repository_url)).map_err(|e| {
            McpError::invalid_request(format!("Invalid repository ID: {}", e), None)
        })?;
        let pr_num = PullRequestNumber::new(pr_number as u32);

        match functions::pull_request::remove_milestone(github_client, &repo_id, pr_num).await {
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
