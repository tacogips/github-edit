use crate::github::client::{GitHubClient, retry_with_backoff};
use crate::github::error::ApiRetryableError;
use crate::types::label::Label;
use crate::types::milestone::{Milestone, MilestoneState};
use crate::types::repository::{MilestoneNumber, RepositoryId};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GitHubMilestoneResponse {
    id: u64,
    number: u32,
    title: String,
    description: Option<String>,
    state: String,
    open_issues: u32,
    closed_issues: u32,
    due_on: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GitHubLabelResponse {
    name: String,
    color: String,
    description: Option<String>,
}

impl GitHubClient {
    /// Create a new milestone in a repository
    ///
    /// Creates a new milestone in the specified repository with the provided title and optional
    /// metadata. The milestone will be assigned a unique ID and created with the current timestamp.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `title` - The title of the milestone (required)
    /// * `description` - Optional description content for the milestone
    /// * `due_on` - Optional due date for the milestone
    /// * `state` - Optional state for the milestone (defaults to Open)
    ///
    /// # Returns
    /// The created `Milestone` with all metadata populated including the assigned milestone ID
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The user does not have permission to create milestones
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn create_milestone(
        &self,
        repository_id: &RepositoryId,
        title: &str,
        description: Option<&str>,
        due_on: Option<chrono::DateTime<chrono::Utc>>,
        state: Option<MilestoneState>,
    ) -> Result<Milestone> {
        let operation_name = "create_milestone";

        retry_with_backoff(operation_name, None, || async {
            self.create_milestone_impl(repository_id, title, description, due_on, state)
                .await
        })
        .await
    }

    async fn create_milestone_impl(
        &self,
        repository_id: &RepositoryId,
        title: &str,
        description: Option<&str>,
        due_on: Option<chrono::DateTime<chrono::Utc>>,
        state: Option<MilestoneState>,
    ) -> std::result::Result<Milestone, ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();

        tracing::debug!("Creating milestone for repository: {}/{}", owner, repo);
        tracing::debug!("Title: {}", title);
        tracing::debug!("Description: {:?}", description);

        // Create milestone using direct GitHub API call instead of octacrab
        // REV: octacrab 0.44.1 has URI parsing bugs with relative paths for milestone operations
        // causing "invalid format" errors. Direct API calls with full URLs avoid this issue.
        let mut request_body = serde_json::json!({
            "title": title,
        });

        if let Some(description) = description {
            request_body["description"] = serde_json::Value::String(description.to_string());
        }

        if let Some(due_on) = due_on {
            request_body["due_on"] = serde_json::Value::String(due_on.to_rfc3339());
        }

        let state_str = match state.unwrap_or(MilestoneState::Open) {
            MilestoneState::Open => "open",
            MilestoneState::Closed => "closed",
        };
        request_body["state"] = serde_json::Value::String(state_str.to_string());

        let url = format!("https://api.github.com/repos/{}/{}/milestones", owner, repo);
        tracing::debug!("Using URL: {}", url);
        tracing::debug!("Request body: {}", request_body);

        let token = self.token.as_ref().ok_or_else(|| {
            ApiRetryableError::NonRetryable("GitHub token not configured".to_string())
        })?;

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "github-edit-cli")
            .header("Accept", "application/vnd.github.v3+json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ApiRetryableError::Retryable(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            let error_msg = format!("GitHub API error {}: {}", status, error_text);
            return Err(if status.is_server_error() {
                ApiRetryableError::Retryable(error_msg)
            } else if status == 429 {
                ApiRetryableError::RateLimit
            } else {
                ApiRetryableError::NonRetryable(error_msg)
            });
        }

        let github_milestone: GitHubMilestoneResponse = response.json().await.map_err(|e| {
            ApiRetryableError::NonRetryable(format!("Failed to parse response: {}", e))
        })?;

        // Convert GitHub API response to our milestone type
        let milestone_state = match github_milestone.state.as_str() {
            "open" => MilestoneState::Open,
            "closed" => MilestoneState::Closed,
            _ => MilestoneState::Open, // Default to Open for any unknown states
        };

        let milestone = Milestone::new(
            MilestoneNumber::new(github_milestone.number as u64),
            github_milestone.title,
            github_milestone.description,
            milestone_state,
            github_milestone.open_issues,
            github_milestone.closed_issues,
            github_milestone.due_on,
            github_milestone.created_at,
            github_milestone
                .updated_at
                .unwrap_or(github_milestone.created_at),
            None, // New milestone is not closed
        );

        Ok(milestone)
    }

    /// Delete a milestone from a repository
    ///
    /// Deletes an existing milestone from the specified repository. This operation is permanent
    /// and cannot be undone. All issues associated with the milestone will be unassigned.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `milestone_number` - The ID of the milestone to delete
    ///
    /// # Returns
    /// Returns `Ok(())` if the milestone was successfully deleted
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The milestone does not exist
    /// - The user does not have permission to delete milestones
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn delete_milestone(
        &self,
        repository_id: &RepositoryId,
        milestone_number: &MilestoneNumber,
    ) -> Result<()> {
        let operation_name = "delete_milestone";

        retry_with_backoff(operation_name, None, || async {
            self.delete_milestone_impl(repository_id, milestone_number)
                .await
        })
        .await
    }

    async fn delete_milestone_impl(
        &self,
        repository_id: &RepositoryId,
        milestone_number: &MilestoneNumber,
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();

        // Use direct GitHub API call instead of octacrab to avoid URI parsing bugs
        // REV: octacrab 0.44.1 fails with relative paths, full URLs work reliably
        let url = format!(
            "https://api.github.com/repos/{}/{}/milestones/{}",
            owner,
            repo,
            milestone_number.value()
        );

        let token = self.token.as_ref().ok_or_else(|| {
            ApiRetryableError::NonRetryable("GitHub token not configured".to_string())
        })?;

        let client = reqwest::Client::new();
        let response = client
            .delete(&url)
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "github-edit-cli")
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .map_err(|e| ApiRetryableError::Retryable(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            let error_msg = format!("GitHub API error {}: {}", status, error_text);
            return Err(if status.is_server_error() {
                ApiRetryableError::Retryable(error_msg)
            } else if status == 429 {
                ApiRetryableError::RateLimit
            } else {
                ApiRetryableError::NonRetryable(error_msg)
            });
        }

        Ok(())
    }

    /// Update a milestone in a repository
    ///
    /// Updates an existing milestone with new metadata. Only the provided fields will be updated;
    /// fields that are `None` will remain unchanged.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `milestone_number` - The ID of the milestone to update
    /// * `title` - Optional new title for the milestone
    /// * `description` - Optional new description for the milestone
    /// * `due_on` - Optional new due date for the milestone
    /// * `state` - Optional new state for the milestone
    ///
    /// # Returns
    /// The updated `Milestone` with all metadata populated
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The milestone does not exist
    /// - The user does not have permission to update milestones
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn update_milestone(
        &self,
        repository_id: &RepositoryId,
        milestone_number: &MilestoneNumber,
        title: Option<&str>,
        description: Option<&str>,
        due_on: Option<chrono::DateTime<chrono::Utc>>,
        state: Option<MilestoneState>,
    ) -> Result<Milestone> {
        let operation_name = "update_milestone";

        retry_with_backoff(operation_name, None, || async {
            self.update_milestone_impl(
                repository_id,
                milestone_number,
                title,
                description,
                due_on,
                state,
            )
            .await
        })
        .await
    }

    async fn update_milestone_impl(
        &self,
        repository_id: &RepositoryId,
        milestone_number: &MilestoneNumber,
        title: Option<&str>,
        description: Option<&str>,
        due_on: Option<chrono::DateTime<chrono::Utc>>,
        state: Option<MilestoneState>,
    ) -> std::result::Result<Milestone, ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();

        // Create request body with only the fields that need to be updated
        let mut request_body = serde_json::json!({});

        if let Some(title) = title {
            request_body["title"] = serde_json::Value::String(title.to_string());
        }

        if let Some(description) = description {
            request_body["description"] = serde_json::Value::String(description.to_string());
        }

        if let Some(due_on) = due_on {
            request_body["due_on"] = serde_json::Value::String(due_on.to_rfc3339());
        }

        if let Some(state) = state {
            let state_str = match state {
                MilestoneState::Open => "open",
                MilestoneState::Closed => "closed",
            };
            request_body["state"] = serde_json::Value::String(state_str.to_string());
        }

        // Use direct GitHub API call instead of octacrab to avoid URI parsing bugs
        // REV: octacrab 0.44.1 fails with relative paths, full URLs work reliably
        let url = format!(
            "https://api.github.com/repos/{}/{}/milestones/{}",
            owner,
            repo,
            milestone_number.value()
        );
        tracing::debug!("Update milestone URL: {}", url);
        tracing::debug!("Update milestone ID: {}", milestone_number.value());
        tracing::debug!("Request body: {}", request_body);

        let token = self.token.as_ref().ok_or_else(|| {
            ApiRetryableError::NonRetryable("GitHub token not configured".to_string())
        })?;

        let client = reqwest::Client::new();
        let response = client
            .patch(&url)
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "github-edit-cli")
            .header("Accept", "application/vnd.github.v3+json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ApiRetryableError::Retryable(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            let error_msg = format!("GitHub API error {}: {}", status, error_text);
            return Err(if status.is_server_error() {
                ApiRetryableError::Retryable(error_msg)
            } else if status == 429 {
                ApiRetryableError::RateLimit
            } else {
                ApiRetryableError::NonRetryable(error_msg)
            });
        }

        let github_milestone: GitHubMilestoneResponse = response.json().await.map_err(|e| {
            ApiRetryableError::NonRetryable(format!("Failed to parse response: {}", e))
        })?;

        // Convert GitHub API response to our milestone type
        let milestone_state = match github_milestone.state.as_str() {
            "open" => MilestoneState::Open,
            "closed" => MilestoneState::Closed,
            _ => MilestoneState::Open, // Default to Open for any unknown states
        };

        let milestone = Milestone::new(
            MilestoneNumber::new(github_milestone.number as u64),
            github_milestone.title,
            github_milestone.description,
            milestone_state,
            github_milestone.open_issues,
            github_milestone.closed_issues,
            github_milestone.due_on,
            github_milestone.created_at,
            github_milestone
                .updated_at
                .unwrap_or(github_milestone.created_at),
            if milestone_state == MilestoneState::Closed {
                Some(
                    github_milestone
                        .updated_at
                        .unwrap_or(github_milestone.created_at),
                )
            } else {
                None
            },
        );

        Ok(milestone)
    }

    /// Create a new label in a repository
    ///
    /// Creates a new label in the specified repository with the provided name, optional color, and optional
    /// description. The label will be created with the current timestamp.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `name` - The name of the label (required)
    /// * `color` - Optional color of the label as a hex string without '#' (defaults to "ffffff")
    /// * `description` - Optional description for the label
    ///
    /// # Returns
    /// The created `Label` with all metadata populated
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The user does not have permission to create labels
    /// - A label with the same name already exists
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn create_label(
        &self,
        repository_id: &RepositoryId,
        name: &str,
        color: Option<&str>,
        description: Option<&str>,
    ) -> Result<Label> {
        let operation_name = "create_label";

        retry_with_backoff(operation_name, None, || async {
            self.create_label_impl(repository_id, name, color, description)
                .await
        })
        .await
    }

    async fn create_label_impl(
        &self,
        repository_id: &RepositoryId,
        name: &str,
        color: Option<&str>,
        description: Option<&str>,
    ) -> std::result::Result<Label, ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();

        tracing::debug!("Creating label for repository: {}/{}", owner, repo);
        tracing::debug!("Label name: {}", name);
        tracing::debug!("Label color: {:?}", color);
        tracing::debug!("Label description: {:?}", description);

        let color = color.unwrap_or("ffffff");

        // Use direct GitHub API call for label operations
        // REV: octocrab doesn't provide repository label operations through issues().labels()
        // Repository labels are managed through the repos API, not issues API
        let url = format!("https://api.github.com/repos/{}/{}/labels", owner, repo);

        let mut request_body = serde_json::json!({
            "name": name,
            "color": color,
        });

        if let Some(description) = description {
            request_body["description"] = serde_json::Value::String(description.to_string());
        }

        let token = self.token.as_ref().ok_or_else(|| {
            ApiRetryableError::NonRetryable("GitHub token not configured".to_string())
        })?;

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "github-edit-cli")
            .header("Accept", "application/vnd.github.v3+json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ApiRetryableError::Retryable(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            let error_msg = format!("GitHub API error {}: {}", status, error_text);
            return Err(if status.is_server_error() {
                ApiRetryableError::Retryable(error_msg)
            } else if status == 429 {
                ApiRetryableError::RateLimit
            } else {
                ApiRetryableError::NonRetryable(error_msg)
            });
        }

        let result = response.json::<GitHubLabelResponse>().await;

        match result {
            Ok(github_label) => {
                let label = Label::new_with_description(
                    github_label.name,
                    Some(github_label.color),
                    github_label.description,
                );
                Ok(label)
            }
            Err(e) => {
                let error_msg = format!("Failed to parse label response: {}", e);
                Err(ApiRetryableError::NonRetryable(error_msg))
            }
        }
    }

    /// Update a label in a repository
    ///
    /// Updates an existing label with new metadata. Only the provided fields will be updated;
    /// fields that are `None` will remain unchanged.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `old_name` - The current name of the label to update
    /// * `new_name` - Optional new name for the label
    /// * `color` - Optional new color for the label
    /// * `description` - Optional new description for the label
    ///
    /// # Returns
    /// The updated `Label` with all metadata populated
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The label does not exist
    /// - The user does not have permission to update labels
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn update_label(
        &self,
        repository_id: &RepositoryId,
        old_name: &str,
        new_name: Option<&str>,
        color: Option<&str>,
        description: Option<&str>,
    ) -> Result<Label> {
        let operation_name = "update_label";

        retry_with_backoff(operation_name, None, || async {
            self.update_label_impl(repository_id, old_name, new_name, color, description)
                .await
        })
        .await
    }

    async fn update_label_impl(
        &self,
        repository_id: &RepositoryId,
        old_name: &str,
        new_name: Option<&str>,
        color: Option<&str>,
        description: Option<&str>,
    ) -> std::result::Result<Label, ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();

        // Use direct GitHub API call for label operations
        // REV: octocrab doesn't provide repository label operations through issues().labels()
        // Repository labels are managed through the repos API, not issues API
        let url = format!(
            "https://api.github.com/repos/{}/{}/labels/{}",
            owner, repo, old_name
        );

        let mut request_body = serde_json::json!({});

        if let Some(new_name) = new_name {
            request_body["name"] = serde_json::Value::String(new_name.to_string());
        }

        if let Some(color) = color {
            request_body["color"] = serde_json::Value::String(color.to_string());
        }

        if let Some(description) = description {
            request_body["description"] = serde_json::Value::String(description.to_string());
        }

        let token = self.token.as_ref().ok_or_else(|| {
            ApiRetryableError::NonRetryable("GitHub token not configured".to_string())
        })?;

        let client = reqwest::Client::new();
        let response = client
            .patch(&url)
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "github-edit-cli")
            .header("Accept", "application/vnd.github.v3+json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ApiRetryableError::Retryable(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            let error_msg = format!("GitHub API error {}: {}", status, error_text);
            return Err(if status.is_server_error() {
                ApiRetryableError::Retryable(error_msg)
            } else if status == 429 {
                ApiRetryableError::RateLimit
            } else {
                ApiRetryableError::NonRetryable(error_msg)
            });
        }

        let result = response.json::<GitHubLabelResponse>().await;

        match result {
            Ok(github_label) => {
                let label = Label::new_with_description(
                    github_label.name,
                    Some(github_label.color),
                    github_label.description,
                );
                Ok(label)
            }
            Err(e) => {
                let error_msg = format!("Failed to parse label response: {}", e);
                Err(ApiRetryableError::NonRetryable(error_msg))
            }
        }
    }

    /// Delete a label from a repository
    ///
    /// Deletes an existing label from the specified repository. This operation is permanent
    /// and cannot be undone. All issues and pull requests with this label will be unassigned.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `label_name` - The name of the label to delete
    ///
    /// # Returns
    /// Returns `Ok(())` if the label was successfully deleted
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The label does not exist
    /// - The user does not have permission to delete labels
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn delete_label(&self, repository_id: &RepositoryId, label_name: &str) -> Result<()> {
        let operation_name = "delete_label";

        retry_with_backoff(operation_name, None, || async {
            self.delete_label_impl(repository_id, label_name).await
        })
        .await
    }

    async fn delete_label_impl(
        &self,
        repository_id: &RepositoryId,
        label_name: &str,
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();

        // Use direct GitHub API call for label operations
        // REV: octocrab doesn't provide repository label operations through issues().labels()
        // Repository labels are managed through the repos API, not issues API
        let url = format!(
            "https://api.github.com/repos/{}/{}/labels/{}",
            owner, repo, label_name
        );

        let token = self.token.as_ref().ok_or_else(|| {
            ApiRetryableError::NonRetryable("GitHub token not configured".to_string())
        })?;

        let client = reqwest::Client::new();
        let response = client
            .delete(&url)
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "github-edit-cli")
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .map_err(|e| ApiRetryableError::Retryable(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            let error_msg = format!("GitHub API error {}: {}", status, error_text);
            return Err(if status.is_server_error() {
                ApiRetryableError::Retryable(error_msg)
            } else if status == 429 {
                ApiRetryableError::RateLimit
            } else {
                ApiRetryableError::NonRetryable(error_msg)
            });
        }

        Ok(())
    }
}
