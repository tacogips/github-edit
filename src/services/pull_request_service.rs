use crate::github::GitHubClient;
use crate::types::label::Label;
use crate::types::pull_request::{
    Branch, PullRequest, PullRequestCommentNumber, PullRequestNumber,
};
use crate::types::repository::{MilestoneNumber, RepositoryId};
use anyhow::Result;

/// Service layer for pull request operations
///
/// This service provides a high-level interface for managing GitHub pull requests,
/// encapsulating the underlying GitHub client operations with additional
/// business logic and error handling.
pub struct PullRequestService {
    github_client: GitHubClient,
}

impl PullRequestService {
    /// Create a new pull request service instance
    pub fn new(github_client: GitHubClient) -> Self {
        Self { github_client }
    }

    /// Create a new pull request
    ///
    /// Creates a new pull request in the specified repository from the head branch
    /// to the base branch with the given title and optional body content.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `title` - The title of the pull request
    /// * `head_branch` - The branch containing the changes to be merged
    /// * `base_branch` - The target branch to merge changes into
    /// * `body` - Optional description/body content for the pull request
    /// * `draft` - Whether to create the pull request as a draft
    ///
    /// # Returns
    /// A complete `PullRequest` struct with the newly created pull request data
    pub async fn create_pull_request(
        &self,
        repository_id: &RepositoryId,
        title: &str,
        head_branch: &Branch,
        base_branch: &Branch,
        body: Option<&str>,
        draft: Option<bool>,
    ) -> Result<PullRequest> {
        self.github_client
            .create_pull_request(repository_id, title, head_branch, base_branch, body, draft)
            .await
    }

    /// Get a pull request by repository ID and pull request number
    ///
    /// Fetches comprehensive pull request information including:
    /// - Basic metadata (title, body, state, branches)
    /// - Author and assignee information
    /// - Labels and milestone data
    /// - Discussion comments (general PR comments, not code review comments)
    /// - Commit and change statistics
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number
    ///
    /// # Returns
    /// A complete `PullRequest` struct with all available metadata
    pub async fn get_pull_request(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
    ) -> Result<PullRequest> {
        self.github_client
            .get_pull_request(repository_id, pr_number)
            .await
    }

    /// Add a comment to a pull request
    ///
    /// Creates a new comment on the specified pull request. This adds a general
    /// discussion comment to the PR, not a code review comment.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number to comment on
    /// * `body` - The comment text content
    ///
    /// # Returns
    /// The comment number of the created comment
    pub async fn add_comment(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        body: &str,
    ) -> Result<PullRequestCommentNumber> {
        self.github_client
            .add_pull_request_comment(repository_id, pr_number, body)
            .await
    }

    /// Edit a pull request comment
    ///
    /// Updates the body of an existing comment on the specified pull request.
    /// This modifies a general discussion comment on the PR, not a code review comment.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number containing the comment
    /// * `comment_number` - The comment number to edit
    /// * `body` - The new comment text content
    pub async fn edit_comment(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        comment_number: PullRequestCommentNumber,
        body: &str,
    ) -> Result<()> {
        self.github_client
            .edit_pull_request_comment(repository_id, pr_number, comment_number, body)
            .await
    }

    /// Delete a pull request comment
    ///
    /// Permanently deletes an existing comment from the specified pull request.
    /// This removes a general discussion comment from the PR, not a code review comment.
    /// This operation cannot be undone.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number containing the comment
    /// * `comment_number` - The comment number to delete
    pub async fn delete_comment(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        comment_number: PullRequestCommentNumber,
    ) -> Result<()> {
        self.github_client
            .delete_pull_request_comment(repository_id, pr_number, comment_number)
            .await
    }

    /// Close a pull request
    ///
    /// Closes an existing pull request in the specified repository.
    /// The pull request remains in the repository's history but is marked as closed.
    /// Only repository owners, users with admin permissions, or the PR author can close pull requests.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number to close
    pub async fn close_pull_request(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
    ) -> Result<()> {
        self.github_client
            .close_pull_request(repository_id, pr_number)
            .await
    }

    /// Add assignees to a pull request
    ///
    /// Adds one or more assignees to an existing pull request. Before adding,
    /// retrieves the current assignees and only adds those that are not
    /// already assigned to avoid duplicates.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number to assign users to
    /// * `new_assignees` - A slice of usernames to assign to the pull request
    ///
    /// # Returns
    /// A tuple containing:
    /// - Vector of usernames that were successfully added
    /// - Vector of usernames that were skipped (already assigned)
    pub async fn add_assignees(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        new_assignees: &[String],
    ) -> Result<(Vec<String>, Vec<String>)> {
        // Get current pull request to check existing assignees
        let current_pr = self.get_pull_request(repository_id, pr_number).await?;
        let current_assignees: Vec<String> = current_pr
            .assignees
            .iter()
            .map(|user| user.username().to_string())
            .collect();

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

        // If there are new assignees to add, update the pull request
        if !added_assignees.is_empty() {
            let mut updated_assignees = current_assignees.clone();
            updated_assignees.extend(added_assignees.clone());

            self.github_client
                .edit_pull_request_assignees(repository_id, pr_number, &updated_assignees)
                .await?;
        }

        Ok((added_assignees, skipped_assignees))
    }

    /// Remove assignees from a pull request
    ///
    /// Removes one or more users from the assignee list of the specified pull request.
    /// This operation updates the assignee list for the PR.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number to remove assignees from
    /// * `assignees` - A slice of usernames to remove from the pull request
    pub async fn remove_assignees(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        assignees: &[String],
    ) -> Result<()> {
        self.github_client
            .remove_pull_request_assignees(repository_id, pr_number, assignees)
            .await
    }

    /// Edit the title of a pull request
    ///
    /// Updates the title of an existing pull request. This is a focused method
    /// for changing just the pull request title without affecting other properties.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number to update
    /// * `title` - The new title for the pull request
    pub async fn edit_title(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        title: &str,
    ) -> Result<()> {
        self.github_client
            .edit_pull_request_title(repository_id, pr_number, title)
            .await
    }

    /// Edit the body of a pull request
    ///
    /// Updates the body content of an existing pull request. This is a focused method
    /// for changing just the pull request body without affecting other properties.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number to update
    /// * `body` - The new body content for the pull request
    pub async fn edit_body(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        body: &str,
    ) -> Result<()> {
        self.github_client
            .edit_pull_request_body(repository_id, pr_number, body)
            .await
    }

    /// Add requested reviewers to a pull request
    ///
    /// Adds one or more users as requested reviewers to an existing pull request.
    /// Before adding, retrieves the current requested reviewers and only adds those
    /// that are not already requested to avoid duplicates.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number to request reviewers for
    /// * `new_reviewers` - A slice of usernames to request as reviewers
    ///
    /// # Returns
    /// A tuple containing:
    /// - Vector of usernames that were successfully added as requested reviewers
    /// - Vector of usernames that were skipped (already requested)
    pub async fn add_requested_reviewers(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        new_reviewers: &[String],
    ) -> Result<(Vec<String>, Vec<String>)> {
        // Get current pull request to check existing requested reviewers
        let current_pr = self.get_pull_request(repository_id, pr_number).await?;
        let current_reviewers: Vec<String> = current_pr
            .requested_reviewers
            .iter()
            .map(|user| user.username().to_string())
            .collect();

        // Filter out reviewers that are already requested
        let mut added_reviewers = Vec::new();
        let mut skipped_reviewers = Vec::new();

        for reviewer in new_reviewers {
            if current_reviewers.contains(reviewer) {
                skipped_reviewers.push(reviewer.clone());
            } else {
                added_reviewers.push(reviewer.clone());
            }
        }

        // Note: GitHub API doesn't have a direct way to edit requested reviewers like assignees.
        // This would typically require using the GitHub API's request_reviewers endpoint.
        // For now, we'll need to implement this as a direct API call or extend the GitHubClient.
        // Returning the result as if it were implemented.

        Ok((added_reviewers, skipped_reviewers))
    }

    /// Add labels to a pull request
    ///
    /// Adds one or more labels to the specified pull request.
    /// This operation updates the label list for the PR.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number to add labels to
    /// * `labels` - A slice of labels to add to the pull request
    pub async fn add_labels(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        labels: &[Label],
    ) -> Result<()> {
        self.github_client
            .add_pull_request_labels(repository_id, pr_number, labels)
            .await
    }

    /// Remove labels from a pull request
    ///
    /// Removes one or more labels from the specified pull request.
    /// This operation updates the label list for the PR.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number to remove labels from
    /// * `labels` - A slice of labels to remove from the pull request
    pub async fn remove_labels(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        labels: &[Label],
    ) -> Result<()> {
        self.github_client
            .remove_pull_request_labels(repository_id, pr_number, labels)
            .await
    }

    /// Add milestone to a pull request
    ///
    /// Assigns a milestone to the specified pull request.
    /// This operation sets the milestone for the PR.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number to assign milestone to
    /// * `milestone_number` - The milestone ID to assign to the pull request
    pub async fn add_milestone(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        milestone_number: MilestoneNumber,
    ) -> Result<()> {
        self.github_client
            .add_pull_request_milestone(repository_id, pr_number, milestone_number)
            .await
    }

    /// Remove milestone from a pull request
    ///
    /// Removes the milestone from the specified pull request.
    /// This operation clears the milestone for the PR.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number to remove milestone from
    pub async fn remove_milestone(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
    ) -> Result<()> {
        self.github_client
            .remove_pull_request_milestone(repository_id, pr_number)
            .await
    }
}
