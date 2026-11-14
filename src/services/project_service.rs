use crate::github::GitHubClient;
use crate::types::project::{ProjectFieldValue, ProjectId};
use crate::types::{
    IssueNumber, ProjectFieldId, ProjectItemId, ProjectNodeId, PullRequestNumber, RepositoryId,
};
use anyhow::Result;

/// Service layer for project operations
///
/// This service provides a high-level interface for managing GitHub project items,
/// encapsulating the underlying GitHub client operations with additional
/// business logic and error handling.
pub struct ProjectService {
    github_client: GitHubClient,
}

impl ProjectService {
    /// Create a new project service instance
    pub fn new(github_client: GitHubClient) -> Self {
        Self { github_client }
    }

    /// Update a project item field value
    ///
    /// Updates a field value for a project item using the project node ID.
    /// This method handles the direct update without project resolution.
    ///
    /// # Arguments
    /// * `project_node_id` - The project node identifier (GraphQL ID)
    /// * `item_id` - The project item ID (GraphQL node ID)
    /// * `field_id` - The field ID (GraphQL node ID)
    /// * `value` - The new field value
    ///
    /// # Returns
    /// Returns `Ok(())` if the field was successfully updated
    pub async fn update_project_item_field_value(
        &self,
        project_node_id: &ProjectNodeId,
        project_item_id: &ProjectItemId,
        project_field_id: &ProjectFieldId,
        value: &ProjectFieldValue,
    ) -> Result<()> {
        self.github_client
            .update_project_item_field_value(
                project_node_id,
                project_item_id,
                project_field_id,
                value,
            )
            .await
    }

    /// Update a project item field using typed field value
    ///
    /// Single method that dispatches to appropriate GitHub client method based on field value type.
    /// This replaces the individual type-specific methods with a unified interface.
    ///
    /// # Arguments
    /// * `project_node_id` - The project node identifier (GraphQL ID)
    /// * `item_id` - The project item ID (GraphQL node ID)
    /// * `field_id` - The field ID (GraphQL node ID)
    /// * `value` - The field value with type information
    ///
    /// # Returns
    /// Returns `Ok(())` if the field was successfully updated
    pub async fn update_project_item_field(
        &self,
        project_node_id: &ProjectNodeId,
        item_id: &ProjectItemId,
        field_id: &ProjectFieldId,
        value: &ProjectFieldValue,
    ) -> Result<()> {
        match value {
            ProjectFieldValue::Text(text_value) => {
                self.github_client
                    .update_project_item_text_field(project_node_id, item_id, field_id, text_value)
                    .await
            }
            ProjectFieldValue::Number(number_value) => {
                self.github_client
                    .update_project_item_number_field(
                        project_node_id,
                        item_id,
                        field_id,
                        *number_value,
                    )
                    .await
            }
            ProjectFieldValue::Date(date_value) => {
                self.github_client
                    .update_project_item_date_field(project_node_id, item_id, field_id, *date_value)
                    .await
            }
            ProjectFieldValue::SingleSelect(option_id) => {
                self.github_client
                    .update_project_item_single_select_field(
                        project_node_id,
                        item_id,
                        field_id,
                        option_id,
                    )
                    .await
            }
            ProjectFieldValue::MultiSelect(_) => {
                // MultiSelect is not supported by the current GitHub client methods
                Err(anyhow::anyhow!(
                    "MultiSelect field updates are not yet supported"
                ))
            }
        }
    }

    /// Update a project item text field
    ///
    /// Convenience method for updating text fields in GitHub Projects v2.
    /// Delegates to the unified update_project_item_field method.
    ///
    /// # Arguments
    /// * `project_node_id` - The project node identifier (GraphQL ID)
    /// * `item_id` - The project item ID (GraphQL node ID)
    /// * `field_id` - The field ID (GraphQL node ID)
    /// * `text_value` - The new text value
    ///
    /// # Returns
    /// Returns `Ok(())` if the field was successfully updated
    pub async fn update_project_item_text_field(
        &self,
        project_node_id: &ProjectNodeId,
        item_id: &ProjectItemId,
        field_id: &ProjectFieldId,
        text_value: &str,
    ) -> Result<()> {
        let value = ProjectFieldValue::Text(text_value.to_string());
        self.update_project_item_field(project_node_id, item_id, field_id, &value)
            .await
    }

    /// Update a project item number field
    ///
    /// Convenience method for updating number fields in GitHub Projects v2.
    /// Delegates to the unified update_project_item_field method.
    ///
    /// # Arguments
    /// * `project_node_id` - The project node identifier (GraphQL ID)
    /// * `item_id` - The project item ID (GraphQL node ID)
    /// * `field_id` - The field ID (GraphQL node ID)
    /// * `number_value` - The new number value
    ///
    /// # Returns
    /// Returns `Ok(())` if the field was successfully updated
    pub async fn update_project_item_number_field(
        &self,
        project_node_id: &ProjectNodeId,
        item_id: &ProjectItemId,
        field_id: &ProjectFieldId,
        number_value: f64,
    ) -> Result<()> {
        let value = ProjectFieldValue::Number(number_value);
        self.update_project_item_field(project_node_id, item_id, field_id, &value)
            .await
    }

    /// Update a project item date field
    ///
    /// Convenience method for updating date fields in GitHub Projects v2.
    /// Delegates to the unified update_project_item_field method.
    ///
    /// # Arguments
    /// * `project_node_id` - The project node identifier (GraphQL ID)
    /// * `item_id` - The project item ID (GraphQL node ID)
    /// * `field_id` - The field ID (GraphQL node ID)
    /// * `date_value` - The new date value
    ///
    /// # Returns
    /// Returns `Ok(())` if the field was successfully updated
    pub async fn update_project_item_date_field(
        &self,
        project_node_id: &ProjectNodeId,
        item_id: &ProjectItemId,
        field_id: &ProjectFieldId,
        date_value: chrono::DateTime<chrono::Utc>,
    ) -> Result<()> {
        let value = ProjectFieldValue::Date(date_value);
        self.update_project_item_field(project_node_id, item_id, field_id, &value)
            .await
    }

    /// Update a project item single select field
    ///
    /// Convenience method for updating single select fields in GitHub Projects v2.
    /// Delegates to the unified update_project_item_field method.
    ///
    /// # Arguments
    /// * `project_node_id` - The project node identifier (GraphQL ID)
    /// * `item_id` - The project item ID (GraphQL node ID)
    /// * `field_id` - The field ID (GraphQL node ID)
    /// * `option_id` - The selected option ID (GraphQL node ID)
    ///
    /// # Returns
    /// Returns `Ok(())` if the field was successfully updated
    pub async fn update_project_item_single_select_field(
        &self,
        project_node_id: &ProjectNodeId,
        item_id: &ProjectItemId,
        field_id: &ProjectFieldId,
        option_id: &str,
    ) -> Result<()> {
        let value = ProjectFieldValue::SingleSelect(option_id.to_string());
        self.update_project_item_field(project_node_id, item_id, field_id, &value)
            .await
    }

    /// Get project node ID from project identifier
    ///
    /// This method resolves a project identifier to its GitHub GraphQL node ID,
    /// which is required for project field update operations.
    ///
    /// # Arguments
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
    pub async fn get_project_node_id(&self, project_id: &ProjectId) -> Result<ProjectNodeId> {
        self.github_client.get_project_node_id(project_id).await
    }

    /// Add an issue to a project
    ///
    /// Adds an existing issue to a GitHub Project v2 using the GraphQL API.
    /// This creates a new project item linked to the specified issue.
    ///
    /// # Arguments
    /// * `project_node_id` - The project node identifier (GraphQL ID)
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `issue_number` - The issue number to add to the project
    ///
    /// # Returns
    /// Returns `Ok(ProjectItemId)` with the new project item ID if successful
    ///
    /// # Errors
    /// Returns an error if:
    /// - The project does not exist or is not accessible
    /// - The issue does not exist or is not accessible
    /// - The user does not have permission to edit the project
    /// - The issue is already in the project
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn add_issue_to_project(
        &self,
        project_node_id: &ProjectNodeId,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
    ) -> Result<ProjectItemId> {
        self.github_client
            .add_issue_to_project(project_node_id, repository_id, issue_number)
            .await
    }

    /// Add a pull request to a project
    ///
    /// Adds an existing pull request to a GitHub Project v2 using the GraphQL API.
    /// This creates a new project item linked to the specified pull request.
    ///
    /// # Arguments
    /// * `project_node_id` - The project node identifier (GraphQL ID)
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pull_request_number` - The pull request number to add to the project
    ///
    /// # Returns
    /// Returns `Ok(ProjectItemId)` with the new project item ID if successful
    ///
    /// # Errors
    /// Returns an error if:
    /// - The project does not exist or is not accessible
    /// - The pull request does not exist or is not accessible
    /// - The user does not have permission to edit the project
    /// - The pull request is already in the project
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn add_pull_request_to_project(
        &self,
        project_node_id: &ProjectNodeId,
        repository_id: &RepositoryId,
        pull_request_number: PullRequestNumber,
    ) -> Result<ProjectItemId> {
        self.github_client
            .add_pull_request_to_project(project_node_id, repository_id, pull_request_number)
            .await
    }
}
