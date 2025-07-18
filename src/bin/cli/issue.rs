//! Issue-related CLI commands and execution logic
//!
//! This module contains the CLI command definitions and execution logic
//! for issue management operations including creating, commenting, editing,
//! and state management.

use anyhow::Result;
use clap::Subcommand;
use github_edit::github::GitHubClient;
use github_edit::tools::functions::issue;
use github_edit::types::issue::{IssueCommentNumber, IssueNumber, IssueState, IssueUrl};
use github_edit::types::label::Label;
use github_edit::types::repository::{MilestoneNumber, RepositoryId, RepositoryUrl};

#[derive(Subcommand)]
pub enum IssueAction {
    /// Get details for issues by URLs
    ///
    /// Examples:
    ///   github-edit-cli issue get https://github.com/owner/repo/issues/123
    ///   github-edit-cli issue get https://github.com/rust-lang/rust/issues/12345 https://github.com/tokio-rs/tokio/issues/5678
    Get {
        /// Issue URLs to fetch
        ///
        /// Examples:
        ///   https://github.com/owner/repo/issues/123
        ///   https://github.com/rust-lang/rust/issues/98765
        ///   https://github.com/microsoft/vscode/issues/142857
        #[arg(required = true, value_name = "URL")]
        urls: Vec<String>,
    },
    /// Create a new issue
    ///
    /// Examples:
    ///   github-edit-cli issue create -r https://github.com/owner/repo -t "Bug: Application crashes on startup" -b "When I run the app..."
    ///   github-edit-cli issue create --repository-url https://github.com/rust-lang/rust --title "Feature Request: New async trait" --body "It would be great to have..."
    Create {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Issue title (concise description of the issue)
        ///
        /// Examples:
        ///   "Bug: Application crashes on startup"
        ///   "Feature Request: Add dark mode support"
        ///   "Documentation: Missing API examples"
        ///   "Performance: Slow query on large datasets"
        #[arg(short, long, value_name = "TITLE")]
        title: String,
        /// Issue body (detailed description, supports Markdown)
        ///
        /// Examples:
        ///   "When I run the application with..."
        ///   "## Steps to Reproduce\n1. Open the app\n2. Click on..."
        ///   "I would like to propose adding a new feature that..."
        #[arg(short, long, value_name = "BODY")]
        body: Option<String>,
    },
    /// Add a comment to an existing issue
    ///
    /// Examples:
    ///   github-edit-cli issue comment -r https://github.com/owner/repo -i 123 -b "I can confirm this bug"
    ///   github-edit-cli issue comment --repository-url https://github.com/rust-lang/rust --issue 98765 --body "Here's a potential fix..."
    Comment {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Issue number (numeric ID from the URL)
        ///
        /// Examples:
        ///   123 (from https://github.com/owner/repo/issues/123)
        ///   98765 (from https://github.com/rust-lang/rust/issues/98765)
        ///   142857 (from https://github.com/microsoft/vscode/issues/142857)
        #[arg(short, long, value_name = "NUMBER")]
        issue: u32,
        /// Comment body (supports Markdown formatting)
        ///
        /// Examples:
        ///   "I can confirm this bug on macOS 13.2"
        ///   "Here's a potential fix: ```rust\nfn solution() {...}\n```"
        ///   "This is related to issue #456"
        ///   "## Analysis\nAfter investigating, I found that..."
        #[arg(short, long, value_name = "BODY")]
        body: String,
    },
    /// Edit the title of an existing issue
    ///
    /// Examples:
    ///   github-edit-cli issue edit-title -r https://github.com/owner/repo -i 123 -t "Updated: Bug found in authentication module"
    ///   github-edit-cli issue edit-title --repository-url https://github.com/rust-lang/rust --issue 98765 --title "[WIP] Feature: New async trait implementation"
    EditTitle {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Issue number (numeric ID from the URL)
        ///
        /// Examples:
        ///   123 (from https://github.com/owner/repo/issues/123)
        ///   98765 (from https://github.com/rust-lang/rust/issues/98765)
        ///   142857 (from https://github.com/microsoft/vscode/issues/142857)
        #[arg(short, long, value_name = "NUMBER")]
        issue: u32,
        /// New title for the issue
        ///
        /// Examples:
        ///   "Fixed: Bug found in authentication module"
        ///   "[WIP] Feature: New async trait implementation"
        ///   "Documentation: Updated API examples with best practices"
        #[arg(short, long, value_name = "TITLE")]
        title: String,
    },
    /// Edit the body of an existing issue
    ///
    /// Examples:
    ///   github-edit-cli issue edit-body -r https://github.com/owner/repo -i 123 -b "Updated description with more details..."
    ///   github-edit-cli issue edit-body --repository-url https://github.com/rust-lang/rust --issue 98765 --body "## Updated Analysis\nAfter further investigation..."
    EditBody {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Issue number (numeric ID from the URL)
        ///
        /// Examples:
        ///   123 (from https://github.com/owner/repo/issues/123)
        ///   98765 (from https://github.com/rust-lang/rust/issues/98765)
        ///   142857 (from https://github.com/microsoft/vscode/issues/142857)
        #[arg(short, long, value_name = "NUMBER")]
        issue: u32,
        /// New body content (supports Markdown formatting)
        ///
        /// Examples:
        ///   "Updated description with reproduction steps..."
        ///   "## Problem\nThe issue occurs when...\n## Solution\nWe can fix this by..."
        ///   "After further investigation, I found that the root cause is..."
        #[arg(short, long, value_name = "BODY")]
        body: String,
    },
    /// Update the state of an issue (open/closed)
    ///
    /// Examples:
    ///   github-edit-cli issue update-state -r https://github.com/owner/repo -i 123 -s closed
    ///   github-edit-cli issue update-state --repository-url https://github.com/rust-lang/rust --issue 98765 --state open
    UpdateState {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Issue number (numeric ID from the URL)
        ///
        /// Examples:
        ///   123 (from https://github.com/owner/repo/issues/123)
        ///   98765 (from https://github.com/rust-lang/rust/issues/98765)
        ///   142857 (from https://github.com/microsoft/vscode/issues/142857)
        #[arg(short, long, value_name = "NUMBER")]
        issue: u32,
        /// New state for the issue
        ///
        /// Valid values:
        ///   open   - Issue is active and needs attention
        ///   closed - Issue has been resolved or dismissed
        #[arg(short, long, value_name = "STATE")]
        state: IssueState,
    },
    /// Edit an existing comment on an issue
    ///
    /// Examples:
    ///   github-edit-cli issue edit-comment -r https://github.com/owner/repo -i 123 -c 456 -b "Updated comment text"
    ///   github-edit-cli issue edit-comment --repository-url https://github.com/rust-lang/rust --issue 98765 --comment 789 --body "Here's the corrected information..."
    EditComment {
        /// Repository URL (HTTPS format)
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Issue number (numeric ID from the URL)
        #[arg(short, long, value_name = "NUMBER")]
        issue: u32,
        /// Comment number to edit
        #[arg(short, long, value_name = "NUMBER")]
        comment: u32,
        /// New comment body (supports Markdown formatting)
        #[arg(short, long, value_name = "BODY")]
        body: String,
    },
    /// Delete a comment from an issue
    ///
    /// Examples:
    ///   github-edit-cli issue delete-comment -r https://github.com/owner/repo -i 123 -c 456
    ///   github-edit-cli issue delete-comment --repository-url https://github.com/rust-lang/rust --issue 98765 --comment 789
    DeleteComment {
        /// Repository URL (HTTPS format)
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Issue number (numeric ID from the URL)
        #[arg(short, long, value_name = "NUMBER")]
        issue: u32,
        /// Comment number to delete
        #[arg(short, long, value_name = "NUMBER")]
        comment: u32,
    },
    /// Add assignees to an issue
    ///
    /// Examples:
    ///   github-edit-cli issue add-assignees -r https://github.com/owner/repo -i 123 -a user1,user2
    ///   github-edit-cli issue add-assignees --repository-url https://github.com/rust-lang/rust --issue 98765 --assignees john,jane
    AddAssignees {
        /// Repository URL (HTTPS format)
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Issue number (numeric ID from the URL)
        #[arg(short, long, value_name = "NUMBER")]
        issue: u32,
        /// Comma-separated list of assignee usernames to add
        #[arg(short, long, value_name = "USERS")]
        assignees: String,
    },
    /// Remove assignees from an issue
    ///
    /// Examples:
    ///   github-edit-cli issue remove-assignees -r https://github.com/owner/repo -i 123 -a user1,user2
    ///   github-edit-cli issue remove-assignees --repository-url https://github.com/rust-lang/rust --issue 98765 --assignees john,jane
    RemoveAssignees {
        /// Repository URL (HTTPS format)
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Issue number (numeric ID from the URL)
        #[arg(short, long, value_name = "NUMBER")]
        issue: u32,
        /// Comma-separated list of assignee usernames to remove
        #[arg(short, long, value_name = "USERS")]
        assignees: String,
    },
    /// Remove labels from an issue
    ///
    /// Examples:
    ///   github-edit-cli issue remove-labels -r https://github.com/owner/repo -i 123 -l bug,enhancement
    ///   github-edit-cli issue remove-labels --repository-url https://github.com/rust-lang/rust --issue 98765 --labels critical,needs-review
    RemoveLabels {
        /// Repository URL (HTTPS format)
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Issue number (numeric ID from the URL)
        #[arg(short, long, value_name = "NUMBER")]
        issue: u32,
        /// Comma-separated list of label names to remove
        #[arg(short, long, value_name = "LABELS")]
        labels: String,
    },
    /// Delete an issue (requires admin permissions)
    ///
    /// Examples:
    ///   github-edit-cli issue delete -r https://github.com/owner/repo -i 123
    ///   github-edit-cli issue delete --repository-url https://github.com/rust-lang/rust --issue 98765
    Delete {
        /// Repository URL (HTTPS format)
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Issue number (numeric ID from the URL)
        #[arg(short, long, value_name = "NUMBER")]
        issue: u32,
    },
    /// Set milestone for an issue
    ///
    /// Examples:
    ///   github-edit-cli issue set-milestone -r https://github.com/owner/repo -i 123 -m 1
    ///   github-edit-cli issue set-milestone --repository-url https://github.com/rust-lang/rust --issue 98765 --milestone-id 5
    SetMilestone {
        /// Repository URL (HTTPS format)
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Issue number (numeric ID from the URL)
        #[arg(short, long, value_name = "NUMBER")]
        issue: u32,
        /// Milestone ID (numeric ID from the milestone)
        #[arg(short, long, value_name = "MILESTONE_ID")]
        milestone_number: u32,
    },
    /// Remove milestone from an issue
    ///
    /// Examples:
    ///   github-edit-cli issue remove-milestone -r https://github.com/owner/repo -i 123
    ///   github-edit-cli issue remove-milestone --repository-url https://github.com/rust-lang/rust --issue 98765
    RemoveMilestone {
        /// Repository URL (HTTPS format)
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Issue number (numeric ID from the URL)
        #[arg(short, long, value_name = "NUMBER")]
        issue: u32,
    },
}

pub async fn execute_issue_action(github_client: &GitHubClient, action: IssueAction) -> Result<()> {
    match action {
        IssueAction::Get { urls } => {
            let issue_urls: Vec<IssueUrl> = urls.into_iter().map(|url| IssueUrl(url)).collect();
            let result = issue::get_issues_details(github_client, issue_urls).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        IssueAction::Create {
            repository_url,
            title,
            body,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let created_issue = issue::create_issue(
                github_client,
                &repo_id,
                &title,
                body.as_deref(),
                None,
                None,
                None,
            )
            .await?;
            println!("Created issue #{}", created_issue.issue_id.number);
        }
        IssueAction::Comment {
            repository_url,
            issue,
            body,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let issue_number = IssueNumber::new(issue);
            let comment_number =
                issue::add_comment(github_client, &repo_id, issue_number, &body).await?;
            println!("Added comment #{}", comment_number);
        }
        IssueAction::EditTitle {
            repository_url,
            issue,
            title,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let issue_number = IssueNumber::new(issue);
            issue::edit_title(github_client, &repo_id, issue_number, &title).await?;
            println!("Updated issue #{} title", issue);
        }
        IssueAction::EditBody {
            repository_url,
            issue,
            body,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let issue_number = IssueNumber::new(issue);
            issue::edit_body(github_client, &repo_id, issue_number, &body).await?;
            println!("Updated issue #{} body", issue);
        }
        IssueAction::UpdateState {
            repository_url,
            issue,
            state,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let issue_number = IssueNumber::new(issue);
            issue::update_state(github_client, &repo_id, issue_number, state).await?;
            println!("Updated issue #{} state to {}", issue, state);
        }
        IssueAction::EditComment {
            repository_url,
            issue,
            comment,
            body,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let issue_number = IssueNumber::new(issue);
            let comment_number = IssueCommentNumber::new(comment.into());
            issue::edit_comment(github_client, &repo_id, issue_number, comment_number, &body)
                .await?;
            println!("Updated comment #{} on issue #{}", comment, issue);
        }
        IssueAction::DeleteComment {
            repository_url,
            issue,
            comment,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let issue_number = IssueNumber::new(issue);
            let comment_number = IssueCommentNumber::new(comment.into());
            issue::delete_comment(github_client, &repo_id, issue_number, comment_number).await?;
            println!("Deleted comment #{} from issue #{}", comment, issue);
        }
        IssueAction::AddAssignees {
            repository_url,
            issue,
            assignees,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let issue_number = IssueNumber::new(issue);
            let assignee_list: Vec<String> =
                assignees.split(',').map(|s| s.trim().to_string()).collect();
            let (added, skipped) =
                issue::add_assignees(github_client, &repo_id, issue_number, &assignee_list).await?;
            println!("Added assignees: {:?}", added);
            if !skipped.is_empty() {
                println!("Skipped (already assigned): {:?}", skipped);
            }
        }
        IssueAction::RemoveAssignees {
            repository_url,
            issue,
            assignees,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let issue_number = IssueNumber::new(issue);
            let assignee_list: Vec<String> =
                assignees.split(',').map(|s| s.trim().to_string()).collect();
            let (removed, skipped) =
                issue::remove_assignees(github_client, &repo_id, issue_number, &assignee_list)
                    .await?;
            println!("Removed assignees: {:?}", removed);
            if !skipped.is_empty() {
                println!("Skipped (not assigned): {:?}", skipped);
            }
        }
        IssueAction::RemoveLabels {
            repository_url,
            issue,
            labels,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let issue_number = IssueNumber::new(issue);
            let label_list: Vec<Label> = labels
                .split(',')
                .map(|s| Label::from(s.trim().to_string()))
                .collect();
            let (removed, skipped) =
                issue::remove_labels(github_client, &repo_id, issue_number, &label_list).await?;
            println!(
                "Removed labels: {:?}",
                removed.iter().map(|l| &l.name).collect::<Vec<_>>()
            );
            if !skipped.is_empty() {
                println!(
                    "Skipped (not assigned): {:?}",
                    skipped.iter().map(|l| &l.name).collect::<Vec<_>>()
                );
            }
        }
        IssueAction::Delete {
            repository_url,
            issue,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let issue_number = IssueNumber::new(issue);
            issue::delete_issue(github_client, &repo_id, issue_number).await?;
            println!("Deleted issue #{}", issue);
        }
        IssueAction::SetMilestone {
            repository_url,
            issue,
            milestone_number,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let issue_number = IssueNumber::new(issue);
            let milestone_number = MilestoneNumber::new(milestone_number.into());
            issue::set_milestone(github_client, &repo_id, issue_number, milestone_number).await?;
            println!(
                "Set milestone {} for issue #{}",
                milestone_number.value(),
                issue
            );
        }
        IssueAction::RemoveMilestone {
            repository_url,
            issue,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;
            let issue_number = IssueNumber::new(issue);
            issue::remove_milestone(github_client, &repo_id, issue_number).await?;
            println!("Removed milestone from issue #{}", issue);
        }
    }
    Ok(())
}
