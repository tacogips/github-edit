//! Project-related tool definitions for GitHub repository operations
//!
//! This module contains MCP tool implementations for managing GitHub projects,
//! including project item field updates and project management operations.
//!
//! Note: This module does not contain any delete operations for safety reasons.

use crate::github::GitHubClient;
use crate::tools::functions;
use crate::types::issue::IssueNumber;
use crate::types::project::{
    ProjectCustomFieldType, ProjectFieldId, ProjectFieldValue, ProjectItemId, ProjectNodeId,
};

use rmcp::{Error as McpError, model::*};
use std::str::FromStr;

/// Project management tools implementation
pub struct ProjectTools;

impl ProjectTools {
    pub async fn update_project_item_field(
        github_client: &GitHubClient,
        project_node_id: String,
        project_item_id: String,
        project_field_id: String,
        field_type: String,
        value: String,
    ) -> Result<CallToolResult, McpError> {
        let typed_project_node_id = ProjectNodeId::new(project_node_id.clone());
        let typed_project_item_id = ProjectItemId::new(project_item_id.clone());
        let typed_project_field_id = ProjectFieldId::new(project_field_id.clone());

        let field_type_enum = match ProjectCustomFieldType::from_str(&field_type) {
            Ok(ft) => ft,
            Err(_) => {
                return Ok(CallToolResult {
                    content: vec![Content::text(format!(
                        "Unsupported field type '{}'. Supported types: text, number, date, single_select, multi_select",
                        field_type
                    ))],
                    is_error: Some(true),
                });
            }
        };

        let parsed_value = match ProjectFieldValue::from_string_with_type(&field_type_enum, &value)
        {
            Ok(pv) => pv,
            Err(e) => {
                return Ok(CallToolResult {
                    content: vec![Content::text(format!("Failed to parse field value: {}", e))],
                    is_error: Some(true),
                });
            }
        };

        match functions::project::update_project_item_field(
            github_client,
            &typed_project_node_id,
            &typed_project_item_id,
            &typed_project_field_id,
            &parsed_value,
        )
        .await
        {
            Ok(_) => Ok(CallToolResult {
                content: vec![Content::text(
                    "Project item field updated successfully".to_string(),
                )],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Failed to update project item field: {}",
                    e
                ))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn get_project_node_id(
        github_client: &GitHubClient,
        project_owner: String,
        project_number: u64,
        project_type: String,
    ) -> Result<CallToolResult, McpError> {
        use crate::types::project::{ProjectId, ProjectNumber, ProjectType};
        use crate::types::repository::Owner;

        let project_type_enum = match project_type.as_str() {
            "user" => ProjectType::User,
            "organization" => ProjectType::Organization,
            _ => {
                return Ok(CallToolResult {
                    content: vec![Content::text(format!(
                        "Unsupported project type '{}'. Supported types: user, organization",
                        project_type
                    ))],
                    is_error: Some(true),
                });
            }
        };

        let project_id = ProjectId::new(
            Owner(project_owner),
            ProjectNumber(project_number),
            project_type_enum,
        );

        match functions::project::get_project_node_id(github_client, &project_id).await {
            Ok(node_id) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Project node ID: {}",
                    node_id.value()
                ))],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Failed to get project node ID: {}",
                    e
                ))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn update_project_item_text_field(
        github_client: &GitHubClient,
        project_node_id: String,
        project_item_id: String,
        project_field_id: String,
        text_value: String,
    ) -> Result<CallToolResult, McpError> {
        let typed_project_node_id = ProjectNodeId::new(project_node_id);
        let typed_project_item_id = ProjectItemId::new(project_item_id);
        let typed_project_field_id = ProjectFieldId::new(project_field_id);

        match functions::project::update_project_item_text_field(
            github_client,
            &typed_project_node_id,
            &typed_project_item_id,
            &typed_project_field_id,
            &text_value,
        )
        .await
        {
            Ok(_) => Ok(CallToolResult {
                content: vec![Content::text(
                    "Project item text field updated successfully".to_string(),
                )],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Failed to update project item text field: {}",
                    e
                ))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn update_project_item_number_field(
        github_client: &GitHubClient,
        project_node_id: String,
        project_item_id: String,
        project_field_id: String,
        number_value: f64,
    ) -> Result<CallToolResult, McpError> {
        let typed_project_node_id = ProjectNodeId::new(project_node_id);
        let typed_project_item_id = ProjectItemId::new(project_item_id);
        let typed_project_field_id = ProjectFieldId::new(project_field_id);

        match functions::project::update_project_item_number_field(
            github_client,
            &typed_project_node_id,
            &typed_project_item_id,
            &typed_project_field_id,
            number_value,
        )
        .await
        {
            Ok(_) => Ok(CallToolResult {
                content: vec![Content::text(
                    "Project item number field updated successfully".to_string(),
                )],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Failed to update project item number field: {}",
                    e
                ))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn update_project_item_date_field(
        github_client: &GitHubClient,
        project_node_id: String,
        project_item_id: String,
        project_field_id: String,
        date_value: String,
    ) -> Result<CallToolResult, McpError> {
        let typed_project_node_id = ProjectNodeId::new(project_node_id);
        let typed_project_item_id = ProjectItemId::new(project_item_id);
        let typed_project_field_id = ProjectFieldId::new(project_field_id);

        let parsed_date = match date_value.parse::<chrono::DateTime<chrono::Utc>>() {
            Ok(date) => date,
            Err(e) => {
                return Ok(CallToolResult {
                    content: vec![Content::text(format!(
                        "Failed to parse date '{}': {}",
                        date_value, e
                    ))],
                    is_error: Some(true),
                });
            }
        };

        match functions::project::update_project_item_date_field(
            github_client,
            &typed_project_node_id,
            &typed_project_item_id,
            &typed_project_field_id,
            parsed_date,
        )
        .await
        {
            Ok(_) => Ok(CallToolResult {
                content: vec![Content::text(
                    "Project item date field updated successfully".to_string(),
                )],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Failed to update project item date field: {}",
                    e
                ))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn update_project_item_single_select_field(
        github_client: &GitHubClient,
        project_node_id: String,
        project_item_id: String,
        project_field_id: String,
        option_id: String,
    ) -> Result<CallToolResult, McpError> {
        let typed_project_node_id = ProjectNodeId::new(project_node_id);
        let typed_project_item_id = ProjectItemId::new(project_item_id);
        let typed_project_field_id = ProjectFieldId::new(project_field_id);

        match functions::project::update_project_item_single_select_field(
            github_client,
            &typed_project_node_id,
            &typed_project_item_id,
            &typed_project_field_id,
            &option_id,
        )
        .await
        {
            Ok(_) => Ok(CallToolResult {
                content: vec![Content::text(
                    "Project item single select field updated successfully".to_string(),
                )],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Failed to update project item single select field: {}",
                    e
                ))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn add_issue_to_project(
        github_client: &GitHubClient,
        project_node_id: String,
        repository_owner: String,
        repository_name: String,
        issue_number: IssueNumber,
    ) -> Result<CallToolResult, McpError> {
        use crate::types::RepositoryId;

        let typed_project_node_id = ProjectNodeId::new(project_node_id);
        let repository_id = RepositoryId::new(repository_owner, repository_name);
        let typed_issue_number = issue_number;

        match functions::project::add_issue_to_project(
            github_client,
            &typed_project_node_id,
            &repository_id,
            typed_issue_number,
        )
        .await
        {
            Ok(project_item_id) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Issue added to project successfully. Project item ID: {}",
                    project_item_id.value()
                ))],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Failed to add issue to project: {}",
                    e
                ))],
                is_error: Some(true),
            }),
        }
    }

    pub async fn add_pull_request_to_project(
        github_client: &GitHubClient,
        project_node_id: String,
        repository_owner: String,
        repository_name: String,
        pull_request_number: u64,
    ) -> Result<CallToolResult, McpError> {
        use crate::types::{PullRequestNumber, RepositoryId};

        let typed_project_node_id = ProjectNodeId::new(project_node_id);
        let repository_id = RepositoryId::new(repository_owner, repository_name);
        let typed_pr_number =
            PullRequestNumber::new(pull_request_number.try_into().map_err(|_| {
                McpError::invalid_params(
                    format!("Pull request number {} is too large", pull_request_number),
                    None,
                )
            })?);

        match functions::project::add_pull_request_to_project(
            github_client,
            &typed_project_node_id,
            &repository_id,
            typed_pr_number,
        )
        .await
        {
            Ok(project_item_id) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Pull request added to project successfully. Project item ID: {}",
                    project_item_id.value()
                ))],
                is_error: Some(false),
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![Content::text(format!(
                    "Failed to add pull request to project: {}",
                    e
                ))],
                is_error: Some(true),
            }),
        }
    }
}
