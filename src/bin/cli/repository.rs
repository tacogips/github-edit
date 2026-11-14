//! Repository-related CLI commands and execution logic
//!
//! This module contains the CLI command definitions and execution logic
//! for repository milestone and label management operations.

use anyhow::Result;
use chrono::{DateTime, Utc};
use clap::Subcommand;
use github_edit::github::GitHubClient;
use github_edit::tools::functions::repository;
use github_edit::types::milestone::MilestoneState;
use github_edit::types::repository::{MilestoneNumber, RepositoryId, RepositoryUrl};

#[derive(Subcommand)]
pub enum RepositoryAction {
    /// Create a new milestone in a repository
    ///
    /// Examples:
    ///   github-edit-cli repository create-milestone -r https://github.com/owner/repo -t "v1.0.0" -d "Initial release"
    ///   github-edit-cli repository create-milestone --repository-url https://github.com/rust-lang/rust --title "Sprint 1" --description "First sprint tasks"
    CreateMilestone {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Milestone title
        ///
        /// Examples:
        ///   "v1.0.0"
        ///   "Sprint 1"
        ///   "Release 2024.1"
        ///   "Bug fixes for Q1"
        #[arg(short, long, value_name = "TITLE")]
        title: String,
        /// Milestone description (optional)
        ///
        /// Examples:
        ///   "Initial release with core features"
        ///   "First sprint focusing on authentication"
        ///   "Critical bug fixes for the first quarter"
        #[arg(short, long, value_name = "DESCRIPTION")]
        description: Option<String>,
        /// Due date for the milestone (optional, ISO 8601 format)
        ///
        /// Examples:
        ///   "2024-12-31T23:59:59Z"
        ///   "2024-06-15T00:00:00Z"
        ///   "2024-03-01T09:00:00Z"
        #[arg(long, value_name = "DUE_DATE")]
        due_on: Option<String>,
        /// Milestone state (optional, defaults to open)
        ///
        /// Valid values:
        ///   open   - Milestone is active and accepting issues
        ///   closed - Milestone is completed or closed
        #[arg(short, long, value_name = "STATE")]
        state: Option<MilestoneState>,
    },
    /// Update an existing milestone in a repository
    ///
    /// Examples:
    ///   github-edit-cli repository update-milestone -r https://github.com/owner/repo -m 1 -t "v1.0.1" -d "Updated release"
    ///   github-edit-cli repository update-milestone --repository-url https://github.com/rust-lang/rust --milestone-id 5 --title "Sprint 2"
    UpdateMilestone {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Milestone ID to update
        ///
        /// Examples:
        ///   1
        ///   5
        ///   42
        #[arg(short, long, value_name = "ID")]
        milestone_number: u32,
        /// New milestone title (optional)
        ///
        /// Examples:
        ///   "v1.0.1"
        ///   "Sprint 2"
        ///   "Release 2024.2"
        ///   "Updated bug fixes for Q1"
        #[arg(short, long, value_name = "TITLE")]
        title: Option<String>,
        /// New milestone description (optional)
        ///
        /// Examples:
        ///   "Updated release with bug fixes"
        ///   "Second sprint focusing on performance"
        ///   "Critical bug fixes and improvements"
        #[arg(short, long, value_name = "DESCRIPTION")]
        description: Option<String>,
        /// New due date for the milestone (optional, ISO 8601 format)
        ///
        /// Examples:
        ///   "2024-12-31T23:59:59Z"
        ///   "2024-06-15T00:00:00Z"
        ///   "2024-03-01T09:00:00Z"
        #[arg(long, value_name = "DUE_DATE")]
        due_on: Option<String>,
        /// New milestone state (optional)
        ///
        /// Valid values:
        ///   open   - Milestone is active and accepting issues
        ///   closed - Milestone is completed or closed
        #[arg(short, long, value_name = "STATE")]
        state: Option<MilestoneState>,
    },
    /// Delete an existing milestone from a repository
    ///
    /// Examples:
    ///   github-edit-cli repository delete-milestone -r https://github.com/owner/repo -m 1
    ///   github-edit-cli repository delete-milestone --repository-url https://github.com/rust-lang/rust --milestone-id 5
    DeleteMilestone {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Milestone ID to delete
        ///
        /// Examples:
        ///   1
        ///   5
        ///   42
        #[arg(short, long, value_name = "ID")]
        milestone_number: u32,
    },
    /// Create a new label in a repository
    ///
    /// Examples:
    ///   github-edit-cli repository create-label -r https://github.com/owner/repo -n "bug" -c "ff0000" -d "Something isn't working"
    ///   github-edit-cli repository create-label --repository-url https://github.com/rust-lang/rust --name "enhancement" --color "00ff00"
    CreateLabel {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Label name
        ///
        /// Examples:
        ///   "bug"
        ///   "enhancement"
        ///   "good first issue"
        ///   "help wanted"
        #[arg(short, long, value_name = "NAME")]
        name: String,
        /// Label color (optional, defaults to ffffff)
        ///
        /// Examples:
        ///   "ff0000" (red)
        ///   "00ff00" (green)
        ///   "0000ff" (blue)
        ///   "ffff00" (yellow)
        #[arg(short, long, value_name = "COLOR")]
        color: Option<String>,
        /// Label description (optional)
        ///
        /// Examples:
        ///   "Something isn't working"
        ///   "New feature or request"
        ///   "Good for newcomers"
        ///   "Extra attention is needed"
        #[arg(short, long, value_name = "DESCRIPTION")]
        description: Option<String>,
    },
    /// Update an existing label in a repository
    ///
    /// Examples:
    ///   github-edit-cli repository update-label -r https://github.com/owner/repo -o "bug" -n "critical-bug" -c "ff0000"
    ///   github-edit-cli repository update-label --repository-url https://github.com/rust-lang/rust --old-name "enhancement" --new-name "feature"
    UpdateLabel {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Current label name
        ///
        /// Examples:
        ///   "bug"
        ///   "enhancement"
        ///   "good first issue"
        #[arg(short, long, value_name = "OLD_NAME")]
        old_name: String,
        /// New label name (optional)
        ///
        /// Examples:
        ///   "critical-bug"
        ///   "feature"
        ///   "beginner-friendly"
        #[arg(short, long, value_name = "NEW_NAME")]
        new_name: Option<String>,
        /// New label color (optional)
        ///
        /// Examples:
        ///   "ff0000" (red)
        ///   "00ff00" (green)
        ///   "0000ff" (blue)
        ///   "ffff00" (yellow)
        #[arg(short, long, value_name = "COLOR")]
        color: Option<String>,
        /// New label description (optional)
        ///
        /// Examples:
        ///   "Critical issue requiring immediate attention"
        ///   "New feature or enhancement request"
        ///   "Good for newcomers to the project"
        #[arg(short, long, value_name = "DESCRIPTION")]
        description: Option<String>,
    },
    /// Delete an existing label from a repository
    ///
    /// Examples:
    ///   github-edit-cli repository delete-label -r https://github.com/owner/repo -n "bug"
    ///   github-edit-cli repository delete-label --repository-url https://github.com/rust-lang/rust --name "enhancement"
    DeleteLabel {
        /// Repository URL (HTTPS format)
        ///
        /// Examples:
        ///   https://github.com/owner/repo
        ///   https://github.com/rust-lang/rust
        ///   https://github.com/microsoft/vscode
        #[arg(short, long, value_name = "URL")]
        repository_url: String,
        /// Label name to delete
        ///
        /// Examples:
        ///   "bug"
        ///   "enhancement"
        ///   "good first issue"
        ///   "help wanted"
        #[arg(short, long, value_name = "NAME")]
        name: String,
    },
}

pub async fn execute_repository_action(
    github_client: &GitHubClient,
    action: RepositoryAction,
) -> Result<()> {
    match action {
        RepositoryAction::CreateMilestone {
            repository_url,
            title,
            description,
            due_on,
            state,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;

            let due_date = if let Some(due_str) = due_on {
                Some(
                    due_str
                        .parse::<DateTime<Utc>>()
                        .map_err(|e| anyhow::anyhow!("Failed to parse due date: {}", e))?,
                )
            } else {
                None
            };

            let created_milestone = repository::create_milestone(
                github_client,
                &repo_id,
                &title,
                description.as_deref(),
                due_date,
                state,
            )
            .await?;

            println!(
                "Created milestone #{} - {}",
                created_milestone.id.value(),
                created_milestone.title
            );
        }
        RepositoryAction::UpdateMilestone {
            repository_url,
            milestone_number,
            title,
            description,
            due_on,
            state,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;

            let milestone_number = MilestoneNumber::new(milestone_number.into());

            let due_date = if let Some(due_str) = due_on {
                Some(
                    due_str
                        .parse::<DateTime<Utc>>()
                        .map_err(|e| anyhow::anyhow!("Failed to parse due date: {}", e))?,
                )
            } else {
                None
            };

            let updated_milestone = repository::update_milestone(
                github_client,
                &repo_id,
                &milestone_number,
                title.as_deref(),
                description.as_deref(),
                due_date,
                state,
            )
            .await?;

            println!(
                "Updated milestone #{} - {}",
                updated_milestone.id.value(),
                updated_milestone.title
            );
        }
        RepositoryAction::DeleteMilestone {
            repository_url,
            milestone_number,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;

            let milestone_number = MilestoneNumber::new(milestone_number.into());

            repository::delete_milestone(github_client, &repo_id, &milestone_number).await?;

            println!("Deleted milestone #{}", milestone_number.value());
        }
        RepositoryAction::CreateLabel {
            repository_url,
            name,
            color,
            description,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;

            let created_label = repository::create_label(
                github_client,
                &repo_id,
                &name,
                color.as_deref(),
                description.as_deref(),
            )
            .await?;

            println!(
                "Created label '{}' with color #{}",
                created_label.name,
                created_label.color()
            );
        }
        RepositoryAction::UpdateLabel {
            repository_url,
            old_name,
            new_name,
            color,
            description,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;

            let updated_label = repository::update_label(
                github_client,
                &repo_id,
                &old_name,
                new_name.as_deref(),
                color.as_deref(),
                description.as_deref(),
            )
            .await?;

            println!(
                "Updated label '{}' with color #{}",
                updated_label.name,
                updated_label.color()
            );
        }
        RepositoryAction::DeleteLabel {
            repository_url,
            name,
        } => {
            let repo_url = RepositoryUrl::new(repository_url);
            let repo_id = RepositoryId::parse_url(&repo_url)
                .map_err(|e| anyhow::anyhow!("Failed to parse repository URL: {}", e))?;

            repository::delete_label(github_client, &repo_id, &name).await?;

            println!("Deleted label '{}'", name);
        }
    }
    Ok(())
}
