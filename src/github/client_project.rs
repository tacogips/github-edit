use crate::github::client::{GitHubClient, retry_with_backoff};
use crate::github::error::ApiRetryableError;
use crate::types::project::{ProjectFieldValue, ProjectId};
use crate::types::{
    IssueNumber, ProjectFieldId, ProjectItemId, ProjectNodeId, PullRequestNumber, RepositoryId,
};

use anyhow::Result;
use serde_json::json;

impl GitHubClient {
    /// Update a project item field value using GraphQL API
    ///
    /// This method updates various field types in GitHub Projects v2:
    /// - Text fields: Set text content
    /// - Number fields: Set numeric values
    /// - Date fields: Set date values (ISO 8601 format)
    /// - Single select fields: Set selected option ID
    /// - Iteration fields: Set iteration values
    ///
    /// # Arguments
    /// * `project_node_id` - The project node identifier (GraphQL ID)
    /// * `item_id` - The project item ID (GraphQL node ID)
    /// * `field_id` - The field ID (GraphQL node ID)
    /// * `value` - The new field value
    ///
    /// # Returns
    /// Returns `Ok(())` if the field was successfully updated
    ///
    /// # Errors
    /// Returns an error if:
    /// - The project, item, or field does not exist or is not accessible
    /// - The field type doesn't match the provided value type
    /// - The user does not have permission to edit the project
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn update_project_item_field_value(
        &self,
        project_node_id: &ProjectNodeId,
        project_item_id: &ProjectItemId,
        project_field_id: &ProjectFieldId,
        value: &ProjectFieldValue,
    ) -> Result<()> {
        let operation_name = "update_project_item_field_value";

        retry_with_backoff(operation_name, None, || async {
            self.update_project_item_field_value_impl(
                project_node_id,
                project_item_id,
                project_field_id,
                value,
            )
            .await
        })
        .await
    }

    async fn update_project_item_field_value_impl(
        &self,
        project_node_id: &ProjectNodeId,
        project_item_id: &ProjectItemId,
        project_field_id: &ProjectFieldId,
        value: &ProjectFieldValue,
    ) -> std::result::Result<(), ApiRetryableError> {
        // Build the GraphQL mutation based on field value type
        let mutation = match value {
            ProjectFieldValue::Text(text_value) => {
                format!(
                    r#"
                    mutation {{
                        updateProjectV2ItemFieldValue(input: {{
                            projectId: "{}"
                            itemId: "{}"
                            fieldId: "{}"
                            value: {{
                                text: "{}"
                            }}
                        }}) {{
                            projectV2Item {{
                                id
                            }}
                        }}
                    }}
                    "#,
                    project_node_id.value(),
                    project_item_id.value(),
                    project_field_id.value(),
                    text_value.replace('"', "\\\"") // Escape quotes in text
                )
            }
            ProjectFieldValue::Number(number_value) => {
                format!(
                    r#"
                    mutation {{
                        updateProjectV2ItemFieldValue(input: {{
                            projectId: "{}"
                            itemId: "{}"
                            fieldId: "{}"
                            value: {{
                                number: {}
                            }}
                        }}) {{
                            projectV2Item {{
                                id
                            }}
                        }}
                    }}
                    "#,
                    project_node_id.value(),
                    project_item_id.value(),
                    project_field_id.value(),
                    number_value
                )
            }
            ProjectFieldValue::Date(date_value) => {
                format!(
                    r#"
                    mutation {{
                        updateProjectV2ItemFieldValue(input: {{
                            projectId: "{}"
                            itemId: "{}"
                            fieldId: "{}"
                            value: {{
                                date: "{}"
                            }}
                        }}) {{
                            projectV2Item {{
                                id
                            }}
                        }}
                    }}
                    "#,
                    project_node_id.value(),
                    project_item_id.value(),
                    project_field_id.value(),
                    date_value.to_rfc3339()
                )
            }
            ProjectFieldValue::SingleSelect(select_value) => {
                format!(
                    r#"
                    mutation {{
                        updateProjectV2ItemFieldValue(input: {{
                            projectId: "{}"
                            itemId: "{}"
                            fieldId: "{}"
                            value: {{
                                singleSelectOptionId: "{}"
                            }}
                        }}) {{
                            projectV2Item {{
                                id
                            }}
                        }}
                    }}
                    "#,
                    project_node_id.value(),
                    project_item_id.value(),
                    project_field_id.value(),
                    select_value
                )
            }
            ProjectFieldValue::MultiSelect(_) => {
                // Multi-select is not supported by updateProjectV2ItemFieldValue
                // Would need separate handling or different mutation
                return Err(ApiRetryableError::NonRetryable(
                    "Multi-select fields are not supported by updateProjectV2ItemFieldValue"
                        .to_string(),
                ));
            }
        };

        // Execute GraphQL mutation
        let response = self
            .client
            .graphql::<serde_json::Value>(&json!({
                "query": mutation
            }))
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        // Check if the mutation was successful
        if response.get("data").is_some() && response.get("errors").is_none() {
            Ok(())
        } else {
            let error_msg = response
                .get("errors")
                .and_then(|errors| errors.as_array())
                .and_then(|arr| arr.first())
                .and_then(|error| error.get("message"))
                .and_then(|msg| msg.as_str())
                .unwrap_or("Unknown GraphQL error");

            Err(ApiRetryableError::NonRetryable(format!(
                "Failed to update project item field value: {}",
                error_msg
            )))
        }
    }

    /// Get project node ID from project identifier
    pub async fn get_project_node_id(&self, project_id: &ProjectId) -> Result<ProjectNodeId> {
        let owner = project_id.owner().as_str();
        let number = project_id.project_number().value();
        let project_type = project_id.project_type();

        let query = match project_type {
            crate::types::project::ProjectType::User => {
                format!(
                    r#"
                    query {{
                        user(login: "{}") {{
                            projectV2(number: {}) {{
                                id
                            }}
                        }}
                    }}
                    "#,
                    owner, number
                )
            }
            crate::types::project::ProjectType::Organization => {
                format!(
                    r#"
                    query {{
                        organization(login: "{}") {{
                            projectV2(number: {}) {{
                                id
                            }}
                        }}
                    }}
                    "#,
                    owner, number
                )
            }
        };

        let response = self
            .client
            .graphql::<serde_json::Value>(&json!({
                "query": query
            }))
            .await?;

        // Extract project node ID from response
        let node_id = response
            .get("data")
            .and_then(|data| match project_type {
                crate::types::project::ProjectType::User => data
                    .get("user")
                    .and_then(|user| user.get("projectV2"))
                    .and_then(|project| project.get("id")),
                crate::types::project::ProjectType::Organization => data
                    .get("organization")
                    .and_then(|org| org.get("projectV2"))
                    .and_then(|project| project.get("id")),
            })
            .and_then(|id| id.as_str())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Failed to get project node ID for project {}/{}",
                    owner,
                    number
                )
            })?;

        Ok(ProjectNodeId::new(node_id.to_string()))
    }

    /// Update a project item text field value
    ///
    /// Convenience method for updating text fields in GitHub Projects v2.
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
        self.update_project_item_field_value(project_node_id, item_id, field_id, &value)
            .await
    }

    /// Update a project item number field value
    ///
    /// Convenience method for updating number fields in GitHub Projects v2.
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
        self.update_project_item_field_value(project_node_id, item_id, field_id, &value)
            .await
    }

    /// Update a project item date field value
    ///
    /// Convenience method for updating date fields in GitHub Projects v2.
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
        self.update_project_item_field_value(project_node_id, item_id, field_id, &value)
            .await
    }

    /// Update a project item single select field value
    ///
    /// Convenience method for updating single select fields in GitHub Projects v2.
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
        self.update_project_item_field_value(project_node_id, item_id, field_id, &value)
            .await
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
        let operation_name = "add_issue_to_project";

        retry_with_backoff(operation_name, None, || async {
            self.add_issue_to_project_impl(project_node_id, repository_id, issue_number)
                .await
        })
        .await
    }

    /// Add a pull request to a GitHub project
    ///
    /// This method adds an existing pull request to a GitHub project by:
    /// 1. Fetching the pull request from the repository to get its node ID
    /// 2. Using GitHub's GraphQL API to add the pull request to the project
    ///
    /// # Arguments
    /// * `project_node_id` - The project node identifier (GraphQL ID)
    /// * `repository_id` - The repository containing the pull request
    /// * `pull_request_number` - The pull request number to add
    ///
    /// # Returns
    /// Returns the project item ID if the pull request was successfully added
    ///
    /// # Errors
    /// Returns an error if:
    /// - The project does not exist or is not accessible
    /// - The pull request does not exist in the repository
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
        let operation_name = "add_pull_request_to_project";

        retry_with_backoff(operation_name, None, || async {
            self.add_pull_request_to_project_impl(
                project_node_id,
                repository_id,
                pull_request_number,
            )
            .await
        })
        .await
    }

    async fn add_issue_to_project_impl(
        &self,
        project_node_id: &ProjectNodeId,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
    ) -> std::result::Result<ProjectItemId, ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = issue_number.value();

        let octocrab_issue = self
            .client
            .issues(owner, repo)
            .get(number.into())
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        let issue_node_id = octocrab_issue.node_id;

        let mutation = format!(
            r#"
            mutation {{
                addProjectV2ItemById(input: {{
                    projectId: "{}"
                    contentId: "{}"
                }}) {{
                    item {{
                        id
                    }}
                }}
            }}
            "#,
            project_node_id.value(),
            issue_node_id
        );

        let response = self
            .client
            .graphql::<serde_json::Value>(&json!({
                "query": mutation
            }))
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        if let Some(data) = response.get("data") {
            if let Some(item_id) = data
                .get("addProjectV2ItemById")
                .and_then(|add_result| add_result.get("item"))
                .and_then(|item| item.get("id"))
                .and_then(|id| id.as_str())
            {
                return Ok(ProjectItemId::new(item_id.to_string()));
            }
        }

        let error_msg = response
            .get("errors")
            .and_then(|errors| errors.as_array())
            .and_then(|arr| arr.first())
            .and_then(|error| error.get("message"))
            .and_then(|msg| msg.as_str())
            .unwrap_or("Unknown GraphQL error");

        Err(ApiRetryableError::NonRetryable(format!(
            "Failed to add issue to project: {}",
            error_msg
        )))
    }

    async fn add_pull_request_to_project_impl(
        &self,
        project_node_id: &ProjectNodeId,
        repository_id: &RepositoryId,
        pull_request_number: PullRequestNumber,
    ) -> std::result::Result<ProjectItemId, ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = pull_request_number.value();

        let octocrab_pull_request = self
            .client
            .pulls(owner, repo)
            .get(number.into())
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        let pull_request_node_id = octocrab_pull_request.node_id.ok_or_else(|| {
            ApiRetryableError::NonRetryable(format!(
                "Pull request {}/{}/{} does not have a node_id",
                owner, repo, number
            ))
        })?;

        let mutation = format!(
            r#"
            mutation {{
                addProjectV2ItemById(input: {{
                    projectId: "{}"
                    contentId: "{}"
                }}) {{
                    item {{
                        id
                    }}
                }}
            }}
            "#,
            project_node_id.value(),
            pull_request_node_id
        );

        let response = self
            .client
            .graphql::<serde_json::Value>(&json!({
                "query": mutation
            }))
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        if let Some(data) = response.get("data") {
            if let Some(item_id) = data
                .get("addProjectV2ItemById")
                .and_then(|add_result| add_result.get("item"))
                .and_then(|item| item.get("id"))
                .and_then(|id| id.as_str())
            {
                return Ok(ProjectItemId::new(item_id.to_string()));
            }
        }

        let error_msg = response
            .get("errors")
            .and_then(|errors| errors.as_array())
            .and_then(|arr| arr.first())
            .and_then(|error| error.get("message"))
            .and_then(|msg| msg.as_str())
            .unwrap_or("Unknown GraphQL error");

        Err(ApiRetryableError::NonRetryable(format!(
            "Failed to add pull request to project: {}",
            error_msg
        )))
    }
}
