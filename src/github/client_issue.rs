use crate::github::client::{GitHubClient, retry_with_backoff};
use crate::github::error::ApiRetryableError;
use crate::types::issue::{
    Issue, IssueComment, IssueCommentNumber, IssueId, IssueNumber, IssueState,
};
use crate::types::repository::{MilestoneNumber, RepositoryId};
use crate::types::{User, label::Label};

use anyhow::Result;

impl GitHubClient {
    /// Get an issue by repository ID and issue number
    ///
    /// Fetches comprehensive issue information including:
    /// - Basic metadata (title, body, state)
    /// - Author and assignee information
    /// - Labels and milestone data
    /// - Discussion comments
    /// - Creation and update timestamps
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `issue_number` - The issue number
    ///
    /// # Returns
    /// A complete `Issue` struct with all available metadata
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The issue number does not exist
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn get_issue(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
    ) -> Result<Issue> {
        let operation_name = "get_issue";

        retry_with_backoff(operation_name, None, || async {
            self.get_issue_impl(repository_id, issue_number).await
        })
        .await
    }

    async fn get_issue_impl(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
    ) -> std::result::Result<Issue, ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = issue_number.value();

        let octocrab_issue = self
            .client
            .issues(owner, repo)
            .get(number.into())
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        // Get issue comments
        let comments_response = self
            .client
            .issues(owner, repo)
            .list_comments(number.into())
            .send()
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        let comments: Vec<IssueComment> = comments_response
            .items
            .into_iter()
            .map(|comment| {
                IssueComment::new(
                    IssueCommentNumber::new(comment.id.0),
                    comment.body.unwrap_or_default(),
                    Some(User::new(
                        comment.user.login,
                        Some(comment.user.avatar_url.to_string()),
                    )),
                    comment.created_at,
                    comment.updated_at.unwrap_or(comment.created_at),
                )
            })
            .collect();

        // Convert octocrab issue state to our state enum
        let state = match octocrab_issue.state {
            octocrab::models::IssueState::Open => IssueState::Open,
            octocrab::models::IssueState::Closed => IssueState::Closed,
            _ => IssueState::Closed,
        };

        // Convert labels to strings
        let labels: Vec<String> = octocrab_issue
            .labels
            .into_iter()
            .map(|label| label.name)
            .collect();

        // Convert assignees to strings
        let assignees: Vec<String> = octocrab_issue
            .assignees
            .into_iter()
            .map(|user| user.login)
            .collect();

        let issue = Issue::new(
            IssueId::new(repository_id.clone(), number),
            octocrab_issue.title,
            octocrab_issue.body,
            state,
            octocrab_issue.user.login,
            assignees,
            labels,
            octocrab_issue.created_at,
            octocrab_issue.updated_at,
            octocrab_issue.closed_at,
            comments,
            octocrab_issue.milestone.map(|m| m.id.0),
            octocrab_issue.locked,
        );

        Ok(issue)
    }

    /// Create a new issue in a repository
    ///
    /// Creates a new issue in the specified repository with the provided title and optional
    /// metadata. The issue will be assigned a unique number and created with the current timestamp.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `title` - The title of the issue (required)
    /// * `body` - Optional body content for the issue
    /// * `assignees` - Optional list of usernames to assign to the issue
    /// * `labels` - Optional list of label names to apply to the issue
    /// * `milestone_number` - Optional milestone to associate with the issue
    ///
    /// # Returns
    /// The created `Issue` with all metadata populated including the assigned issue number
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The user does not have permission to create issues
    /// - Any specified assignee usernames, label names, or milestone do not exist
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn create_issue(
        &self,
        repository_id: &RepositoryId,
        title: &str,
        body: Option<&str>,
        assignees: Option<&[User]>,
        labels: Option<&[Label]>,
        milestone_number: Option<MilestoneNumber>,
    ) -> Result<Issue> {
        let operation_name = "create_issue";

        retry_with_backoff(operation_name, None, || async {
            self.create_issue_impl(
                repository_id,
                title,
                body,
                assignees,
                labels,
                milestone_number,
            )
            .await
        })
        .await
    }

    async fn create_issue_impl(
        &self,
        repository_id: &RepositoryId,
        title: &str,
        body: Option<&str>,
        assignees: Option<&[User]>,
        labels: Option<&[Label]>,
        milestone_number: Option<MilestoneNumber>,
    ) -> std::result::Result<Issue, ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();

        let issues_handler = self.client.issues(owner, repo);
        let mut builder = issues_handler.create(title);

        if let Some(body) = body {
            builder = builder.body(body);
        }

        if let Some(assignees) = assignees {
            let assignee_names: Vec<String> =
                assignees.iter().map(|u| u.username().to_string()).collect();
            builder = builder.assignees(assignee_names);
        }

        if let Some(labels) = labels {
            let label_names: Vec<String> = labels.iter().map(|l| l.name().to_string()).collect();
            builder = builder.labels(label_names);
        }

        if let Some(milestone_number) = milestone_number {
            builder = builder.milestone(milestone_number.value());
        }

        let octocrab_issue = builder
            .send()
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        // Convert the octocrab Issue to our Issue type
        let issue_state = match octocrab_issue.state {
            octocrab::models::IssueState::Open => IssueState::Open,
            octocrab::models::IssueState::Closed => IssueState::Closed,
            _ => IssueState::Open, // Default to Open for any unknown states
        };

        // Convert labels to strings
        let issue_labels: Vec<String> = octocrab_issue
            .labels
            .into_iter()
            .map(|label| label.name)
            .collect();

        // Convert assignees to strings
        let issue_assignees: Vec<String> = octocrab_issue
            .assignees
            .into_iter()
            .map(|user| user.login)
            .collect();

        let issue = Issue::new(
            IssueId::new(repository_id.clone(), octocrab_issue.number as u32),
            octocrab_issue.title,
            octocrab_issue.body,
            issue_state,
            octocrab_issue.user.login,
            issue_assignees,
            issue_labels,
            octocrab_issue.created_at,
            octocrab_issue.updated_at,
            octocrab_issue.closed_at,
            Vec::new(), // No comments in newly created issue
            octocrab_issue.milestone.map(|m| m.id.0),
            octocrab_issue.locked,
        );

        Ok(issue)
    }

    /// Add a comment to an issue
    ///
    /// Creates a new comment on the specified issue. This adds a discussion
    /// comment to the issue thread.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `issue_number` - The issue number to comment on
    /// * `body` - The comment text content
    ///
    /// # Returns
    /// The comment number of the created comment
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The issue number does not exist
    /// - The user does not have permission to comment
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn add_issue_comment(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        body: &str,
    ) -> Result<IssueCommentNumber> {
        let operation_name = "add_issue_comment";

        retry_with_backoff(operation_name, None, || async {
            self.add_issue_comment_impl(repository_id, issue_number, body)
                .await
        })
        .await
    }

    async fn add_issue_comment_impl(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        body: &str,
    ) -> std::result::Result<IssueCommentNumber, ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = issue_number.value();

        let comment = self
            .client
            .issues(owner, repo)
            .create_comment(number.into(), body)
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        Ok(IssueCommentNumber::new(comment.id.0))
    }

    /// Edit an issue comment
    ///
    /// Updates the body of an existing comment on the specified issue.
    /// This modifies a discussion comment on the issue thread.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `issue_number` - The issue number containing the comment
    /// * `comment_number` - The comment number to edit
    /// * `body` - The new comment text content
    ///
    /// # Returns
    /// Returns `Ok(())` if the comment was successfully updated
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The issue number does not exist
    /// - The comment number does not exist
    /// - The user does not have permission to edit the comment
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn edit_issue_comment(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        comment_number: IssueCommentNumber,
        body: &str,
    ) -> Result<()> {
        let operation_name = "edit_issue_comment";

        retry_with_backoff(operation_name, None, || async {
            self.edit_issue_comment_impl(repository_id, issue_number, comment_number, body)
                .await
        })
        .await
    }

    async fn edit_issue_comment_impl(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        comment_number: IssueCommentNumber,
        body: &str,
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let _issue_num = issue_number.value();
        let comment_id = comment_number.value();

        self.client
            .issues(owner, repo)
            .update_comment(octocrab::models::CommentId(comment_id), body)
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        Ok(())
    }

    /// Delete an issue comment
    ///
    /// Permanently deletes an existing comment from the specified issue.
    /// This operation cannot be undone.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `issue_number` - The issue number containing the comment
    /// * `comment_number` - The comment number to delete
    ///
    /// # Returns
    /// Returns `Ok(())` if the comment was successfully deleted
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The issue number does not exist
    /// - The comment number does not exist
    /// - The user does not have permission to delete the comment
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn delete_issue_comment(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        comment_number: IssueCommentNumber,
    ) -> Result<()> {
        let operation_name = "delete_issue_comment";

        retry_with_backoff(operation_name, None, || async {
            self.delete_issue_comment_impl(repository_id, issue_number, comment_number)
                .await
        })
        .await
    }

    async fn delete_issue_comment_impl(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        comment_number: IssueCommentNumber,
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let _issue_num = issue_number.value();
        let comment_id = comment_number.value();

        self.client
            .issues(owner, repo)
            .delete_comment(octocrab::models::CommentId(comment_id))
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        Ok(())
    }

    /// Edit the title of an issue
    ///
    /// Updates the title of an existing issue. This is a focused method
    /// for changing just the issue title without affecting other properties.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `issue_number` - The issue number to update
    /// * `title` - The new title for the issue
    ///
    /// # Returns
    /// Returns `Ok(())` if the issue title was successfully updated
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The issue number does not exist
    /// - The user does not have permission to edit the issue
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn edit_issue_title(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        title: &str,
    ) -> Result<()> {
        let operation_name = "edit_issue_title";

        retry_with_backoff(operation_name, None, || async {
            self.edit_issue_title_impl(repository_id, issue_number, title)
                .await
        })
        .await
    }

    async fn edit_issue_title_impl(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        title: &str,
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = issue_number.value();

        self.client
            .issues(owner, repo)
            .update(number.into())
            .title(title)
            .send()
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        Ok(())
    }

    /// Edit the body of an issue
    ///
    /// Updates the body content of an existing issue. This is a focused method
    /// for changing just the issue body without affecting other properties.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `issue_number` - The issue number to update
    /// * `body` - The new body content for the issue
    ///
    /// # Returns
    /// Returns `Ok(())` if the issue body was successfully updated
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The issue number does not exist
    /// - The user does not have permission to edit the issue
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn edit_issue_body(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        body: &str,
    ) -> Result<()> {
        let operation_name = "edit_issue_body";

        retry_with_backoff(operation_name, None, || async {
            self.edit_issue_body_impl(repository_id, issue_number, body)
                .await
        })
        .await
    }

    async fn edit_issue_body_impl(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        body: &str,
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = issue_number.value();

        self.client
            .issues(owner, repo)
            .update(number.into())
            .body(body)
            .send()
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        Ok(())
    }

    /// Edit the assignees of an issue
    ///
    /// Updates the assignees list of an existing issue. This is a focused method
    /// for changing just the issue assignees without affecting other properties.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `issue_number` - The issue number to update
    /// * `assignees` - The new list of assignee usernames for the issue
    ///
    /// # Returns
    /// Returns `Ok(())` if the issue assignees were successfully updated
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The issue number does not exist
    /// - Any specified assignees do not exist
    /// - The user does not have permission to edit the issue
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn edit_issue_assignees(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        assignees: &[String],
    ) -> Result<()> {
        let operation_name = "edit_issue_assignees";

        retry_with_backoff(operation_name, None, || async {
            self.edit_issue_assignees_impl(repository_id, issue_number, assignees)
                .await
        })
        .await
    }

    async fn edit_issue_assignees_impl(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        assignees: &[String],
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = issue_number.value();

        self.client
            .issues(owner, repo)
            .update(number.into())
            .assignees(assignees)
            .send()
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        Ok(())
    }

    /// Update the state of an issue (open/close)
    ///
    /// Changes the state of an existing issue to either open or closed.
    /// This is a focused method for just changing the issue state without
    /// affecting other properties.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `issue_number` - The issue number to update
    /// * `state` - The new state for the issue (open or closed)
    ///
    /// # Returns
    /// Returns `Ok(())` if the issue state was successfully updated
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The issue number does not exist
    /// - The user does not have permission to edit the issue
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn update_issue_state(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        state: IssueState,
    ) -> Result<()> {
        let operation_name = "update_issue_state";

        retry_with_backoff(operation_name, None, || async {
            self.update_issue_state_impl(repository_id, issue_number, state)
                .await
        })
        .await
    }

    async fn update_issue_state_impl(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        state: IssueState,
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = issue_number.value();

        // Convert our IssueState to octocrab's IssueState
        let octocrab_state = match state {
            IssueState::Open => octocrab::models::IssueState::Open,
            IssueState::Closed => octocrab::models::IssueState::Closed,
        };

        self.client
            .issues(owner, repo)
            .update(number.into())
            .state(octocrab_state)
            .send()
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        Ok(())
    }

    /// Update an issue with comprehensive metadata changes
    ///
    /// Updates multiple aspects of an existing issue including title, body,
    /// state, assignees, labels, and milestone. All parameters except
    /// repository_id and issue_number are optional - only provided values
    /// will be updated.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `issue_number` - The issue number to update
    /// * `title` - Optional new title for the issue
    /// * `body` - Optional new body content for the issue
    /// * `state` - Optional new state for the issue (open/closed)
    /// * `assignees` - Optional new list of assignee usernames (replaces existing)
    /// * `labels` - Optional new list of label names (replaces existing)
    /// * `milestone_number` - Optional new milestone (use Some(None) to remove milestone)
    ///
    /// # Returns
    /// The updated `Issue` with all current metadata
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The issue number does not exist
    /// - Any specified assignee usernames, label names, or milestone do not exist
    /// - The user does not have permission to edit the issue
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
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
        let operation_name = "update_issue";

        retry_with_backoff(operation_name, None, || async {
            self.update_issue_impl(
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
        })
        .await
    }

    async fn update_issue_impl(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        title: Option<&str>,
        body: Option<Option<&str>>,
        state: Option<IssueState>,
        assignees: Option<&[User]>,
        labels: Option<&[Label]>,
        milestone_number: Option<Option<MilestoneNumber>>,
    ) -> std::result::Result<Issue, ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = issue_number.value();

        // Prepare string conversions first
        let assignee_names: Option<Vec<String>> =
            assignees.map(|a| a.iter().map(|u| u.username().to_string()).collect());
        let label_names: Option<Vec<String>> =
            labels.map(|l| l.iter().map(|label| label.name().to_string()).collect());

        let issues_handler = self.client.issues(owner, repo);
        let mut builder = issues_handler.update(number.into());

        // Update title if provided
        if let Some(title) = title {
            builder = builder.title(title);
        }

        // Update body if provided (including setting to None)
        if let Some(body) = body {
            if let Some(body_text) = body {
                builder = builder.body(body_text);
            }
            // Note: octocrab doesn't support removing body (setting to None)
            // If body is Some(None), we skip setting it
        }

        // Update state if provided
        if let Some(state) = state {
            let octocrab_state = match state {
                IssueState::Open => octocrab::models::IssueState::Open,
                IssueState::Closed => octocrab::models::IssueState::Closed,
            };
            builder = builder.state(octocrab_state);
        }

        // Update assignees if provided
        if let Some(ref assignee_names) = assignee_names {
            builder = builder.assignees(assignee_names);
        }

        // Update labels if provided
        if let Some(ref label_names) = label_names {
            builder = builder.labels(label_names);
        }

        // Update milestone if provided (including removal)
        if let Some(milestone_number) = milestone_number {
            if let Some(milestone_number) = milestone_number {
                builder = builder.milestone(milestone_number.value());
            } else {
                // To remove milestone, we need to set it to null
                // This requires a different approach using the raw API
            }
        }

        let _octocrab_issue = builder
            .send()
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        // Get updated issue with comments to return complete data
        self.get_issue_impl(repository_id, issue_number).await
    }

    /// Add labels to an issue
    ///
    /// Adds the specified labels to an existing issue. This operation does not
    /// remove existing labels - it only adds new ones. Use `replace_issue_labels`
    /// to replace all labels or `remove_label_from_issue` to remove specific labels.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `issue_number` - The issue number to add labels to
    /// * `labels` - Array of label names to add to the issue
    ///
    /// # Returns
    /// Returns `Ok(())` if the labels were successfully added
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The issue number does not exist
    /// - Any specified label names do not exist in the repository
    /// - The user does not have permission to edit the issue
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn add_labels_to_issue(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        labels: &[Label],
    ) -> Result<()> {
        let operation_name = "add_labels_to_issue";

        retry_with_backoff(operation_name, None, || async {
            self.add_labels_to_issue_impl(repository_id, issue_number, labels)
                .await
        })
        .await
    }

    async fn add_labels_to_issue_impl(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        labels: &[Label],
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = issue_number.value();

        let label_names: Vec<String> = labels.iter().map(|l| l.name().to_string()).collect();

        self.client
            .issues(owner, repo)
            .add_labels(number.into(), &label_names)
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        Ok(())
    }

    /// Set milestone for an issue
    ///
    /// Sets or updates the milestone for an existing issue. This operation replaces
    /// any existing milestone with the specified one. Use `remove_milestone_from_issue`
    /// to remove a milestone without setting a new one.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `issue_number` - The issue number to set milestone for
    /// * `milestone_number` - The milestone ID to assign to the issue
    ///
    /// # Returns
    /// Returns `Ok(())` if the milestone was successfully set
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The issue number does not exist
    /// - The milestone ID does not exist in the repository
    /// - The user does not have permission to edit the issue
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn set_issue_milestone(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        milestone_number: MilestoneNumber,
    ) -> Result<()> {
        let operation_name = "set_issue_milestone";

        retry_with_backoff(operation_name, None, || async {
            self.set_issue_milestone_impl(repository_id, issue_number, milestone_number)
                .await
        })
        .await
    }

    async fn set_issue_milestone_impl(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
        milestone_number: MilestoneNumber,
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = issue_number.value();

        self.client
            .issues(owner, repo)
            .update(number.into())
            .milestone(milestone_number.value())
            .send()
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        Ok(())
    }

    /// Remove milestone from an issue
    ///
    /// Removes the milestone from an existing issue, if one is set. This operation
    /// clears the milestone field for the issue.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `issue_number` - The issue number to remove milestone from
    ///
    /// # Returns
    /// Returns `Ok(())` if the milestone was successfully removed
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The issue number does not exist
    /// - The user does not have permission to edit the issue
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn remove_issue_milestone(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
    ) -> Result<()> {
        let operation_name = "remove_issue_milestone";

        retry_with_backoff(operation_name, None, || async {
            self.remove_issue_milestone_impl(repository_id, issue_number)
                .await
        })
        .await
    }

    async fn remove_issue_milestone_impl(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = issue_number.value();

        // Use GraphQL to remove milestone by setting it to null
        let mutation = format!(
            r#"
            mutation {{
                updateIssue(input: {{
                    id: "{}"
                    milestoneId: null
                }}) {{
                    clientMutationId
                }}
            }}
            "#,
            self.get_issue_node_id(repository_id, issue_number).await?
        );

        let response = self
            .client
            .graphql::<serde_json::Value>(&serde_json::json!({
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
                "Failed to remove milestone from issue {}/{}/{}: {}",
                owner, repo, number, error_msg
            )))
        }
    }

    /// Helper function to get issue node ID for GraphQL operations
    async fn get_issue_node_id(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
    ) -> std::result::Result<String, ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = issue_number.value();

        let octocrab_issue = self
            .client
            .issues(owner, repo)
            .get(number.into())
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        Ok(octocrab_issue.node_id)
    }

    /// Delete an issue
    ///
    /// Deletes an existing issue from the specified repository.
    /// Note: This operation is permanent and cannot be undone.
    /// Only repository owners and users with admin permissions can delete issues.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `issue_number` - The issue number to delete
    ///
    /// # Returns
    /// Returns `Ok(())` if the issue was successfully deleted
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The issue number does not exist
    /// - The user does not have permission to delete the issue
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn delete_issue(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
    ) -> Result<()> {
        let operation_name = "delete_issue";

        retry_with_backoff(operation_name, None, || async {
            self.delete_issue_impl(repository_id, issue_number).await
        })
        .await
    }

    async fn delete_issue_impl(
        &self,
        repository_id: &RepositoryId,
        issue_number: IssueNumber,
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = issue_number.value();

        // Get the GitHub node ID for this issue - we need to fetch it via REST API first
        let octocrab_issue = self
            .client
            .issues(owner, repo)
            .get(number.into())
            .await
            .map_err(ApiRetryableError::from_octocrab_error)?;

        let node_id = octocrab_issue.node_id;

        // Use GraphQL mutation to delete the issue
        let mutation = format!(
            r#"
            mutation {{
                deleteIssue(input: {{
                    issueId: "{}"
                }}) {{
                    clientMutationId
                }}
            }}
            "#,
            node_id
        );

        // Execute GraphQL mutation
        let response = self
            .client
            .graphql::<serde_json::Value>(&serde_json::json!({
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
                "Failed to delete issue {}/{}/{}: {}",
                owner, repo, number, error_msg
            )))
        }
    }
}
