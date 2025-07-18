//! GitHub Edit CLI
//!
//! Command-line interface for GitHub repository editing operations.
//! This CLI provides access to issue, pull request, and project management
//! functionality by delegating to the functions in src/tools/functions/.

use anyhow::Result;
use clap::{Parser, Subcommand};
use github_edit::github::GitHubClient;
use std::env;

mod cli;
use cli::{
    IssueAction, ProjectAction, PullRequestAction, RepositoryAction, execute_issue_action,
    execute_pr_action, execute_project_action, execute_repository_action,
};

#[derive(Parser)]
#[command(name = "github-edit-cli")]
#[command(about = "GitHub repository editing operations")]
#[command(
    long_about = r"A command-line interface for GitHub repository editing operations including issues, pull requests, and projects.

SETUP:
Set the GITHUB_EDIT_GITHUB_TOKEN environment variable with your GitHub personal access token:
    export GITHUB_EDIT_GITHUB_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx

EXAMPLES:
    # Get issue details
    github-edit-cli issue get https://github.com/rust-lang/rust/issues/98765

    # Create a new issue
    github-edit-cli issue create -r https://github.com/owner/repo -t 'Bug: App crashes' -b 'Detailed description...'

    # Create a pull request
    github-edit-cli pull-request create -r https://github.com/owner/repo -t 'Fix bug' --head feature-branch --base main

    # Update project field
    github-edit-cli project update-field --project-node-id 'PN_kwDOBw6lbs4AAVGQ' --project-item-id 'PVTI_xxx' --project-field-id 'PVTF_xxx' --field-type text --value 'In Progress'

Use 'github-edit-cli <command> --help' for detailed command-specific help and examples."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Issue management operations (create, comment, edit, update state)
    ///
    /// Examples:
    ///   github-edit-cli issue get https://github.com/owner/repo/issues/123
    ///   github-edit-cli issue create -r https://github.com/owner/repo -t "Bug title" -b "Description"
    ///   github-edit-cli issue comment -r https://github.com/owner/repo -i 123 -b "My comment"
    Issue {
        #[command(subcommand)]
        action: IssueAction,
    },
    /// Pull request management operations (create, comment, edit, close)
    ///
    /// Examples:
    ///   github-edit-cli pull-request get https://github.com/owner/repo/pull/123
    ///   github-edit-cli pull-request create -r https://github.com/owner/repo -t "PR title" --head feature --base main
    ///   github-edit-cli pull-request comment -r https://github.com/owner/repo -p 123 -b "Review comment"
    #[command(name = "pull-request")]
    PullRequest {
        #[command(subcommand)]
        action: PullRequestAction,
    },
    /// Project management operations (update custom fields)
    ///
    /// Examples:
    ///   github-edit-cli project update-field --project-node-id "PN_xxx" --project-item-id "PVTI_xxx" --project-field-id "PVTF_xxx" --field-type text --value "In Progress"
    Project {
        #[command(subcommand)]
        action: ProjectAction,
    },
    /// Repository management operations (create milestones)
    ///
    /// Examples:
    ///   github-edit-cli repository create-milestone -r https://github.com/owner/repo -t "v1.0.0" -d "Initial release"
    Repository {
        #[command(subcommand)]
        action: RepositoryAction,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get GitHub token from environment
    let github_token = env::var("GITHUB_EDIT_GITHUB_TOKEN").map_err(|_| {
        anyhow::anyhow!("GITHUB_EDIT_GITHUB_TOKEN environment variable is required")
    })?;

    // Create GitHub client
    let github_client = GitHubClient::new(Some(github_token), None)?;

    // Parse CLI arguments
    let cli = Cli::parse();

    // Execute command
    match cli.command {
        Commands::Issue { action } => execute_issue_action(&github_client, action).await,
        Commands::PullRequest { action } => execute_pr_action(&github_client, action).await,
        Commands::Project { action } => execute_project_action(&github_client, action).await,
        Commands::Repository { action } => execute_repository_action(&github_client, action).await,
    }
}
