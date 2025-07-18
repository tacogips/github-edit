use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::github::GitHubClient;
use crate::services::repository_service::RepositoryService;
use crate::types::label::Label;
use crate::types::milestone::{Milestone, MilestoneState};
use crate::types::repository::{MilestoneNumber, RepositoryId};

/// Create a new label in a repository
///
/// Creates a new label in the specified repository with the provided
/// name, optional color, and optional description.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `name` - The label name
/// * `color` - Optional label color (defaults to "ffffff")
/// * `description` - Optional label description
///
/// # Returns
/// The created label with all metadata
pub async fn create_label(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    name: &str,
    color: Option<&str>,
    description: Option<&str>,
) -> Result<Label> {
    let repository_service = RepositoryService::new(github_client.clone());
    repository_service
        .create_label(repository_id, name, color, description)
        .await
}

/// Update an existing label in a repository
///
/// Updates an existing label with new metadata. Only the provided
/// fields will be updated; fields that are `None` will remain unchanged.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `old_name` - The current name of the label to update
/// * `new_name` - Optional new name for the label
/// * `color` - Optional new color for the label
/// * `description` - Optional new description for the label
///
/// # Returns
/// The updated label with all metadata
pub async fn update_label(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    old_name: &str,
    new_name: Option<&str>,
    color: Option<&str>,
    description: Option<&str>,
) -> Result<Label> {
    let repository_service = RepositoryService::new(github_client.clone());
    repository_service
        .update_label(repository_id, old_name, new_name, color, description)
        .await
}

/// Delete an existing label from a repository
///
/// Deletes an existing label from the specified repository. This operation
/// is permanent and cannot be undone. All issues and pull requests with
/// this label will be unassigned.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `label_name` - The name of the label to delete
///
/// # Returns
/// Success or error result
pub async fn delete_label(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    label_name: &str,
) -> Result<()> {
    let repository_service = RepositoryService::new(github_client.clone());
    repository_service
        .delete_label(repository_id, label_name)
        .await
}

/// Create a new milestone in a repository
///
/// Creates a new milestone in the specified repository with the provided
/// title and optional metadata.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `title` - The milestone title
/// * `description` - Optional milestone description
/// * `due_on` - Optional due date for the milestone
/// * `state` - Optional state for the milestone (defaults to Open)
///
/// # Returns
/// The created milestone with assigned ID and metadata
pub async fn create_milestone(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    title: &str,
    description: Option<&str>,
    due_on: Option<DateTime<Utc>>,
    state: Option<MilestoneState>,
) -> Result<Milestone> {
    let repository_service = RepositoryService::new(github_client.clone());
    repository_service
        .create_milestone(repository_id, title, description, due_on, state)
        .await
}

/// Update an existing milestone in a repository
///
/// Updates an existing milestone in the specified repository with the provided
/// optional metadata. Only specified fields will be updated.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `milestone_number` - The milestone identifier to update
/// * `title` - Optional new milestone title
/// * `description` - Optional new milestone description
/// * `due_on` - Optional new due date for the milestone
/// * `state` - Optional new state for the milestone
///
/// # Returns
/// The updated milestone with new metadata
pub async fn update_milestone(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    milestone_number: &MilestoneNumber,
    title: Option<&str>,
    description: Option<&str>,
    due_on: Option<DateTime<Utc>>,
    state: Option<MilestoneState>,
) -> Result<Milestone> {
    let repository_service = RepositoryService::new(github_client.clone());
    repository_service
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

/// Delete an existing milestone from a repository
///
/// Deletes the specified milestone from the repository. This action cannot be undone.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `milestone_number` - The milestone identifier to delete
///
/// # Returns
/// Success or error result
pub async fn delete_milestone(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    milestone_number: &MilestoneNumber,
) -> Result<()> {
    let repository_service = RepositoryService::new(github_client.clone());
    repository_service
        .delete_milestone(repository_id, milestone_number)
        .await
}
