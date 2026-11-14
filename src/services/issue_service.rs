use crate::github::GitHubClient;
use crate::types::issue::{Issue, IssueCommentNumber, IssueNumber, IssueState};
use crate::types::repository::{MilestoneNumber, RepositoryId};
use crate::types::{User, label::Label};
use anyhow::Result;

/// Service layer for issue operations
///
/// This service provides a high-level interface for managing GitHub issues,
/// encapsulating the underlying GitHub client operations with additional
/// business logic and error handling.
pub struct IssueService {
    github_client: GitHubClient,
}

impl IssueService {
    /// Create a new issue service instance
    pub fn new(github_client: GitHubClient) -> Self {
        Self { github_client }
    }

    /// Create a new issue
    ///
    /// Creates a new issue in the specified repository with the provided
    /// title and optional metadata.
    ///
    /// # Arguments
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
        &self,
        repository_id: &RepositoryId,
        title: &str,
        body: Option<&str>,
        assignees: Option<&[User]>,
        labels: Option<&[Label]>,
        milestone_number: Option<MilestoneNumber>,
    ) -> Result<Issue> {
        self.github_client
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
    /// * `repository_id` - The repository identifier
    /// * `issue_number` - The issue number to comment on
    /// * `body` - The comment content
    ///
    /// # Returns
    /// The comment number of the created comment
    pub async fn add_comment(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        body: &str,
    ) -> Result<IssueCommentNumber> {
        self.github_client
            .add_issue_comment(repository_id, issue_number, body)
            .await
    }

    /// Edit an existing issue comment
    ///
    /// Updates the content of an existing comment.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier
    /// * `issue_number` - The issue number containing the comment
    /// * `comment_number` - The comment number to edit
    /// * `body` - The new comment content
    pub async fn edit_comment(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        comment_number: IssueCommentNumber,
        body: &str,
    ) -> Result<()> {
        self.github_client
            .edit_issue_comment(repository_id, issue_number, comment_number, body)
            .await
    }

    /// Delete an issue comment
    ///
    /// Permanently removes a comment from an issue.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier
    /// * `issue_number` - The issue number containing the comment
    /// * `comment_number` - The comment number to delete
    pub async fn delete_comment(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        comment_number: IssueCommentNumber,
    ) -> Result<()> {
        self.github_client
            .delete_issue_comment(repository_id, issue_number, comment_number)
            .await
    }

    /// Edit the title of an issue
    ///
    /// Updates only the title of an existing issue.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier
    /// * `issue_number` - The issue number to update
    /// * `title` - The new title
    pub async fn edit_title(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        title: &str,
    ) -> Result<()> {
        self.github_client
            .edit_issue_title(repository_id, issue_number, title)
            .await
    }

    /// Edit the body of an issue
    ///
    /// Updates only the body content of an existing issue.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier
    /// * `issue_number` - The issue number to update
    /// * `body` - The new body content
    pub async fn edit_body(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        body: &str,
    ) -> Result<()> {
        self.github_client
            .edit_issue_body(repository_id, issue_number, body)
            .await
    }

    /// Edit the assignees of an issue
    ///
    /// Updates the assignees list for an existing issue.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier
    /// * `issue_number` - The issue number to update
    /// * `assignees` - The new list of assignee usernames
    pub async fn edit_assignees(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        assignees: &[String],
    ) -> Result<()> {
        self.github_client
            .edit_issue_assignees(repository_id, issue_number, assignees)
            .await
    }

    /// Update the state of an issue
    ///
    /// Changes an issue's state between open and closed.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier
    /// * `issue_number` - The issue number to update
    /// * `state` - The new state for the issue
    pub async fn update_state(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        state: IssueState,
    ) -> Result<()> {
        self.github_client
            .update_issue_state(repository_id, issue_number, state)
            .await
    }

    /// Update multiple aspects of an issue
    ///
    /// Performs a comprehensive update of an issue's metadata including
    /// title, body, state, assignees, labels, and milestone.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier
    /// * `issue_number` - The issue number to update
    /// * `title` - Optional new title
    /// * `body` - Optional new body content
    /// * `state` - Optional new state
    /// * `assignees` - Optional new assignees list
    /// * `labels` - Optional new labels list
    /// * `milestone_number` - Optional new milestone
    ///
    /// # Returns
    /// The updated issue with all current metadata
    pub async fn update_issue(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        title: Option<&str>,
        body: Option<Option<&str>>,
        state: Option<IssueState>,
        assignees: Option<&[User]>,
        labels: Option<&[Label]>,
        milestone_number: Option<Option<MilestoneNumber>>,
    ) -> Result<Issue> {
        self.github_client
            .update_issue(
                repository_id,
                issue_number,
                title,
                body,
                state,
                assignees,
                labels,
                milestone_number,
            )
            .await
    }

    /// Delete an issue
    ///
    /// Permanently removes an issue from the repository.
    /// This operation requires admin permissions.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier
    /// * `issue_number` - The issue number to delete
    pub async fn delete_issue(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
    ) -> Result<()> {
        self.github_client
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
    /// * `repository_id` - The repository identifier
    /// * `issue_number` - The issue number to update
    /// * `new_assignees` - List of usernames to add as assignees
    ///
    /// # Returns
    /// A tuple containing:
    /// - Vector of usernames that were successfully added
    /// - Vector of usernames that were skipped (already assigned)
    pub async fn add_assignees(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        new_assignees: &[String],
    ) -> Result<(Vec<String>, Vec<String>)> {
        // Get current issue to check existing assignees
        let current_issue = self
            .github_client
            .get_issue(repository_id, issue_number)
            .await?;

        let current_assignees = &current_issue.assignees;

        // Filter out assignees that are already assigned
        let mut added_assignees = Vec::new();
        let mut skipped_assignees = Vec::new();

        for assignee in new_assignees {
            if current_assignees.contains(assignee) {
                skipped_assignees.push(assignee.clone());
            } else {
                added_assignees.push(assignee.clone());
            }
        }

        // If there are new assignees to add, update the issue
        if !added_assignees.is_empty() {
            let mut updated_assignees = current_assignees.clone();
            updated_assignees.extend(added_assignees.clone());

            self.edit_assignees(repository_id, issue_number, &updated_assignees)
                .await?;
        }

        Ok((added_assignees, skipped_assignees))
    }

    /// Add labels to an issue
    ///
    /// Adds one or more labels to an existing issue. This operation does not
    /// remove existing labels - it only adds new ones.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier
    /// * `issue_number` - The issue number to add labels to
    /// * `labels` - Array of labels to add to the issue
    pub async fn add_labels(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        labels: &[Label],
    ) -> Result<()> {
        self.github_client
            .add_labels_to_issue(repository_id, issue_number, labels)
            .await
    }

    /// Set milestone for an issue
    ///
    /// Sets or updates the milestone for an existing issue. This operation
    /// replaces any existing milestone with the specified one.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier
    /// * `issue_number` - The issue number to set milestone for
    /// * `milestone_number` - The milestone ID to assign to the issue
    pub async fn set_milestone(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        milestone_number: MilestoneNumber,
    ) -> Result<()> {
        self.github_client
            .set_issue_milestone(repository_id, issue_number, milestone_number)
            .await
    }

    /// Remove milestone from an issue
    ///
    /// Removes the milestone from an existing issue, if one is set.
    /// This operation clears the milestone field for the issue.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier
    /// * `issue_number` - The issue number to remove milestone from
    pub async fn remove_milestone(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
    ) -> Result<()> {
        self.github_client
            .remove_issue_milestone(repository_id, issue_number)
            .await
    }
}
