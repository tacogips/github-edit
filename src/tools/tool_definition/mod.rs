//! Tool definition modules for GitHub repository operations
//!
//! This module contains the separated tool definitions organized by functionality:
//! - `issue`: Issue management tools
//! - `project`: Project management tools  
//! - `pull_request`: Pull request management tools
//!
//! The GitEditTools implementation is now split across multiple files conceptually,
//! but the actual tool implementations are consolidated in the main mod.rs file
//! to satisfy the #[tool(tool_box)] macro requirements.

pub mod issue;
pub mod project;
pub mod pull_request;
pub mod repository;

pub use issue::IssueTools;
pub use project::ProjectTools;
pub use pull_request::PullRequestTools;
pub use repository::RepositoryTools;
