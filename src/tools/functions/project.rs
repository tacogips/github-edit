use anyhow::Result;

use crate::github::GitHubClient;
use crate::services::project_service::ProjectService;
use crate::types::project::{ProjectFieldValue, ProjectId};
use crate::types::{
    IssueNumber, ProjectFieldId, ProjectItemId, ProjectNodeId, PullRequestNumber, RepositoryId,
};

/// Update a project item field using typed field value
///
/// Single method that dispatches to appropriate GitHub client method based on field value type.
/// Updates a field value for a project item using the project node ID.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `project_node_id` - The project node identifier (GraphQL ID)
/// * `project_item_id` - The project item ID (GraphQL node ID)
/// * `project_field_id` - The field ID (GraphQL node ID)
/// * `value` - The field value with type information
///
/// # Returns
/// Returns `Ok(())` if the field was successfully updated
pub async fn update_project_item_field(
    github_client: &GitHubClient,
    project_node_id: &ProjectNodeId,
    project_item_id: &ProjectItemId,
    project_field_id: &ProjectFieldId,
    value: &ProjectFieldValue,
) -> Result<()> {
    let project_service = ProjectService::new(github_client.clone());
    project_service
        .update_project_item_field(project_node_id, project_item_id, project_field_id, value)
        .await
}

/// Get project node ID from project identifier
///
/// This method resolves a project identifier to its GitHub GraphQL node ID,
/// which is required for project field update operations.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `project_id` - The project identifier containing owner, project number, and project type
///
/// # Returns
/// Returns the project node ID (GraphQL ID) that can be used for project operations
///
/// # Errors
/// Returns an error if:
/// - The project does not exist or is not accessible
/// - The user does not have permission to access the project
/// - API rate limits are exceeded
/// - Network errors occur
pub async fn get_project_node_id(
    github_client: &GitHubClient,
    project_id: &ProjectId,
) -> Result<ProjectNodeId> {
    let project_service = ProjectService::new(github_client.clone());
    project_service.get_project_node_id(project_id).await
}

/// Update a project item field value using raw field value
///
/// This method provides direct access to the GitHub client's update_project_item_field_value method
/// for cases where you need to bypass the typed field value system.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `project_node_id` - The project node identifier (GraphQL ID)
/// * `project_item_id` - The project item ID (GraphQL node ID)
/// * `project_field_id` - The field ID (GraphQL node ID)
/// * `value` - The field value with type information
///
/// # Returns
/// Returns `Ok(())` if the field was successfully updated
pub async fn update_project_item_field_value(
    github_client: &GitHubClient,
    project_node_id: &ProjectNodeId,
    project_item_id: &ProjectItemId,
    project_field_id: &ProjectFieldId,
    value: &ProjectFieldValue,
) -> Result<()> {
    let project_service = ProjectService::new(github_client.clone());
    project_service
        .update_project_item_field_value(project_node_id, project_item_id, project_field_id, value)
        .await
}

/// Update a project item text field
///
/// Convenience method for updating text fields in GitHub Projects v2.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `project_node_id` - The project node identifier (GraphQL ID)
/// * `project_item_id` - The project item ID (GraphQL node ID)
/// * `project_field_id` - The field ID (GraphQL node ID)
/// * `text_value` - The new text value
///
/// # Returns
/// Returns `Ok(())` if the field was successfully updated
pub async fn update_project_item_text_field(
    github_client: &GitHubClient,
    project_node_id: &ProjectNodeId,
    project_item_id: &ProjectItemId,
    project_field_id: &ProjectFieldId,
    text_value: &str,
) -> Result<()> {
    let project_service = ProjectService::new(github_client.clone());
    project_service
        .update_project_item_text_field(
            project_node_id,
            project_item_id,
            project_field_id,
            text_value,
        )
        .await
}

/// Update a project item number field
///
/// Convenience method for updating number fields in GitHub Projects v2.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `project_node_id` - The project node identifier (GraphQL ID)
/// * `project_item_id` - The project item ID (GraphQL node ID)
/// * `project_field_id` - The field ID (GraphQL node ID)
/// * `number_value` - The new number value
///
/// # Returns
/// Returns `Ok(())` if the field was successfully updated
pub async fn update_project_item_number_field(
    github_client: &GitHubClient,
    project_node_id: &ProjectNodeId,
    project_item_id: &ProjectItemId,
    project_field_id: &ProjectFieldId,
    number_value: f64,
) -> Result<()> {
    let project_service = ProjectService::new(github_client.clone());
    project_service
        .update_project_item_number_field(
            project_node_id,
            project_item_id,
            project_field_id,
            number_value,
        )
        .await
}

/// Update a project item date field
///
/// Convenience method for updating date fields in GitHub Projects v2.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `project_node_id` - The project node identifier (GraphQL ID)
/// * `project_item_id` - The project item ID (GraphQL node ID)
/// * `project_field_id` - The field ID (GraphQL node ID)
/// * `date_value` - The new date value
///
/// # Returns
/// Returns `Ok(())` if the field was successfully updated
pub async fn update_project_item_date_field(
    github_client: &GitHubClient,
    project_node_id: &ProjectNodeId,
    project_item_id: &ProjectItemId,
    project_field_id: &ProjectFieldId,
    date_value: chrono::DateTime<chrono::Utc>,
) -> Result<()> {
    let project_service = ProjectService::new(github_client.clone());
    project_service
        .update_project_item_date_field(
            project_node_id,
            project_item_id,
            project_field_id,
            date_value,
        )
        .await
}

/// Update a project item single select field
///
/// Convenience method for updating single select fields in GitHub Projects v2.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `project_node_id` - The project node identifier (GraphQL ID)
/// * `project_item_id` - The project item ID (GraphQL node ID)
/// * `project_field_id` - The field ID (GraphQL node ID)
/// * `option_id` - The selected option ID (GraphQL node ID)
///
/// # Returns
/// Returns `Ok(())` if the field was successfully updated
pub async fn update_project_item_single_select_field(
    github_client: &GitHubClient,
    project_node_id: &ProjectNodeId,
    project_item_id: &ProjectItemId,
    project_field_id: &ProjectFieldId,
    option_id: &str,
) -> Result<()> {
    let project_service = ProjectService::new(github_client.clone());
    project_service
        .update_project_item_single_select_field(
            project_node_id,
            project_item_id,
            project_field_id,
            option_id,
        )
        .await
}

/// Add an issue to a project
///
/// Adds an existing issue to a GitHub Project v2 using the GraphQL API.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `project_node_id` - The project node identifier (GraphQL ID)
/// * `repository_id` - The repository identifier containing owner and repo name
/// * `issue_number` - The issue number to add to the project
///
/// # Returns
/// Returns `Ok(ProjectItemId)` with the new project item ID if successful
pub async fn add_issue_to_project(
    github_client: &GitHubClient,
    project_node_id: &ProjectNodeId,
    repository_id: &RepositoryId,
    issue_number: IssueNumber,
) -> Result<ProjectItemId> {
    let project_service = ProjectService::new(github_client.clone());
    project_service
        .add_issue_to_project(project_node_id, repository_id, issue_number)
        .await
}

/// Add a pull request to a project
///
/// Adds an existing pull request to a GitHub Project v2 using the GraphQL API.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `project_node_id` - The project node identifier (GraphQL ID)
/// * `repository_id` - The repository identifier containing owner and repo name
/// * `pull_request_number` - The pull request number to add to the project
///
/// # Returns
/// Returns `Ok(ProjectItemId)` with the new project item ID if successful
pub async fn add_pull_request_to_project(
    github_client: &GitHubClient,
    project_node_id: &ProjectNodeId,
    repository_id: &RepositoryId,
    pull_request_number: PullRequestNumber,
) -> Result<ProjectItemId> {
    let project_service = ProjectService::new(github_client.clone());
    project_service
        .add_pull_request_to_project(project_node_id, repository_id, pull_request_number)
        .await
}
