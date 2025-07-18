//! CLI module for GitHub Edit
//!
//! This module contains the command-line interface definitions and execution logic
//! organized by resource type (issues, pull requests, projects).

pub mod issue;
pub mod project;
pub mod pull_request;
pub mod repository;

pub use issue::{IssueAction, execute_issue_action};
pub use project::{ProjectAction, execute_project_action};
pub use pull_request::{PullRequestAction, execute_pr_action};
pub use repository::{RepositoryAction, execute_repository_action};
