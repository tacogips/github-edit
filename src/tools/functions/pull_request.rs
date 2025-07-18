use crate::github::GitHubClient;
use crate::services::pull_request_service::PullRequestService;
use crate::types::label::Label;
use crate::types::pull_request::{
    Branch, PullRequest, PullRequestCommentNumber, PullRequestNumber,
};
use crate::types::repository::{MilestoneNumber, RepositoryId};
use anyhow::Result;

/// Create a new pull request
///
/// Creates a new pull request in the specified repository from the head branch
/// to the base branch with the provided title and optional metadata.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `title` - The pull request title
/// * `head_branch` - The branch containing the changes to be merged
/// * `base_branch` - The target branch to merge changes into
/// * `body` - Optional pull request body content
/// * `draft` - Whether to create the pull request as a draft
///
/// # Returns
/// The created pull request with assigned number and metadata
pub async fn create_pull_request(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    title: &str,
    head_branch: &Branch,
    base_branch: &Branch,
    body: Option<&str>,
    draft: Option<bool>,
) -> Result<PullRequest> {
    let pr_service = PullRequestService::new(github_client.clone());
    pr_service
        .create_pull_request(repository_id, title, head_branch, base_branch, body, draft)
        .await
}

/// Add a comment to a pull request
///
/// Creates a new comment on the specified pull request.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `pr_number` - The pull request number to comment on
/// * `body` - The comment content
///
/// # Returns
/// The comment number of the created comment
pub async fn add_comment(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    pr_number: PullRequestNumber,
    body: &str,
) -> Result<PullRequestCommentNumber> {
    let pr_service = PullRequestService::new(github_client.clone());
    pr_service.add_comment(repository_id, pr_number, body).await
}

/// Edit an existing pull request comment
///
/// Updates the content of an existing comment.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `pr_number` - The pull request number containing the comment
/// * `comment_number` - The comment number to edit
/// * `body` - The new comment content
pub async fn edit_comment(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    pr_number: PullRequestNumber,
    comment_number: PullRequestCommentNumber,
    body: &str,
) -> Result<()> {
    let pr_service = PullRequestService::new(github_client.clone());
    pr_service
        .edit_comment(repository_id, pr_number, comment_number, body)
        .await
}

/// Delete a pull request comment
///
/// Permanently removes a comment from a pull request.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `pr_number` - The pull request number containing the comment
/// * `comment_number` - The comment number to delete
pub async fn delete_comment(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    pr_number: PullRequestNumber,
    comment_number: PullRequestCommentNumber,
) -> Result<()> {
    let pr_service = PullRequestService::new(github_client.clone());
    pr_service
        .delete_comment(repository_id, pr_number, comment_number)
        .await
}

/// Close a pull request
///
/// Closes an existing pull request in the specified repository.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `pr_number` - The pull request number to close
pub async fn close_pull_request(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    pr_number: PullRequestNumber,
) -> Result<()> {
    let pr_service = PullRequestService::new(github_client.clone());
    pr_service
        .close_pull_request(repository_id, pr_number)
        .await
}

/// Edit the title of a pull request
///
/// Updates only the title of an existing pull request.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `pr_number` - The pull request number to update
/// * `title` - The new title content
pub async fn edit_title(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    pr_number: PullRequestNumber,
    title: &str,
) -> Result<()> {
    let pr_service = PullRequestService::new(github_client.clone());
    pr_service.edit_title(repository_id, pr_number, title).await
}

/// Edit the body of a pull request
///
/// Updates only the body content of an existing pull request.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `pr_number` - The pull request number to update
/// * `body` - The new body content
pub async fn edit_body(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    pr_number: PullRequestNumber,
    body: &str,
) -> Result<()> {
    let pr_service = PullRequestService::new(github_client.clone());
    pr_service.edit_body(repository_id, pr_number, body).await
}

/// Add assignees to a pull request
///
/// Adds one or more assignees to an existing pull request. Before adding,
/// retrieves the current assignees and only adds those that are not
/// already assigned to avoid duplicates.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `pr_number` - The pull request number to update
/// * `new_assignees` - List of usernames to add as assignees
///
/// # Returns
/// A tuple containing:
/// - Vector of usernames that were successfully added
/// - Vector of usernames that were skipped (already assigned)
pub async fn add_assignees(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    pr_number: PullRequestNumber,
    new_assignees: &[String],
) -> Result<(Vec<String>, Vec<String>)> {
    let pr_service = PullRequestService::new(github_client.clone());
    pr_service
        .add_assignees(repository_id, pr_number, new_assignees)
        .await
}

/// Remove assignees from a pull request
///
/// Removes one or more users from the assignee list of the specified pull request.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `pr_number` - The pull request number to remove assignees from
/// * `assignees` - List of usernames to remove from the pull request
pub async fn remove_assignees(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    pr_number: PullRequestNumber,
    assignees: &[String],
) -> Result<()> {
    let pr_service = PullRequestService::new(github_client.clone());
    pr_service
        .remove_assignees(repository_id, pr_number, assignees)
        .await
}

/// Add requested reviewers to a pull request
///
/// Adds one or more users as requested reviewers to an existing pull request.
/// Before adding, retrieves the current requested reviewers and only adds those
/// that are not already requested to avoid duplicates.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `pr_number` - The pull request number to request reviewers for
/// * `new_reviewers` - List of usernames to request as reviewers
///
/// # Returns
/// A tuple containing:
/// - Vector of usernames that were successfully added as requested reviewers
/// - Vector of usernames that were skipped (already requested)
pub async fn add_requested_reviewers(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    pr_number: PullRequestNumber,
    new_reviewers: &[String],
) -> Result<(Vec<String>, Vec<String>)> {
    let pr_service = PullRequestService::new(github_client.clone());
    pr_service
        .add_requested_reviewers(repository_id, pr_number, new_reviewers)
        .await
}

/// Add labels to a pull request
///
/// Adds one or more labels to the specified pull request.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `pr_number` - The pull request number to add labels to
/// * `labels` - List of labels to add to the pull request
pub async fn add_labels(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    pr_number: PullRequestNumber,
    labels: &[Label],
) -> Result<()> {
    let pr_service = PullRequestService::new(github_client.clone());
    pr_service
        .add_labels(repository_id, pr_number, labels)
        .await
}

/// Remove labels from a pull request
///
/// Removes one or more labels from the specified pull request.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `pr_number` - The pull request number to remove labels from
/// * `labels` - List of labels to remove from the pull request
pub async fn remove_labels(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    pr_number: PullRequestNumber,
    labels: &[Label],
) -> Result<()> {
    let pr_service = PullRequestService::new(github_client.clone());
    pr_service
        .remove_labels(repository_id, pr_number, labels)
        .await
}

/// Add milestone to a pull request
///
/// Assigns a milestone to the specified pull request.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `pr_number` - The pull request number to assign milestone to
/// * `milestone_number` - The milestone ID to assign to the pull request
pub async fn add_milestone(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    pr_number: PullRequestNumber,
    milestone_number: MilestoneNumber,
) -> Result<()> {
    let pr_service = PullRequestService::new(github_client.clone());
    pr_service
        .add_milestone(repository_id, pr_number, milestone_number)
        .await
}

/// Remove milestone from a pull request
///
/// Removes the milestone from the specified pull request.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `pr_number` - The pull request number to remove milestone from
pub async fn remove_milestone(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    pr_number: PullRequestNumber,
) -> Result<()> {
    let pr_service = PullRequestService::new(github_client.clone());
    pr_service.remove_milestone(repository_id, pr_number).await
}
