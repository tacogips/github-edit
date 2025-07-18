use crate::github::client::retry_with_backoff;
use crate::github::error::ApiRetryableError;
use crate::types::pull_request::{
    Branch, PullRequest, PullRequestComment, PullRequestCommentNumber, PullRequestNumber,
    PullRequestState,
};
use crate::types::repository::{MilestoneNumber, RepositoryId};
use crate::types::{User, label::Label};

use anyhow::Result;

impl crate::github::client::GitHubClient {
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
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The head or base branch does not exist
    /// - The user does not have permission to create pull requests
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn create_pull_request(
        &self,
        repository_id: &RepositoryId,
        title: &str,
        head_branch: &Branch,
        base_branch: &Branch,
        body: Option<&str>,
        draft: Option<bool>,
    ) -> Result<PullRequest> {
        let operation_name = "create_pull_request";

        retry_with_backoff(operation_name, None, || async {
            self.create_pull_request_impl(
                repository_id,
                title,
                head_branch,
                base_branch,
                body,
                draft,
            )
            .await
        })
        .await
    }

    async fn create_pull_request_impl(
        &self,
        repository_id: &RepositoryId,
        title: &str,
        head_branch: &Branch,
        base_branch: &Branch,
        body: Option<&str>,
        draft: Option<bool>,
    ) -> std::result::Result<PullRequest, ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();

        // Create the pull request using octocrab
        let pulls_handler = self.client.pulls(owner, repo);
        let mut pr_builder = pulls_handler.create(title, &head_branch.0, &base_branch.0);

        // Add optional body if provided
        if let Some(body_content) = body {
            pr_builder = pr_builder.body(body_content);
        }

        // Set draft status if provided
        if let Some(is_draft) = draft {
            pr_builder = pr_builder.draft(is_draft);
        }

        let octocrab_pr = pr_builder
            .send()
            .await
            .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;

        // Convert the created PR to our internal PullRequest type
        // by fetching it again to get complete data
        let pr_number = PullRequestNumber::new(octocrab_pr.number as u32);
        self.get_pull_request_impl(repository_id, pr_number).await
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
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The pull request number does not exist
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn get_pull_request(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
    ) -> Result<PullRequest> {
        let operation_name = "get_pull_request";

        retry_with_backoff(operation_name, None, || async {
            self.get_pull_request_impl(repository_id, pr_number).await
        })
        .await
    }

    async fn get_pull_request_impl(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
    ) -> std::result::Result<PullRequest, ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = pr_number.value();

        let octocrab_pr = self
            .client
            .pulls(owner, repo)
            .get(number.into())
            .await
            .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;

        // Get PR discussion comments (issue comments API is correct for general PR discussion)
        let comments_response = self
            .client
            .issues(owner, repo)
            .list_comments(number.into())
            .send()
            .await
            .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;

        let comments: Vec<PullRequestComment> = comments_response
            .items
            .into_iter()
            .map(|comment| {
                PullRequestComment::new(
                    PullRequestCommentNumber::new(comment.id.0),
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

        // Convert octocrab PR state to our state enum
        let state = match octocrab_pr.state.unwrap() {
            octocrab::models::IssueState::Open => PullRequestState::Open,
            octocrab::models::IssueState::Closed => {
                if octocrab_pr.merged_at.is_some() {
                    PullRequestState::Merged
                } else {
                    PullRequestState::Closed
                }
            }
            _ => PullRequestState::Closed,
        };

        // Convert labels
        let labels: Vec<Label> = octocrab_pr
            .labels
            .unwrap_or_default()
            .into_iter()
            .map(|label| Label::new(label.name, Some(label.color)))
            .collect();

        // Convert assignees
        let assignees: Vec<User> = octocrab_pr
            .assignees
            .unwrap_or_default()
            .into_iter()
            .map(|user| User::new(user.login, Some(user.avatar_url.to_string())))
            .collect();

        // Convert requested reviewers
        let requested_reviewers: Vec<User> = octocrab_pr
            .requested_reviewers
            .unwrap_or_default()
            .into_iter()
            .map(|user| User::new(user.login, Some(user.avatar_url.to_string())))
            .collect();

        let pull_request = PullRequest {
            pull_request_id: crate::types::pull_request::PullRequestId::new(
                repository_id.clone(),
                number,
            ),
            title: octocrab_pr.title.unwrap_or_default(),
            body: octocrab_pr.body,
            state,
            author: octocrab_pr
                .user
                .map(|u| User::new(u.login, Some(u.avatar_url.to_string()))),
            assignees,
            requested_reviewers,
            labels,
            head_branch: octocrab_pr.head.ref_field,
            base_branch: octocrab_pr.base.ref_field,
            created_at: octocrab_pr.created_at.unwrap(),
            updated_at: octocrab_pr.updated_at.unwrap(),
            closed_at: octocrab_pr.closed_at,
            merged_at: octocrab_pr.merged_at,
            commits_count: octocrab_pr.commits.unwrap_or(0) as u32,
            additions: octocrab_pr.additions.unwrap_or(0) as u32,
            deletions: octocrab_pr.deletions.unwrap_or(0) as u32,
            changed_files: octocrab_pr.changed_files.unwrap_or(0) as u32,
            comments,
            milestone_number: octocrab_pr.milestone.map(|m| m.id.0),
            draft: octocrab_pr.draft.unwrap_or(false),
            mergeable: octocrab_pr.mergeable,
        };

        Ok(pull_request)
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
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The pull request number does not exist
    /// - The user does not have permission to comment
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn add_pull_request_comment(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        body: &str,
    ) -> Result<PullRequestCommentNumber> {
        let operation_name = "add_pull_request_comment";

        retry_with_backoff(operation_name, None, || async {
            self.add_pull_request_comment_impl(repository_id, pr_number, body)
                .await
        })
        .await
    }

    async fn add_pull_request_comment_impl(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        body: &str,
    ) -> std::result::Result<PullRequestCommentNumber, ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = pr_number.value();

        let comment = self
            .client
            .issues(owner, repo)
            .create_comment(number.into(), body)
            .await
            .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;

        Ok(PullRequestCommentNumber::new(comment.id.0))
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
    ///
    /// # Returns
    /// Returns `Ok(())` if the comment was successfully updated
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The pull request number does not exist
    /// - The comment number does not exist
    /// - The user does not have permission to edit the comment
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn edit_pull_request_comment(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        comment_number: PullRequestCommentNumber,
        body: &str,
    ) -> Result<()> {
        let operation_name = "edit_pull_request_comment";

        retry_with_backoff(operation_name, None, || async {
            self.edit_pull_request_comment_impl(repository_id, pr_number, comment_number, body)
                .await
        })
        .await
    }

    async fn edit_pull_request_comment_impl(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        comment_number: PullRequestCommentNumber,
        body: &str,
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let _pr_num = pr_number.value();
        let comment_id = comment_number.value();

        self.client
            .issues(owner, repo)
            .update_comment(octocrab::models::CommentId(comment_id), body)
            .await
            .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;

        Ok(())
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
    ///
    /// # Returns
    /// Returns `Ok(())` if the comment was successfully deleted
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The pull request number does not exist
    /// - The comment number does not exist
    /// - The user does not have permission to delete the comment
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn delete_pull_request_comment(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        comment_number: PullRequestCommentNumber,
    ) -> Result<()> {
        let operation_name = "delete_pull_request_comment";

        retry_with_backoff(operation_name, None, || async {
            self.delete_pull_request_comment_impl(repository_id, pr_number, comment_number)
                .await
        })
        .await
    }

    async fn delete_pull_request_comment_impl(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        comment_number: PullRequestCommentNumber,
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let _pr_num = pr_number.value();
        let comment_id = comment_number.value();

        self.client
            .issues(owner, repo)
            .delete_comment(octocrab::models::CommentId(comment_id))
            .await
            .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;

        Ok(())
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
    ///
    /// # Returns
    /// Returns `Ok(())` if the pull request was successfully closed
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The pull request number does not exist
    /// - The user does not have permission to close the pull request
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn close_pull_request(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
    ) -> Result<()> {
        let operation_name = "close_pull_request";

        retry_with_backoff(operation_name, None, || async {
            self.close_pull_request_impl(repository_id, pr_number).await
        })
        .await
    }

    async fn close_pull_request_impl(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = pr_number.value();

        // Get the GitHub node ID for this pull request - we need to fetch it via REST API first
        let octocrab_pr = self
            .client
            .pulls(owner, repo)
            .get(number.into())
            .await
            .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;

        let node_id = octocrab_pr.node_id.ok_or_else(|| {
            ApiRetryableError::NonRetryable(format!(
                "Pull request {}/{}/{} has no node_id",
                owner, repo, number
            ))
        })?;

        // Use GraphQL mutation to close the pull request
        let mutation = format!(
            r#"
            mutation {{
                closePullRequest(input: {{
                    pullRequestId: "{}"
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
            .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;

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
                "Failed to close pull request {}/{}/{}: {}",
                owner, repo, number, error_msg
            )))
        }
    }

    /// Add assignees to a pull request
    ///
    /// Adds one or more users as assignees to the specified pull request.
    /// This operation updates the assignee list for the PR.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number to assign users to
    /// * `assignees` - A slice of usernames to assign to the pull request
    ///
    /// # Returns
    /// Returns `Ok(())` if the assignees were successfully added
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The pull request number does not exist
    /// - Any of the specified users do not exist
    /// - The user does not have permission to modify assignees
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn add_pull_request_assignees(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        assignees: &[String],
    ) -> Result<()> {
        let operation_name = "add_pull_request_assignees";

        retry_with_backoff(operation_name, None, || async {
            self.add_pull_request_assignees_impl(repository_id, pr_number, assignees)
                .await
        })
        .await
    }

    async fn add_pull_request_assignees_impl(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        assignees: &[String],
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = pr_number.value();

        let assignee_refs: Vec<&str> = assignees.iter().map(|s| s.as_str()).collect();
        self.client
            .issues(owner, repo)
            .add_assignees(number.into(), &assignee_refs)
            .await
            .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;

        Ok(())
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
    ///
    /// # Returns
    /// Returns `Ok(())` if the assignees were successfully removed
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The pull request number does not exist
    /// - Any of the specified users are not currently assigned
    /// - The user does not have permission to modify assignees
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn remove_pull_request_assignees(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        assignees: &[String],
    ) -> Result<()> {
        let operation_name = "remove_pull_request_assignees";

        retry_with_backoff(operation_name, None, || async {
            self.remove_pull_request_assignees_impl(repository_id, pr_number, assignees)
                .await
        })
        .await
    }

    async fn remove_pull_request_assignees_impl(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        assignees: &[String],
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = pr_number.value();

        let assignee_refs: Vec<&str> = assignees.iter().map(|s| s.as_str()).collect();
        self.client
            .issues(owner, repo)
            .remove_assignees(number.into(), &assignee_refs)
            .await
            .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;

        Ok(())
    }

    /// Edit (replace) assignees of a pull request
    ///
    /// Completely replaces the current assignee list with the specified users.
    /// This operation first fetches the current assignees, removes all of them,
    /// and then adds the new assignees provided.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number to edit assignees for
    /// * `new_assignees` - A slice of usernames to set as the new assignees
    ///
    /// # Returns
    /// Returns `Ok(())` if the assignees were successfully replaced
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The pull request number does not exist
    /// - Any of the specified users do not exist
    /// - The user does not have permission to modify assignees
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn edit_pull_request_assignees(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        new_assignees: &[String],
    ) -> Result<()> {
        let operation_name = "edit_pull_request_assignees";

        retry_with_backoff(operation_name, None, || async {
            self.edit_pull_request_assignees_impl(repository_id, pr_number, new_assignees)
                .await
        })
        .await
    }

    async fn edit_pull_request_assignees_impl(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        new_assignees: &[String],
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = pr_number.value();

        // First, get the current pull request to see existing assignees
        let current_pr = self.get_pull_request_impl(repository_id, pr_number).await?;

        // Extract current assignee usernames
        let current_assignees: Vec<String> = current_pr
            .assignees
            .iter()
            .map(|user| user.username().to_string())
            .collect();

        // Remove all current assignees if any exist
        if !current_assignees.is_empty() {
            let current_assignee_refs: Vec<&str> =
                current_assignees.iter().map(|s| s.as_str()).collect();
            self.client
                .issues(owner, repo)
                .remove_assignees(number.into(), &current_assignee_refs)
                .await
                .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;
        }

        // Add new assignees if any specified
        if !new_assignees.is_empty() {
            let new_assignee_refs: Vec<&str> = new_assignees.iter().map(|s| s.as_str()).collect();
            self.client
                .issues(owner, repo)
                .add_assignees(number.into(), &new_assignee_refs)
                .await
                .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;
        }

        Ok(())
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
    ///
    /// # Returns
    /// Returns `Ok(())` if the pull request title was successfully updated
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The pull request number does not exist
    /// - The user does not have permission to edit the pull request
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn edit_pull_request_title(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        title: &str,
    ) -> Result<()> {
        let operation_name = "edit_pull_request_title";

        retry_with_backoff(operation_name, None, || async {
            self.edit_pull_request_title_impl(repository_id, pr_number, title)
                .await
        })
        .await
    }

    async fn edit_pull_request_title_impl(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        title: &str,
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = pr_number.value();

        self.client
            .pulls(owner, repo)
            .update(number.into())
            .title(title)
            .send()
            .await
            .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;

        Ok(())
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
    ///
    /// # Returns
    /// Returns `Ok(())` if the pull request body was successfully updated
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The pull request number does not exist
    /// - The user does not have permission to edit the pull request
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn edit_pull_request_body(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        body: &str,
    ) -> Result<()> {
        let operation_name = "edit_pull_request_body";

        retry_with_backoff(operation_name, None, || async {
            self.edit_pull_request_body_impl(repository_id, pr_number, body)
                .await
        })
        .await
    }

    async fn edit_pull_request_body_impl(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        body: &str,
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = pr_number.value();

        self.client
            .pulls(owner, repo)
            .update(number.into())
            .body(body)
            .send()
            .await
            .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;

        Ok(())
    }

    /// Add labels to a pull request
    ///
    /// Adds one or more labels to the specified pull request.
    /// This operation updates the label list for the PR.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number to add labels to
    /// * `labels` - A slice of label names to add to the pull request
    ///
    /// # Returns
    /// Returns `Ok(())` if the labels were successfully added
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The pull request number does not exist
    /// - Any of the specified labels do not exist in the repository
    /// - The user does not have permission to modify labels
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn add_pull_request_labels(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        labels: &[Label],
    ) -> Result<()> {
        let operation_name = "add_pull_request_labels";

        retry_with_backoff(operation_name, None, || async {
            self.add_pull_request_labels_impl(repository_id, pr_number, labels)
                .await
        })
        .await
    }

    async fn add_pull_request_labels_impl(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        labels: &[Label],
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = pr_number.value();

        let label_names: Vec<String> = labels.iter().map(|l| l.name().to_string()).collect();

        self.client
            .issues(owner, repo)
            .add_labels(number.into(), &label_names)
            .await
            .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;

        Ok(())
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
    ///
    /// # Returns
    /// Returns `Ok(())` if the labels were successfully removed
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The pull request number does not exist
    /// - Any of the specified labels are not currently on the pull request
    /// - The user does not have permission to modify labels
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn remove_pull_request_labels(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        labels: &[Label],
    ) -> Result<()> {
        let operation_name = "remove_pull_request_labels";

        retry_with_backoff(operation_name, None, || async {
            self.remove_pull_request_labels_impl(repository_id, pr_number, labels)
                .await
        })
        .await
    }

    async fn remove_pull_request_labels_impl(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        labels: &[Label],
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = pr_number.value();

        for label in labels {
            self.client
                .issues(owner, repo)
                .remove_label(number.into(), label.name())
                .await
                .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;
        }

        Ok(())
    }

    /// Edit (replace) labels of a pull request
    ///
    /// Completely replaces the current label list with the specified labels.
    /// This operation first fetches the current labels, removes all of them,
    /// and then adds the new labels provided.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number to edit labels for
    /// * `new_labels` - A slice of labels to set as the new labels
    ///
    /// # Returns
    /// Returns `Ok(())` if the labels were successfully replaced
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The pull request number does not exist
    /// - Any of the specified labels do not exist in the repository
    /// - The user does not have permission to modify labels
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn edit_pull_request_labels(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        new_labels: &[Label],
    ) -> Result<()> {
        let operation_name = "edit_pull_request_labels";

        retry_with_backoff(operation_name, None, || async {
            self.edit_pull_request_labels_impl(repository_id, pr_number, new_labels)
                .await
        })
        .await
    }

    async fn edit_pull_request_labels_impl(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        new_labels: &[Label],
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = pr_number.value();

        // First, get the current pull request to see existing labels
        let current_pr = self.get_pull_request_impl(repository_id, pr_number).await?;

        // Extract current label names
        let current_labels: Vec<String> = current_pr
            .labels
            .iter()
            .map(|label| label.name().to_string())
            .collect();

        // Remove all current labels if any exist
        if !current_labels.is_empty() {
            for label in &current_labels {
                self.client
                    .issues(owner, repo)
                    .remove_label(number.into(), label)
                    .await
                    .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;
            }
        }

        // Add new labels if any specified
        if !new_labels.is_empty() {
            let label_names: Vec<String> =
                new_labels.iter().map(|l| l.name().to_string()).collect();
            self.client
                .issues(owner, repo)
                .add_labels(number.into(), &label_names)
                .await
                .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;
        }

        Ok(())
    }

    /// Add a milestone to a pull request
    ///
    /// Assigns a milestone to the specified pull request.
    /// This operation sets the milestone for the PR.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number to assign milestone to
    /// * `milestone_number` - The milestone ID to assign to the pull request
    ///
    /// # Returns
    /// Returns `Ok(())` if the milestone was successfully assigned
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The pull request number does not exist
    /// - The milestone ID does not exist in the repository
    /// - The user does not have permission to modify milestones
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn add_pull_request_milestone(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        milestone_number: MilestoneNumber,
    ) -> Result<()> {
        let operation_name = "add_pull_request_milestone";

        retry_with_backoff(operation_name, None, || async {
            self.add_pull_request_milestone_impl(repository_id, pr_number, milestone_number)
                .await
        })
        .await
    }

    async fn add_pull_request_milestone_impl(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        milestone_number: MilestoneNumber,
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = pr_number.value();

        self.client
            .issues(owner, repo)
            .update(number.into())
            .milestone(milestone_number.value())
            .send()
            .await
            .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;

        Ok(())
    }

    /// Remove milestone from a pull request
    ///
    /// Removes the milestone from the specified pull request.
    /// This operation clears the milestone for the PR.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number to remove milestone from
    ///
    /// # Returns
    /// Returns `Ok(())` if the milestone was successfully removed
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The pull request number does not exist
    /// - The user does not have permission to modify milestones
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn remove_pull_request_milestone(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
    ) -> Result<()> {
        let operation_name = "remove_pull_request_milestone";

        retry_with_backoff(operation_name, None, || async {
            self.remove_pull_request_milestone_impl(repository_id, pr_number)
                .await
        })
        .await
    }

    async fn remove_pull_request_milestone_impl(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = pr_number.value();

        // Use GraphQL to remove milestone by setting it to null
        let mutation = format!(
            r#"
            mutation {{
                updatePullRequest(input: {{
                    pullRequestId: "{}"
                    milestoneId: null
                }}) {{
                    clientMutationId
                }}
            }}
            "#,
            self.get_pull_request_node_id(repository_id, pr_number)
                .await?
        );

        let response = self
            .client
            .graphql::<serde_json::Value>(&serde_json::json!({
                "query": mutation
            }))
            .await
            .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;

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
                "Failed to remove milestone from pull request {}/{}/{}: {}",
                owner, repo, number, error_msg
            )))
        }
    }

    /// Edit (replace) milestone of a pull request
    ///
    /// Replaces the current milestone with the specified milestone.
    /// This operation updates the milestone for the PR.
    ///
    /// # Arguments
    /// * `repository_id` - The repository identifier containing owner and repo name
    /// * `pr_number` - The pull request number to edit milestone for
    /// * `milestone_number` - Optional milestone ID to set (use None to remove milestone)
    ///
    /// # Returns
    /// Returns `Ok(())` if the milestone was successfully updated
    ///
    /// # Errors
    /// Returns an error if:
    /// - The repository does not exist or is not accessible
    /// - The pull request number does not exist
    /// - The milestone ID does not exist in the repository (if provided)
    /// - The user does not have permission to modify milestones
    /// - API rate limits are exceeded (with automatic retry)
    /// - Network errors occur (with automatic retry)
    pub async fn edit_pull_request_milestone(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        milestone_number: Option<MilestoneNumber>,
    ) -> Result<()> {
        let operation_name = "edit_pull_request_milestone";

        retry_with_backoff(operation_name, None, || async {
            self.edit_pull_request_milestone_impl(repository_id, pr_number, milestone_number)
                .await
        })
        .await
    }

    async fn edit_pull_request_milestone_impl(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
        milestone_number: Option<MilestoneNumber>,
    ) -> std::result::Result<(), ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = pr_number.value();

        if let Some(milestone_number) = milestone_number {
            self.client
                .issues(owner, repo)
                .update(number.into())
                .milestone(milestone_number.value())
                .send()
                .await
                .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;
        } else {
            // For removing milestone, use the GraphQL approach
            return self
                .remove_pull_request_milestone_impl(repository_id, pr_number)
                .await;
        }

        Ok(())
    }

    /// Helper method to get pull request node ID for GraphQL operations
    async fn get_pull_request_node_id(
        &self,
        repository_id: &RepositoryId,
        pr_number: PullRequestNumber,
    ) -> std::result::Result<String, ApiRetryableError> {
        let owner = repository_id.owner().as_str();
        let repo = repository_id.repo_name().as_str();
        let number = pr_number.value();

        let octocrab_pr = self
            .client
            .pulls(owner, repo)
            .get(number.into())
            .await
            .map_err(|e| ApiRetryableError::from_octocrab_error(e))?;

        octocrab_pr.node_id.ok_or_else(|| {
            ApiRetryableError::NonRetryable(format!(
                "Pull request {}/{}/{} has no node_id",
                owner, repo, number
            ))
        })
    }
}
