use crate::github::GitHubClient;
use crate::types::label::Label;
use crate::types::milestone::{Milestone, MilestoneState};
use crate::types::repository::{MilestoneNumber, RepositoryId};
use anyhow::Result;

/// Service layer for repository operations
///
/// This service provides a high-level interface for managing GitHub repository resources,
/// including labels and milestones, encapsulating the underlying GitHub client operations
/// with additional business logic and error handling.
pub struct RepositoryService {
    github_client: GitHubClient,
}

impl RepositoryService {
    /// Create a new repository service instance
    pub fn new(github_client: GitHubClient) -> Self {
        Self { github_client }
    }

    /// Create a new label
    ///
    /// Creates a new label in the specified repository with the provided
    /// name, optional color, and optional description.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier
    /// * `name` - The label name
    /// * `color` - Optional label color (defaults to "ffffff")
    /// * `description` - Optional label description
    ///
    /// # Returns
    /// The created label with all metadata
    pub async fn create_label(
        &self,
        repository_id: &RepositoryId,
        name: &str,
        color: Option<&str>,
        description: Option<&str>,
    ) -> Result<Label> {
        self.github_client
            .create_label(repository_id, name, color, description)
            .await
    }

    /// Update a label
    ///
    /// Updates an existing label with new metadata. Only the provided
    /// fields will be updated; fields that are `None` will remain unchanged.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier
    /// * `old_name` - The current name of the label to update
    /// * `new_name` - Optional new name for the label
    /// * `color` - Optional new color for the label
    /// * `description` - Optional new description for the label
    ///
    /// # Returns
    /// The updated label with all metadata
    pub async fn update_label(
        &self,
        repository_id: &RepositoryId,
        old_name: &str,
        new_name: Option<&str>,
        color: Option<&str>,
        description: Option<&str>,
    ) -> Result<Label> {
        self.github_client
            .update_label(repository_id, old_name, new_name, color, description)
            .await
    }

    /// Delete a label
    ///
    /// Deletes an existing label from the specified repository. This operation
    /// is permanent and cannot be undone. All issues and pull requests with
    /// this label will be unassigned.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier
    /// * `label_name` - The name of the label to delete
    pub async fn delete_label(&self, repository_id: &RepositoryId, label_name: &str) -> Result<()> {
        self.github_client
            .delete_label(repository_id, label_name)
            .await
    }

    /// Create a new milestone
    ///
    /// Creates a new milestone in the specified repository with the provided
    /// title and optional metadata.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier
    /// * `title` - The milestone title
    /// * `description` - Optional milestone description
    /// * `due_on` - Optional due date for the milestone
    /// * `state` - Optional state for the milestone (defaults to Open)
    ///
    /// # Returns
    /// The created milestone with all metadata
    pub async fn create_milestone(
        &self,
        repository_id: &RepositoryId,
        title: &str,
        description: Option<&str>,
        due_on: Option<chrono::DateTime<chrono::Utc>>,
        state: Option<MilestoneState>,
    ) -> Result<Milestone> {
        self.github_client
            .create_milestone(repository_id, title, description, due_on, state)
            .await
    }

    /// Update a milestone
    ///
    /// Updates an existing milestone with new metadata. Only the provided
    /// fields will be updated; fields that are `None` will remain unchanged.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier
    /// * `milestone_number` - The ID of the milestone to update
    /// * `title` - Optional new title for the milestone
    /// * `description` - Optional new description for the milestone
    /// * `due_on` - Optional new due date for the milestone
    /// * `state` - Optional new state for the milestone
    ///
    /// # Returns
    /// The updated milestone with all metadata
    pub async fn update_milestone(
        &self,
        repository_id: &RepositoryId,
        milestone_number: &MilestoneNumber,
        title: Option<&str>,
        description: Option<&str>,
        due_on: Option<chrono::DateTime<chrono::Utc>>,
        state: Option<MilestoneState>,
    ) -> Result<Milestone> {
        self.github_client
            .update_milestone(
                repository_id,
                milestone_number,
                title,
                description,
                due_on,
                state,
            )
            .await
    }

    /// Delete a milestone
    ///
    /// Deletes an existing milestone from the specified repository. This operation
    /// is permanent and cannot be undone. All issues associated with the milestone
    /// will be unassigned.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier
    /// * `milestone_number` - The ID of the milestone to delete
    pub async fn delete_milestone(
        &self,
        repository_id: &RepositoryId,
        milestone_number: &MilestoneNumber,
    ) -> Result<()> {
        self.github_client
            .delete_milestone(repository_id, milestone_number)
            .await
    }
}
