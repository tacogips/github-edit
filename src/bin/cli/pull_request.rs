//! Pull request-related CLI commands and execution logic
//!
//! This module contains the CLI command definitions and execution logic
//! for pull request management operations including creating, commenting,
//! editing, and managing assignees, reviewers, labels, and milestones.

use anyhow::Result;
use clap::Subcommand;
use github_edit::github::GitHubClient;
use github_edit::tools::functions::pull_request;
use github_edit::types::label::Label;
use github_edit::types::pull_request::{Branch, PullRequestCommentNumber, PullRequestNumber};
use github_edit::types::repository::{MilestoneNumber, RepositoryId, RepositoryUrl};

#[derive(Subcommand)]
pub enum PullRequestAction {
    /// Get details for pull requests by URLs
    ///
    /// Examples:
    ///   github-edit-cli pull-request get https://github.com/owner/repo/pull/123
    ///   github-edit-cli pull-request get https://github.com/rust-lang/rust/pull/98765 https://github.com/tokio-rs/tokio/pull/5432
    Get {
        /// Pull request URLs to fetch
        ///
        /// Examples:
        ///   https://github.com/owner/repo/pull/123
        ///   https://github.com/rust-lang/rust/pull/98765
        ///   https://github.com/microsoft/vscode/pull/142857
        #[arg(required = true, value_name = "URL")]
        urls: Vec<String>,
    },
    /// Create a new pull request
    ///
    /// Examples:
    ///   github-edit-cli pull-request create -r https://github.com/owner/repo -t "Fix authentication bug" --head feature-auth-fix --base main
    ///   github-edit-cli pull-request create --repository-url https://github.com/rust-lang/rust --title "Add async support to trait" --head async-trait --base master --body "This PR adds..." --draft
    Create {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Pull request title (concise description of changes)
        ///
        /// Examples:
        ///   "Fix authentication bug in login module"
        ///   "Add async support to HTTP client trait"
        ///   "Update documentation with new API examples"
        ///   "Performance: Optimize database query execution"
        #[arg(short, long, value_name = "TITLE")]
        title: String,
        /// Head branch (source branch with your changes)
        ///
        /// Examples:
        ///   feature-auth-fix
        ///   async-trait-support
        ///   bugfix/memory-leak
        ///   improvement/better-logging
        ///   username:feature-branch (for forks)
        #[arg(long, value_name = "BRANCH")]
        head: String,
        /// Base branch (target branch to merge into)
        ///
        /// Examples:
        ///   main
        ///   master
        ///   develop
        ///   v2.0
        ///   release/1.5
        #[arg(long, value_name = "BRANCH")]
        base: String,
        /// Pull request body (detailed description, supports Markdown)
        ///
        /// Examples:
        ///   "This PR fixes the authentication bug by..."
        ///   "## Changes\n- Added async support\n- Updated tests\n## Breaking Changes\nNone"
        ///   "Closes #123\n\nThis implementation..."
        #[arg(short, long, value_name = "BODY")]
        body: Option<String>,
        /// Create as draft pull request (not ready for review)
        ///
        /// Use this flag when:
        ///   - Work is still in progress
        ///   - You want early feedback
        ///   - Testing is not complete
        #[arg(long)]
        draft: bool,
    },
    /// Add a comment to an existing pull request
    ///
    /// Examples:
    ///   github-edit-cli pull-request comment -r https://github.com/owner/repo -p 123 -b "LGTM! Great work on this fix."
    ///   github-edit-cli pull-request comment --repository-url https://github.com/rust-lang/rust --pr 98765 --body "Could you add a test for the edge case?"
    Comment {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Pull request number (numeric ID from the URL)
        ///
        /// Examples:
        ///   123 (from https://github.com/owner/repo/pull/123)
        ///   98765 (from https://github.com/rust-lang/rust/pull/98765)
        ///   142857 (from https://github.com/microsoft/vscode/pull/142857)
        #[arg(short = 'p', long, value_name = "NUMBER")]
        pull_request_number: u32,
        /// Comment body (supports Markdown formatting)
        ///
        /// Examples:
        ///   "LGTM! Great work on this implementation."
        ///   "Could you add a test for the edge case when input is null?"
        ///   "## Review Comments\n- Line 45: Consider using a more descriptive variable name"
        ///   "This looks good but please rebase on latest main"
        #[arg(short, long, value_name = "BODY")]
        body: String,
    },
    /// Close a pull request without merging
    ///
    /// Examples:
    ///   github-edit-cli pull-request close -r https://github.com/owner/repo -p 123
    ///   github-edit-cli pull-request close --repository-url https://github.com/rust-lang/rust --pr 98765
    Close {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Pull request number (numeric ID from the URL)
        ///
        /// Examples:
        ///   123 (from https://github.com/owner/repo/pull/123)
        ///   98765 (from https://github.com/rust-lang/rust/pull/98765)
        ///   142857 (from https://github.com/microsoft/vscode/pull/142857)
        #[arg(short = 'p', long, value_name = "NUMBER")]
        pull_request_number: u32,
    },
    /// Edit the title of an existing pull request
    ///
    /// Examples:
    ///   github-edit-cli pull-request edit-title -r https://github.com/owner/repo -p 123 -t "Updated: Fix authentication bug with OAuth flow"
    ///   github-edit-cli pull-request edit-title --repository-url https://github.com/rust-lang/rust --pr 98765 --title "[WIP] Feature: New async trait implementation"
    EditTitle {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Pull request number (numeric ID from the URL)
        ///
        /// Examples:
        ///   123 (from https://github.com/owner/repo/pull/123)
        ///   98765 (from https://github.com/rust-lang/rust/pull/98765)
        ///   142857 (from https://github.com/microsoft/vscode/pull/142857)
        #[arg(short = 'p', long, value_name = "NUMBER")]
        pull_request_number: u32,
        /// New title for the pull request
        ///
        /// Examples:
        ///   "Updated: Fix authentication bug with OAuth flow"
        ///   "[WIP] Feature: New async trait implementation"
        ///   "Performance: Optimize database query execution"
        #[arg(short, long, value_name = "TITLE")]
        title: String,
    },
    /// Edit the body of an existing pull request
    ///
    /// Examples:
    ///   github-edit-cli pull-request edit-body -r https://github.com/owner/repo -p 123 -b "Updated description with benchmark results..."
    ///   github-edit-cli pull-request edit-body --repository-url https://github.com/rust-lang/rust --pr 98765 --body "## Updated Implementation\nAfter review feedback..."
    EditBody {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Pull request number (numeric ID from the URL)
        ///
        /// Examples:
        ///   123 (from https://github.com/owner/repo/pull/123)
        ///   98765 (from https://github.com/rust-lang/rust/pull/98765)
        ///   142857 (from https://github.com/microsoft/vscode/pull/142857)
        #[arg(short = 'p', long, value_name = "NUMBER")]
        pull_request_number: u32,
        /// New body content (supports Markdown formatting)
        ///
        /// Examples:
        ///   "Updated implementation with performance improvements..."
        ///   "## Changes\n- Fixed memory leak\n- Added benchmarks\n## Results\nPerformance improved by 40%"
        ///   "After review feedback, I've updated the approach to..."
        #[arg(short, long, value_name = "BODY")]
        body: String,
    },
    /// Edit an existing pull request comment
    ///
    /// Examples:
    ///   github-edit-cli pull-request edit-comment -r https://github.com/owner/repo -p 123 -c 456 -b "Updated comment with clarification..."
    ///   github-edit-cli pull-request edit-comment --repository-url https://github.com/rust-lang/rust --pr 98765 --comment 789 --body "After thinking more about this..."
    EditComment {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Pull request number (numeric ID from the URL)
        ///
        /// Examples:
        ///   123 (from https://github.com/owner/repo/pull/123)
        ///   98765 (from https://github.com/rust-lang/rust/pull/98765)
        ///   142857 (from https://github.com/microsoft/vscode/pull/142857)
        #[arg(short = 'p', long, value_name = "NUMBER")]
        pull_request_number: u32,
        /// Comment number (numeric ID of the comment)
        ///
        /// Examples:
        ///   456 (comment ID within the pull request)
        ///   789 (another comment ID)
        #[arg(short = 'c', long, value_name = "NUMBER")]
        comment_number: u32,
        /// New comment body (supports Markdown formatting)
        ///
        /// Examples:
        ///   "Updated comment with clarification..."
        ///   "After thinking more about this approach..."
        ///   "## Revised Analysis\nI've reconsidered the implementation..."
        #[arg(short, long, value_name = "BODY")]
        body: String,
    },
    /// Delete a pull request comment
    ///
    /// Examples:
    ///   github-edit-cli pull-request delete-comment -r https://github.com/owner/repo -p 123 -c 456
    ///   github-edit-cli pull-request delete-comment --repository-url https://github.com/rust-lang/rust --pr 98765 --comment 789
    DeleteComment {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Pull request number (numeric ID from the URL)
        ///
        /// Examples:
        ///   123 (from https://github.com/owner/repo/pull/123)
        ///   98765 (from https://github.com/rust-lang/rust/pull/98765)
        ///   142857 (from https://github.com/microsoft/vscode/pull/142857)
        #[arg(short = 'p', long, value_name = "NUMBER")]
        pull_request_number: u32,
        /// Comment number (numeric ID of the comment to delete)
        ///
        /// Examples:
        ///   456 (comment ID within the pull request)
        ///   789 (another comment ID)
        #[arg(short = 'c', long, value_name = "NUMBER")]
        comment_number: u32,
    },
    /// Add assignees to a pull request
    ///
    /// Examples:
    ///   github-edit-cli pull-request add-assignees -r https://github.com/owner/repo -p 123 -a "user1,user2"
    ///   github-edit-cli pull-request add-assignees --repository-url https://github.com/rust-lang/rust --pr 98765 --assignees "maintainer1,maintainer2"
    AddAssignees {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Pull request number (numeric ID from the URL)
        ///
        /// Examples:
        ///   123 (from https://github.com/owner/repo/pull/123)
        ///   98765 (from https://github.com/rust-lang/rust/pull/98765)
        ///   142857 (from https://github.com/microsoft/vscode/pull/142857)
        #[arg(short = 'p', long, value_name = "NUMBER")]
        pull_request_number: u32,
        /// Comma-separated list of assignee usernames to add
        ///
        /// Examples:
        ///   "user1,user2,user3"
        ///   "maintainer1,reviewer2"
        ///   "singleuser"
        #[arg(short = 'a', long, value_name = "USERNAMES")]
        assignees: String,
    },
    /// Remove assignees from a pull request
    ///
    /// Examples:
    ///   github-edit-cli pull-request remove-assignees -r https://github.com/owner/repo -p 123 -a "user1,user2"
    ///   github-edit-cli pull-request remove-assignees --repository-url https://github.com/rust-lang/rust --pr 98765 --assignees "maintainer1,maintainer2"
    RemoveAssignees {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Pull request number (numeric ID from the URL)
        ///
        /// Examples:
        ///   123 (from https://github.com/owner/repo/pull/123)
        ///   98765 (from https://github.com/rust-lang/rust/pull/98765)
        ///   142857 (from https://github.com/microsoft/vscode/pull/142857)
        #[arg(short = 'p', long, value_name = "NUMBER")]
        pull_request_number: u32,
        /// Comma-separated list of assignee usernames to remove
        ///
        /// Examples:
        ///   "user1,user2,user3"
        ///   "maintainer1,reviewer2"
        ///   "singleuser"
        #[arg(short = 'a', long, value_name = "USERNAMES")]
        assignees: String,
    },
    /// Add requested reviewers to a pull request
    ///
    /// Examples:
    ///   github-edit-cli pull-request add-reviewers -r https://github.com/owner/repo -p 123 -u "reviewer1,reviewer2"
    ///   github-edit-cli pull-request add-reviewers --repository-url https://github.com/rust-lang/rust --pr 98765 --reviewers "expert1,expert2"
    AddReviewers {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Pull request number (numeric ID from the URL)
        ///
        /// Examples:
        ///   123 (from https://github.com/owner/repo/pull/123)
        ///   98765 (from https://github.com/rust-lang/rust/pull/98765)
        ///   142857 (from https://github.com/microsoft/vscode/pull/142857)
        #[arg(short = 'p', long, value_name = "NUMBER")]
        pull_request_number: u32,
        /// Comma-separated list of reviewer usernames
        ///
        /// Examples:
        ///   "reviewer1,reviewer2,reviewer3"
        ///   "expert1,expert2"
        ///   "singlereviewer"
        #[arg(short = 'u', long, value_name = "USERNAMES")]
        reviewers: String,
    },
    /// Add labels to a pull request
    ///
    /// Examples:
    ///   github-edit-cli pull-request add-labels -r https://github.com/owner/repo -p 123 -l "bug,critical"
    ///   github-edit-cli pull-request add-labels --repository-url https://github.com/rust-lang/rust --pr 98765 --labels "enhancement,performance"
    AddLabels {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Pull request number (numeric ID from the URL)
        ///
        /// Examples:
        ///   123 (from https://github.com/owner/repo/pull/123)
        ///   98765 (from https://github.com/rust-lang/rust/pull/98765)
        ///   142857 (from https://github.com/microsoft/vscode/pull/142857)
        #[arg(short = 'p', long, value_name = "NUMBER")]
        pull_request_number: u32,
        /// Comma-separated list of label names
        ///
        /// Examples:
        ///   "bug,critical,high-priority"
        ///   "enhancement,performance"
        ///   "documentation"
        #[arg(short = 'l', long, value_name = "LABELS")]
        labels: String,
    },
    /// Remove labels from a pull request
    ///
    /// Examples:
    ///   github-edit-cli pull-request remove-labels -r https://github.com/owner/repo -p 123 -l "bug,critical"
    ///   github-edit-cli pull-request remove-labels --repository-url https://github.com/rust-lang/rust --pr 98765 --labels "enhancement,performance"
    RemoveLabels {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Pull request number (numeric ID from the URL)
        ///
        /// Examples:
        ///   123 (from https://github.com/owner/repo/pull/123)
        ///   98765 (from https://github.com/rust-lang/rust/pull/98765)
        ///   142857 (from https://github.com/microsoft/vscode/pull/142857)
        #[arg(short = 'p', long, value_name = "NUMBER")]
        pull_request_number: u32,
        /// Comma-separated list of label names to remove
        ///
        /// Examples:
        ///   "bug,critical,high-priority"
        ///   "enhancement,performance"
        ///   "documentation"
        #[arg(short = 'l', long, value_name = "LABELS")]
        labels: String,
    },
    /// Add milestone to a pull request
    ///
    /// Examples:
    ///   github-edit-cli pull-request add-milestone -r https://github.com/owner/repo -p 123 -m 5
    ///   github-edit-cli pull-request add-milestone --repository-url https://github.com/rust-lang/rust --pr 98765 --milestone 10
    AddMilestone {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Pull request number (numeric ID from the URL)
        ///
        /// Examples:
        ///   123 (from https://github.com/owner/repo/pull/123)
        ///   98765 (from https://github.com/rust-lang/rust/pull/98765)
        ///   142857 (from https://github.com/microsoft/vscode/pull/142857)
        #[arg(short = 'p', long, value_name = "NUMBER")]
        pull_request_number: u32,
        /// Milestone ID (numeric ID of the milestone)
        ///
        /// Examples:
        ///   5 (milestone ID)
        ///   10 (another milestone ID)
        ///   15 (yet another milestone ID)
        #[arg(short = 'm', long, value_name = "ID")]
        milestone: u64,
    },
    /// Remove milestone from a pull request
    ///
    /// Examples:
    ///   github-edit-cli pull-request remove-milestone -r https://github.com/owner/repo -p 123
    ///   github-edit-cli pull-request remove-milestone --repository-url https://github.com/rust-lang/rust --pr 98765
    RemoveMilestone {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Pull request number (numeric ID from the URL)
        ///
        /// Examples:
        ///   123 (from https://github.com/owner/repo/pull/123)
        ///   98765 (from https://github.com/rust-lang/rust/pull/98765)
        ///   142857 (from https://github.com/microsoft/vscode/pull/142857)
        #[arg(short = 'p', long, value_name = "NUMBER")]
        pull_request_number: u32,
    },
}

pub async fn execute_pr_action(
    github_client: &GitHubClient,
    action: PullRequestAction,
) -> Result<()> {
    match action {
        PullRequestAction::Get { urls: _ } => {
            eprintln!("Get pull request details functionality has been removed");
            return Err(anyhow::anyhow!(
                "Get pull request details functionality has been removed"
            ));
        }
        PullRequestAction::Create {
            repository_url,
            title,
            head,
            base,
            body,
            draft,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let head_branch = Branch::new(head);
            let base_branch = Branch::new(base);
            let created_pr = pull_request::create_pull_request(
                github_client,
                &repo_id,
                &title,
                &head_branch,
                &base_branch,
                body.as_deref(),
                Some(draft),
            )
            .await?;
            println!(
                "Created pull request #{}",
                created_pr.pull_request_id.number
            );
        }
        PullRequestAction::Comment {
            repository_url,
            pull_request_number,
            body,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let pr_number = PullRequestNumber::new(pull_request_number);
            let comment_number =
                pull_request::add_comment(github_client, &repo_id, pr_number, &body).await?;
            println!("Added comment #{}", comment_number);
        }
        PullRequestAction::Close {
            repository_url,
            pull_request_number,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let pr_number = PullRequestNumber::new(pull_request_number);
            pull_request::close_pull_request(github_client, &repo_id, pr_number).await?;
            println!("Closed pull request #{}", pull_request_number);
        }
        PullRequestAction::EditTitle {
            repository_url,
            pull_request_number,
            title,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let pr_number = PullRequestNumber::new(pull_request_number);
            pull_request::edit_title(github_client, &repo_id, pr_number, &title).await?;
            println!("Updated pull request #{} title", pull_request_number);
        }
        PullRequestAction::EditBody {
            repository_url,
            pull_request_number,
            body,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let pr_number = PullRequestNumber::new(pull_request_number);
            pull_request::edit_body(github_client, &repo_id, pr_number, &body).await?;
            println!("Updated pull request #{} body", pull_request_number);
        }
        PullRequestAction::EditComment {
            repository_url,
            pull_request_number,
            comment_number,
            body,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let pr_number = PullRequestNumber::new(pull_request_number);
            let comment_num = PullRequestCommentNumber::new(comment_number.into());
            pull_request::edit_comment(github_client, &repo_id, pr_number, comment_num, &body)
                .await?;
            println!(
                "Updated pull request #{} comment #{}",
                pull_request_number, comment_number
            );
        }
        PullRequestAction::DeleteComment {
            repository_url,
            pull_request_number,
            comment_number,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let pr_number = PullRequestNumber::new(pull_request_number);
            let comment_num = PullRequestCommentNumber::new(comment_number.into());
            pull_request::delete_comment(github_client, &repo_id, pr_number, comment_num).await?;
            println!(
                "Deleted pull request #{} comment #{}",
                pull_request_number, comment_number
            );
        }
        PullRequestAction::AddAssignees {
            repository_url,
            pull_request_number,
            assignees,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let pr_number = PullRequestNumber::new(pull_request_number);
            let assignee_list: Vec<String> =
                assignees.split(',').map(|s| s.trim().to_string()).collect();
            let (added, skipped) =
                pull_request::add_assignees(github_client, &repo_id, pr_number, &assignee_list)
                    .await?;
            println!(
                "Added {} assignees to pull request #{}",
                added.len(),
                pull_request_number
            );
            if !skipped.is_empty() {
                println!(
                    "Skipped {} assignees (already assigned): {}",
                    skipped.len(),
                    skipped.join(", ")
                );
            }
        }
        PullRequestAction::RemoveAssignees {
            repository_url,
            pull_request_number,
            assignees,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let pr_number = PullRequestNumber::new(pull_request_number);
            let assignee_list: Vec<String> =
                assignees.split(',').map(|s| s.trim().to_string()).collect();
            pull_request::remove_assignees(github_client, &repo_id, pr_number, &assignee_list)
                .await?;
            println!(
                "Removed assignees from pull request #{}",
                pull_request_number
            );
        }
        PullRequestAction::AddReviewers {
            repository_url,
            pull_request_number,
            reviewers,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let pr_number = PullRequestNumber::new(pull_request_number);
            let reviewer_list: Vec<String> =
                reviewers.split(',').map(|s| s.trim().to_string()).collect();
            let (added, skipped) = pull_request::add_requested_reviewers(
                github_client,
                &repo_id,
                pr_number,
                &reviewer_list,
            )
            .await?;
            println!(
                "Added {} reviewers to pull request #{}",
                added.len(),
                pull_request_number
            );
            if !skipped.is_empty() {
                println!(
                    "Skipped {} reviewers (already requested): {}",
                    skipped.len(),
                    skipped.join(", ")
                );
            }
        }
        PullRequestAction::AddLabels {
            repository_url,
            pull_request_number,
            labels,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let pr_number = PullRequestNumber::new(pull_request_number);
            let label_list: Vec<Label> = labels
                .split(',')
                .map(|s| Label::from(s.trim().to_string()))
                .collect();
            pull_request::add_labels(github_client, &repo_id, pr_number, &label_list).await?;
            println!("Added labels to pull request #{}", pull_request_number);
        }
        PullRequestAction::RemoveLabels {
            repository_url,
            pull_request_number,
            labels,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let pr_number = PullRequestNumber::new(pull_request_number);
            let label_list: Vec<Label> = labels
                .split(',')
                .map(|s| Label::from(s.trim().to_string()))
                .collect();
            pull_request::remove_labels(github_client, &repo_id, pr_number, &label_list).await?;
            println!("Removed labels from pull request #{}", pull_request_number);
        }
        PullRequestAction::AddMilestone {
            repository_url,
            pull_request_number,
            milestone,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let pr_number = PullRequestNumber::new(pull_request_number);
            let milestone_number = MilestoneNumber::new(milestone);
            pull_request::add_milestone(github_client, &repo_id, pr_number, milestone_number)
                .await?;
            println!(
                "Added milestone {} to pull request #{}",
                milestone, pull_request_number
            );
        }
        PullRequestAction::RemoveMilestone {
            repository_url,
            pull_request_number,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let pr_number = PullRequestNumber::new(pull_request_number);
            pull_request::remove_milestone(github_client, &repo_id, pr_number).await?;
            println!(
                "Removed milestone from pull request #{}",
                pull_request_number
            );
        }
    }
    Ok(())
}
