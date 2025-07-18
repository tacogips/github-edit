//! MCP tool implementations for GitHub repository exploration
//!
//! This module provides MCP (Model Context Protocol) tool implementations
//! for exploring GitHub repositories, issues, pull requests, and projects.
//!
//! # Key Features
//!
//! - Search across multiple repositories with advanced filtering
//! - Get detailed repository and project information
//! - Find related resources through cross-references and semantic similarity
//! - Support for multiple filtering options and hybrid search

pub mod tool_definition;
use crate::github::GitHubClient;
use crate::types::issue::{IssueCommentNumber, IssueNumber};
use crate::types::pull_request::PullRequestCommentNumber;

use rmcp::{Error as McpError, ServerHandler, model::*, tool};

pub mod error;
pub mod functions;

/// The main MCP tools service for GitHub repository exploration
#[derive(Clone)]
pub struct GitEditTools {
    github_client: GitHubClient,
}

impl GitEditTools {
    /// Create a new GitInsightTools instance
    pub fn new(github_client: GitHubClient) -> Self {
        Self { github_client }
    }

    /// Initializes the GitInsightTools instance
    pub async fn init(&self) -> Result<(), anyhow::Error> {
        // Basic initialization without services
        Ok(())
    }
}

// Tool implementations are now split across multiple files in tool_definition/

#[tool(tool_box)]
impl GitEditTools {
    // This implementation is split across multiple files but needs to be
    // combined into a single #[tool(tool_box)] impl block

    // Project tools - defined in tool_definition/project_impl.rs
    #[tool(
        description = "Update a project item field using string parameters. Supports text, number, date, single_select, and multi_select field types."
    )]
    async fn update_project_item_field(
        &self,
        #[tool(param)]
        #[schemars(description = "The project node identifier (GraphQL ID)")]
        project_node_id: String,
        #[tool(param)]
        #[schemars(description = "The project item ID (GraphQL node ID)")]
        project_item_id: String,
        #[tool(param)]
        #[schemars(description = "The field ID (GraphQL node ID)")]
        project_field_id: String,
        #[tool(param)]
        #[schemars(
            description = "The field type (text, number, date, single_select, multi_select)"
        )]
        field_type: String,
        #[tool(param)]
        #[schemars(
            description = "The field value as string (will be parsed according to field_type). Examples: text: 'Hello World', number: '42.5', date: '2024-01-15T10:30:00Z', single_select: 'In Progress', multi_select: 'bug,enhancement,feature'"
        )]
        value: String,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::ProjectTools::update_project_item_field(
            &self.github_client,
            project_node_id,
            project_item_id,
            project_field_id,
            field_type,
            value,
        )
        .await
    }

    #[tool(description = "Get project node ID from project identifier")]
    async fn get_project_node_id(
        &self,
        #[tool(param)]
        #[schemars(description = "Project owner username or organization name")]
        project_owner: String,
        #[tool(param)]
        #[schemars(description = "Project number")]
        project_number: u64,
        #[tool(param)]
        #[schemars(description = "Project type (user or organization)")]
        project_type: String,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::ProjectTools::get_project_node_id(
            &self.github_client,
            project_owner,
            project_number,
            project_type,
        )
        .await
    }

    #[tool(description = "Update a project item text field")]
    async fn update_project_item_text_field(
        &self,
        #[tool(param)]
        #[schemars(description = "The project node identifier (GraphQL ID)")]
        project_node_id: String,
        #[tool(param)]
        #[schemars(description = "The project item ID (GraphQL node ID)")]
        project_item_id: String,
        #[tool(param)]
        #[schemars(description = "The field ID (GraphQL node ID)")]
        project_field_id: String,
        #[tool(param)]
        #[schemars(description = "The text value to set")]
        text_value: String,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::ProjectTools::update_project_item_text_field(
            &self.github_client,
            project_node_id,
            project_item_id,
            project_field_id,
            text_value,
        )
        .await
    }

    #[tool(description = "Update a project item number field")]
    async fn update_project_item_number_field(
        &self,
        #[tool(param)]
        #[schemars(description = "The project node identifier (GraphQL ID)")]
        project_node_id: String,
        #[tool(param)]
        #[schemars(description = "The project item ID (GraphQL node ID)")]
        project_item_id: String,
        #[tool(param)]
        #[schemars(description = "The field ID (GraphQL node ID)")]
        project_field_id: String,
        #[tool(param)]
        #[schemars(description = "The number value to set")]
        number_value: f64,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::ProjectTools::update_project_item_number_field(
            &self.github_client,
            project_node_id,
            project_item_id,
            project_field_id,
            number_value,
        )
        .await
    }

    #[tool(description = "Update a project item date field")]
    async fn update_project_item_date_field(
        &self,
        #[tool(param)]
        #[schemars(description = "The project node identifier (GraphQL ID)")]
        project_node_id: String,
        #[tool(param)]
        #[schemars(description = "The project item ID (GraphQL node ID)")]
        project_item_id: String,
        #[tool(param)]
        #[schemars(description = "The field ID (GraphQL node ID)")]
        project_field_id: String,
        #[tool(param)]
        #[schemars(
            description = "The date value in ISO 8601 format (e.g., '2024-01-15T10:30:00Z')"
        )]
        date_value: String,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::ProjectTools::update_project_item_date_field(
            &self.github_client,
            project_node_id,
            project_item_id,
            project_field_id,
            date_value,
        )
        .await
    }

    #[tool(description = "Update a project item single select field")]
    async fn update_project_item_single_select_field(
        &self,
        #[tool(param)]
        #[schemars(description = "The project node identifier (GraphQL ID)")]
        project_node_id: String,
        #[tool(param)]
        #[schemars(description = "The project item ID (GraphQL node ID)")]
        project_item_id: String,
        #[tool(param)]
        #[schemars(description = "The field ID (GraphQL node ID)")]
        project_field_id: String,
        #[tool(param)]
        #[schemars(description = "The option ID to select (GraphQL node ID)")]
        option_id: String,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::ProjectTools::update_project_item_single_select_field(
            &self.github_client,
            project_node_id,
            project_item_id,
            project_field_id,
            option_id,
        )
        .await
    }

    #[tool(description = "Add an issue to a project")]
    async fn add_issue_to_project(
        &self,
        #[tool(param)]
        #[schemars(description = "The project node identifier (GraphQL ID)")]
        project_node_id: String,
        #[tool(param)]
        #[schemars(description = "Repository owner username or organization name")]
        repository_owner: String,
        #[tool(param)]
        #[schemars(description = "Repository name")]
        repository_name: String,
        #[tool(param)]
        #[schemars(description = "Issue number to add to the project")]
        issue_number: u64,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::ProjectTools::add_issue_to_project(
            &self.github_client,
            project_node_id,
            repository_owner,
            repository_name,
            IssueNumber::new(issue_number.try_into().unwrap()),
        )
        .await
    }

    #[tool(description = "Add a pull request to a project")]
    async fn add_pull_request_to_project(
        &self,
        #[tool(param)]
        #[schemars(description = "The project node identifier (GraphQL ID)")]
        project_node_id: String,
        #[tool(param)]
        #[schemars(description = "Repository owner username or organization name")]
        repository_owner: String,
        #[tool(param)]
        #[schemars(description = "Repository name")]
        repository_name: String,
        #[tool(param)]
        #[schemars(description = "Pull request number to add to the project")]
        pull_request_number: u64,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::ProjectTools::add_pull_request_to_project(
            &self.github_client,
            project_node_id,
            repository_owner,
            repository_name,
            pull_request_number,
        )
        .await
    }

    #[tool(description = "Create a new pull request")]
    async fn create_pull_request(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Pull request title")]
        title: String,
        #[tool(param)]
        #[schemars(description = "Head branch name containing the changes")]
        head_branch: String,
        #[tool(param)]
        #[schemars(description = "Base branch name to merge into")]
        base_branch: String,
        #[tool(param)]
        #[schemars(description = "Optional pull request body content")]
        body: Option<String>,
        #[tool(param)]
        #[schemars(description = "Whether to create as draft (default: false)")]
        draft: Option<bool>,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::PullRequestTools::create_pull_request(
            &self.github_client,
            repository_url,
            title,
            head_branch,
            base_branch,
            body,
            draft,
        )
        .await
    }

    #[tool(description = "Add a comment to a pull request")]
    async fn add_comment_to_pull_request(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Pull request number")]
        pr_number: u64,
        #[tool(param)]
        #[schemars(description = "Comment content")]
        body: String,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::PullRequestTools::add_comment_to_pull_request(
            &self.github_client,
            repository_url,
            pr_number,
            body,
        )
        .await
    }

    #[tool(description = "Edit an existing pull request comment")]
    async fn edit_comment_on_pull_request(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Pull request number")]
        pr_number: u64,
        #[tool(param)]
        #[schemars(description = "Comment number to edit")]
        comment_number: u64,
        #[tool(param)]
        #[schemars(description = "New comment content")]
        body: String,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::PullRequestTools::edit_comment_on_pull_request(
            &self.github_client,
            repository_url,
            pr_number,
            PullRequestCommentNumber::new(comment_number),
            body,
        )
        .await
    }

    #[tool(description = "Close a pull request")]
    async fn close_pull_request(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Pull request number to close")]
        pr_number: u64,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::PullRequestTools::close_pull_request(
            &self.github_client,
            repository_url,
            pr_number,
        )
        .await
    }

    #[tool(description = "Edit the title of a pull request")]
    async fn edit_pull_request_title(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Pull request number")]
        pr_number: u64,
        #[tool(param)]
        #[schemars(description = "New title content")]
        title: String,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::PullRequestTools::edit_pull_request_title(
            &self.github_client,
            repository_url,
            pr_number,
            title,
        )
        .await
    }

    #[tool(description = "Edit the body of a pull request")]
    async fn edit_pull_request_body(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Pull request number")]
        pr_number: u64,
        #[tool(param)]
        #[schemars(description = "New body content")]
        body: String,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::PullRequestTools::edit_pull_request_body(
            &self.github_client,
            repository_url,
            pr_number,
            body,
        )
        .await
    }

    #[tool(description = "Add assignees to a pull request")]
    async fn add_assignees_to_pull_request(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Pull request number")]
        pr_number: u64,
        #[tool(param)]
        #[schemars(description = "List of usernames to add as assignees")]
        new_assignees: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::PullRequestTools::add_assignees_to_pull_request(
            &self.github_client,
            repository_url,
            pr_number,
            new_assignees,
        )
        .await
    }

    #[tool(description = "Remove assignees from a pull request")]
    async fn remove_assignees_from_pull_request(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Pull request number")]
        pr_number: u64,
        #[tool(param)]
        #[schemars(description = "List of usernames to remove from assignees")]
        assignees: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::PullRequestTools::remove_assignees_from_pull_request(
            &self.github_client,
            repository_url,
            pr_number,
            assignees,
        )
        .await
    }

    #[tool(description = "Add requested reviewers to a pull request")]
    async fn add_requested_reviewers_to_pull_request(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Pull request number")]
        pr_number: u64,
        #[tool(param)]
        #[schemars(description = "List of usernames to request as reviewers")]
        new_reviewers: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::PullRequestTools::add_requested_reviewers_to_pull_request(
            &self.github_client,
            repository_url,
            pr_number,
            new_reviewers,
        )
        .await
    }

    #[tool(description = "Add labels to a pull request")]
    async fn add_labels_to_pull_request(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Pull request number")]
        pr_number: u64,
        #[tool(param)]
        #[schemars(description = "List of label names to add")]
        labels: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::PullRequestTools::add_labels_to_pull_request(
            &self.github_client,
            repository_url,
            pr_number,
            labels,
        )
        .await
    }

    #[tool(description = "Remove labels from a pull request")]
    async fn remove_labels_from_pull_request(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Pull request number")]
        pr_number: u64,
        #[tool(param)]
        #[schemars(description = "List of label names to remove")]
        labels: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::PullRequestTools::remove_labels_from_pull_request(
            &self.github_client,
            repository_url,
            pr_number,
            labels,
        )
        .await
    }

    #[tool(description = "Add milestone to a pull request")]
    async fn add_milestone_to_pull_request(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Pull request number")]
        pr_number: u64,
        #[tool(param)]
        #[schemars(description = "Milestone ID to assign")]
        milestone_number: u64,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::PullRequestTools::add_milestone_to_pull_request(
            &self.github_client,
            repository_url,
            pr_number,
            milestone_number,
        )
        .await
    }

    #[tool(description = "Remove milestone from a pull request")]
    async fn remove_milestone_from_pull_request(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Pull request number")]
        pr_number: u64,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::PullRequestTools::remove_milestone_from_pull_request(
            &self.github_client,
            repository_url,
            pr_number,
        )
        .await
    }

    #[tool(description = "Create a new issue")]
    async fn create_issue(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Issue title")]
        title: String,
        #[tool(param)]
        #[schemars(description = "Optional issue body content")]
        body: Option<String>,
        #[tool(param)]
        #[schemars(description = "Optional list of assignee usernames")]
        assignees: Option<Vec<String>>,
        #[tool(param)]
        #[schemars(description = "Optional list of label names")]
        labels: Option<Vec<String>>,
        #[tool(param)]
        #[schemars(description = "Optional milestone ID")]
        milestone_number: Option<u64>,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::IssueTools::create_issue(
            &self.github_client,
            repository_url,
            title,
            body,
            assignees,
            labels,
            milestone_number,
        )
        .await
    }

    #[tool(description = "Add a comment to an issue")]
    async fn add_comment_to_issue(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Issue number")]
        issue_number: u64,
        #[tool(param)]
        #[schemars(description = "Comment content")]
        body: String,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::IssueTools::add_comment_to_issue(
            &self.github_client,
            repository_url,
            IssueNumber::new(issue_number.try_into().unwrap()),
            body,
        )
        .await
    }

    #[tool(description = "Edit an existing issue comment")]
    async fn edit_comment_on_issue(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Issue number")]
        issue_number: u64,
        #[tool(param)]
        #[schemars(description = "Comment number to edit")]
        comment_number: u64,
        #[tool(param)]
        #[schemars(description = "New comment content")]
        body: String,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::IssueTools::edit_comment_on_issue(
            &self.github_client,
            repository_url,
            IssueNumber::new(issue_number.try_into().unwrap()),
            IssueCommentNumber::new(comment_number),
            body,
        )
        .await
    }

    #[tool(description = "Edit the title of an issue")]
    async fn edit_issue_title(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Issue number")]
        issue_number: u64,
        #[tool(param)]
        #[schemars(description = "New title content")]
        title: String,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::IssueTools::edit_issue_title(
            &self.github_client,
            repository_url,
            IssueNumber::new(issue_number.try_into().unwrap()),
            title,
        )
        .await
    }

    #[tool(description = "Edit the body of an issue")]
    async fn edit_issue_body(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Issue number")]
        issue_number: u64,
        #[tool(param)]
        #[schemars(description = "New body content")]
        body: String,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::IssueTools::edit_issue_body(
            &self.github_client,
            repository_url,
            IssueNumber::new(issue_number.try_into().unwrap()),
            body,
        )
        .await
    }

    #[tool(description = "Update the state of an issue")]
    async fn update_issue_state(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Issue number")]
        issue_number: u64,
        #[tool(param)]
        #[schemars(description = "New state (open or closed)")]
        state: String,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::IssueTools::update_issue_state(
            &self.github_client,
            repository_url,
            IssueNumber::new(issue_number.try_into().unwrap()),
            state,
        )
        .await
    }

    #[tool(description = "Add assignees to an issue")]
    async fn add_assignees_to_issue(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Issue number")]
        issue_number: u64,
        #[tool(param)]
        #[schemars(description = "List of usernames to add as assignees")]
        new_assignees: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::IssueTools::add_assignees_to_issue(
            &self.github_client,
            repository_url,
            IssueNumber::new(issue_number.try_into().unwrap()),
            new_assignees,
        )
        .await
    }

    #[tool(description = "Remove assignees from an issue")]
    async fn remove_assignees_from_issue(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Issue number")]
        issue_number: u64,
        #[tool(param)]
        #[schemars(description = "List of usernames to remove from assignees")]
        assignees: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::IssueTools::remove_assignees_from_issue(
            &self.github_client,
            repository_url,
            IssueNumber::new(issue_number.try_into().unwrap()),
            assignees,
        )
        .await
    }

    #[tool(description = "Add labels to an issue")]
    async fn add_labels_to_issue(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Issue number")]
        issue_number: u64,
        #[tool(param)]
        #[schemars(description = "List of label names to add")]
        labels: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::IssueTools::add_labels_to_issue(
            &self.github_client,
            repository_url,
            IssueNumber::new(issue_number.try_into().unwrap()),
            labels,
        )
        .await
    }

    #[tool(description = "Add milestone to an issue")]
    async fn add_milestone_to_issue(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Issue number")]
        issue_number: u64,
        #[tool(param)]
        #[schemars(description = "Milestone number to assign")]
        milestone_number: u64,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::IssueTools::add_milestone_to_issue(
            &self.github_client,
            repository_url,
            IssueNumber::new(issue_number.try_into().unwrap()),
            milestone_number,
        )
        .await
    }

    #[tool(description = "Remove labels from an issue")]
    async fn remove_labels_from_issue(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Issue number")]
        issue_number: u64,
        #[tool(param)]
        #[schemars(description = "List of label names to remove")]
        labels: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::IssueTools::remove_labels_from_issue(
            &self.github_client,
            repository_url,
            IssueNumber::new(issue_number.try_into().unwrap()),
            labels,
        )
        .await
    }

    #[tool(description = "Remove milestone from an issue")]
    async fn remove_milestone_from_issue(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Issue number")]
        issue_number: u64,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::IssueTools::remove_milestone_from_issue(
            &self.github_client,
            repository_url,
            IssueNumber::new(issue_number.try_into().unwrap()),
        )
        .await
    }

    #[tool(description = "Create a new milestone in a repository")]
    async fn create_milestone(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Milestone title")]
        title: String,
        #[tool(param)]
        #[schemars(description = "Optional milestone description")]
        description: Option<String>,
        #[tool(param)]
        #[schemars(
            description = "Optional due date in ISO 8601 format (e.g., '2024-01-15T10:30:00Z')"
        )]
        due_on: Option<String>,
        #[tool(param)]
        #[schemars(description = "Optional state (open or closed)")]
        state: Option<String>,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::RepositoryTools::create_milestone(
            &self.github_client,
            repository_url,
            title,
            description,
            due_on,
            state,
        )
        .await
    }

    #[tool(description = "Create a new label in a repository")]
    async fn create_label(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Label name")]
        name: String,
        #[tool(param)]
        #[schemars(description = "Optional label color (6 character hex code without #)")]
        color: Option<String>,
        #[tool(param)]
        #[schemars(description = "Optional label description")]
        description: Option<String>,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::RepositoryTools::create_label(
            &self.github_client,
            repository_url,
            name,
            color,
            description,
        )
        .await
    }

    #[tool(description = "Update an existing label in a repository")]
    async fn update_label(
        &self,
        #[tool(param)]
        #[schemars(
            description = "Repository URL (e.g., 'https://github.com/owner/repo', 'owner/repo')"
        )]
        repository_url: String,
        #[tool(param)]
        #[schemars(description = "Current label name")]
        old_name: String,
        #[tool(param)]
        #[schemars(description = "Optional new label name")]
        new_name: Option<String>,
        #[tool(param)]
        #[schemars(description = "Optional new label color (6 character hex code without #)")]
        color: Option<String>,
        #[tool(param)]
        #[schemars(description = "Optional new label description")]
        description: Option<String>,
    ) -> Result<CallToolResult, McpError> {
        tool_definition::RepositoryTools::update_label(
            &self.github_client,
            repository_url,
            old_name,
            new_name,
            color,
            description,
        )
        .await
    }
}

#[tool(tool_box)]
impl ServerHandler for GitEditTools {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            server_info: Implementation {
                name: "github-edit".into(),
                version: "0.1.3".into(),
            },
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            instructions: Some(
                "GitHub Edit MCP Server - Updates GitHub project item fields".into(),
            ),
        }
    }
}
