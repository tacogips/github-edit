use anyhow::Result;
use std::collections::BTreeMap;

use crate::github::GitHubClient;
use crate::services::issue_service::IssueService;
use crate::types::issue::{Issue, IssueCommentNumber, IssueId, IssueNumber, IssueState, IssueUrl};
use crate::types::repository::{MilestoneNumber, RepositoryId};
use crate::types::{User, label::Label};

/// Get details for multiple issues from their URLs
///
/// This function parses issue URLs, groups them by repository,
/// and fetches the corresponding issues using the IssueService.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `issue_urls` - Vector of issue URLs to fetch
///
/// # Returns
/// A BTreeMap grouping issues by repository ID
pub async fn get_issues_details(
    github_client: &GitHubClient,
    issue_urls: Vec<IssueUrl>,
) -> Result<BTreeMap<RepositoryId, Vec<Issue>>> {
    // Convert URLs to IssueIds and group by repository
    let mut issue_ids_by_repo: BTreeMap<RepositoryId, Vec<IssueNumber>> = BTreeMap::new();

    for url in issue_urls {
        match IssueId::parse_url(&url) {
            Ok(issue_id) => {
                let issue_number = IssueNumber::new(issue_id.number);
                issue_ids_by_repo
                    .entry(issue_id.git_repository)
                    .or_default()
                    .push(issue_number);
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to parse issue URL {}: {}", url, e));
            }
        }
    }

    // Fetch issues directly using GitHub client
    let mut result: BTreeMap<RepositoryId, Vec<Issue>> = BTreeMap::new();

    for (repository_id, issue_numbers) in issue_ids_by_repo {
        let mut issues = Vec::new();

        for issue_number in issue_numbers {
            match github_client.get_issue(&repository_id, issue_number).await {
                Ok(issue) => issues.push(issue),
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Failed to fetch issue {} from repository {}: {}",
                        issue_number,
                        repository_id,
                        e
                    ));
                }
            }
        }

        result.insert(repository_id, issues);
    }

    Ok(result)
}

/// Create a new issue
///
/// Creates a new issue in the specified repository with the provided
/// title and optional metadata.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `title` - The issue title
/// * `body` - Optional issue body content
/// * `assignees` - Optional list of users to assign
/// * `labels` - Optional list of labels to apply
/// * `milestone_number` - Optional milestone to associate
///
/// # Returns
/// The created issue with assigned number and metadata
pub async fn create_issue(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    title: &str,
    body: Option<&str>,
    assignees: Option<&[User]>,
    labels: Option<&[Label]>,
    milestone_number: Option<MilestoneNumber>,
) -> Result<Issue> {
    let issue_service = IssueService::new(github_client.clone());
    issue_service
        .create_issue(
            repository_id,
            title,
            body,
            assignees,
            labels,
            milestone_number,
        )
        .await
}

/// Add a comment to an issue
///
/// Creates a new comment on the specified issue.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `issue_number` - The issue number to comment on
/// * `body` - The comment content
///
/// # Returns
/// The comment number of the created comment
pub async fn add_comment(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    issue_number: IssueNumber,
    body: &str,
) -> Result<IssueCommentNumber> {
    let issue_service = IssueService::new(github_client.clone());
    issue_service
        .add_comment(repository_id, issue_number, body)
        .await
}

/// Edit an existing issue comment
///
/// Updates the content of an existing comment.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `issue_number` - The issue number containing the comment
/// * `comment_number` - The comment number to edit
/// * `body` - The new comment content
pub async fn edit_comment(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    issue_number: IssueNumber,
    comment_number: IssueCommentNumber,
    body: &str,
) -> Result<()> {
    let issue_service = IssueService::new(github_client.clone());
    issue_service
        .edit_comment(repository_id, issue_number, comment_number, body)
        .await
}

/// Delete an issue comment
///
/// Permanently removes a comment from an issue.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `issue_number` - The issue number containing the comment
/// * `comment_number` - The comment number to delete
pub async fn delete_comment(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    issue_number: IssueNumber,
    comment_number: IssueCommentNumber,
) -> Result<()> {
    let issue_service = IssueService::new(github_client.clone());
    issue_service
        .delete_comment(repository_id, issue_number, comment_number)
        .await
}

/// Edit the title of an issue
///
/// Updates only the title of an existing issue.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `issue_number` - The issue number to update
/// * `title` - The new title
pub async fn edit_title(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    issue_number: IssueNumber,
    title: &str,
) -> Result<()> {
    let issue_service = IssueService::new(github_client.clone());
    issue_service
        .edit_title(repository_id, issue_number, title)
        .await
}

/// Edit the body of an issue
///
/// Updates only the body content of an existing issue.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `issue_number` - The issue number to update
/// * `body` - The new body content
pub async fn edit_body(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    issue_number: IssueNumber,
    body: &str,
) -> Result<()> {
    let issue_service = IssueService::new(github_client.clone());
    issue_service
        .edit_body(repository_id, issue_number, body)
        .await
}

/// Update the state of an issue
///
/// Changes an issue's state between open and closed.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `issue_number` - The issue number to update
/// * `state` - The new state for the issue
pub async fn update_state(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    issue_number: IssueNumber,
    state: IssueState,
) -> Result<()> {
    let issue_service = IssueService::new(github_client.clone());
    issue_service
        .update_state(repository_id, issue_number, state)
        .await
}

/// Delete an issue
///
/// Permanently removes an issue from the repository.
/// This operation requires admin permissions.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `issue_number` - The issue number to delete
pub async fn delete_issue(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    issue_number: IssueNumber,
) -> Result<()> {
    let issue_service = IssueService::new(github_client.clone());
    issue_service
        .delete_issue(repository_id, issue_number)
        .await
}

/// Add assignees to an issue
///
/// Adds one or more assignees to an existing issue. Before adding,
/// retrieves the current assignees and only adds those that are not
/// already assigned to avoid duplicates.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `issue_number` - The issue number to update
/// * `new_assignees` - List of usernames to add as assignees
///
/// # Returns
/// A tuple containing:
/// - Vector of usernames that were successfully added
/// - Vector of usernames that were skipped (already assigned)
pub async fn add_assignees(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    issue_number: IssueNumber,
    new_assignees: &[String],
) -> Result<(Vec<String>, Vec<String>)> {
    let issue_service = IssueService::new(github_client.clone());
    issue_service
        .add_assignees(repository_id, issue_number, new_assignees)
        .await
}

/// Remove assignees from an issue
///
/// Removes one or more assignees from an existing issue. Before removing,
/// retrieves the current assignees and only removes those that are currently
/// assigned to avoid errors.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `issue_number` - The issue number to update
/// * `assignees_to_remove` - List of usernames to remove as assignees
///
/// # Returns
/// A tuple containing:
/// - Vector of usernames that were successfully removed
/// - Vector of usernames that were skipped (not assigned)
pub async fn remove_assignees(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    issue_number: IssueNumber,
    assignees_to_remove: &[String],
) -> Result<(Vec<String>, Vec<String>)> {
    // Get current issue to check existing assignees
    let current_issue = github_client.get_issue(repository_id, issue_number).await?;
    let current_assignees = &current_issue.assignees;

    // Filter out assignees that are not currently assigned
    let mut removed_assignees = Vec::new();
    let mut skipped_assignees = Vec::new();

    for assignee in assignees_to_remove {
        if current_assignees.contains(assignee) {
            removed_assignees.push(assignee.clone());
        } else {
            skipped_assignees.push(assignee.clone());
        }
    }

    // If there are assignees to remove, update the issue
    if !removed_assignees.is_empty() {
        let updated_assignees: Vec<String> = current_assignees
            .iter()
            .filter(|a| !removed_assignees.contains(a))
            .cloned()
            .collect();

        let issue_service = IssueService::new(github_client.clone());
        issue_service
            .edit_assignees(repository_id, issue_number, &updated_assignees)
            .await?;
    }

    Ok((removed_assignees, skipped_assignees))
}

/// Remove labels from an issue
///
/// Removes one or more labels from an existing issue. Before removing,
/// retrieves the current labels and only removes those that are currently
/// assigned to avoid errors.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `issue_number` - The issue number to update
/// * `labels_to_remove` - List of labels to remove from the issue
///
/// # Returns
/// A tuple containing:
/// - Vector of labels that were successfully removed
/// - Vector of labels that were skipped (not assigned)
pub async fn remove_labels(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    issue_number: IssueNumber,
    labels_to_remove: &[Label],
) -> Result<(Vec<Label>, Vec<Label>)> {
    // Get current issue to check existing labels
    let current_issue = github_client.get_issue(repository_id, issue_number).await?;
    let current_labels = &current_issue.labels;

    // Filter out labels that are not currently assigned
    let mut removed_labels = Vec::new();
    let mut skipped_labels = Vec::new();

    for label in labels_to_remove {
        if current_labels.contains(&label.name) {
            removed_labels.push(label.clone());
        } else {
            skipped_labels.push(label.clone());
        }
    }

    // If there are labels to remove, update the issue
    if !removed_labels.is_empty() {
        let updated_labels: Vec<Label> = current_labels
            .iter()
            .filter(|l| !removed_labels.iter().any(|rl| rl.name == **l))
            .map(|l| Label::from(l.clone()))
            .collect();

        let issue_service = IssueService::new(github_client.clone());
        issue_service
            .update_issue(
                repository_id,
                issue_number,
                None,
                None,
                None,
                None,
                Some(&updated_labels),
                None,
            )
            .await?;
    }

    Ok((removed_labels, skipped_labels))
}

/// Set milestone for an issue
///
/// Sets or updates the milestone for an existing issue. This operation
/// replaces any existing milestone with the specified one.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `issue_number` - The issue number to set milestone for
/// * `milestone_number` - The milestone ID to assign to the issue
pub async fn set_milestone(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    issue_number: IssueNumber,
    milestone_number: MilestoneNumber,
) -> Result<()> {
    let issue_service = IssueService::new(github_client.clone());
    issue_service
        .set_milestone(repository_id, issue_number, milestone_number)
        .await
}

/// Add labels to an issue
///
/// Adds one or more labels to an existing issue. Before adding,
/// retrieves the current labels and only adds those that are not
/// already assigned to avoid duplicates.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `issue_number` - The issue number to update
/// * `new_labels` - List of labels to add to the issue
///
/// # Returns
/// A tuple containing:
/// - Vector of labels that were successfully added
/// - Vector of labels that were skipped (already assigned)
pub async fn add_labels(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    issue_number: IssueNumber,
    new_labels: &[Label],
) -> Result<(Vec<Label>, Vec<Label>)> {
    // Get current issue to check existing labels
    let current_issue = github_client.get_issue(repository_id, issue_number).await?;
    let current_labels = &current_issue.labels;

    // Filter out labels that are already assigned
    let mut added_labels = Vec::new();
    let mut skipped_labels = Vec::new();

    for label in new_labels {
        if current_labels.contains(&label.name) {
            skipped_labels.push(label.clone());
        } else {
            added_labels.push(label.clone());
        }
    }

    // If there are new labels to add, update the issue
    if !added_labels.is_empty() {
        let issue_service = IssueService::new(github_client.clone());
        issue_service
            .add_labels(repository_id, issue_number, &added_labels)
            .await?;
    }

    Ok((added_labels, skipped_labels))
}

/// Remove milestone from an issue
///
/// Removes the milestone from an existing issue, if one is set.
/// This operation clears the milestone field for the issue.
///
/// # Arguments
/// * `github_client` - The GitHub client instance
/// * `repository_id` - The repository identifier
/// * `issue_number` - The issue number to remove milestone from
pub async fn remove_milestone(
    github_client: &GitHubClient,
    repository_id: &RepositoryId,
    issue_number: IssueNumber,
) -> Result<()> {
    let issue_service = IssueService::new(github_client.clone());
    issue_service
        .remove_milestone(repository_id, issue_number)
        .await
}
