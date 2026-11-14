//! Project-related CLI commands and execution logic
//!
//! This module contains the CLI command definitions and execution logic
//! for project management operations including updating custom fields
//! and managing project items.

use anyhow::Result;
use clap::Subcommand;
use github_edit::github::GitHubClient;
use github_edit::tools::functions::project;
use github_edit::types::project::{
    ProjectCustomFieldType, ProjectFieldId, ProjectFieldValue, ProjectItemId, ProjectNodeId,
};
use github_edit::types::{IssueNumber, PullRequestNumber, RepositoryId};
use std::str::FromStr;

#[derive(Subcommand)]
pub enum ProjectAction {
    /// Update a project item field value
    ///
    /// Examples:
    ///   github-edit-cli project update-field --project-node-id "PN_kwDOBw6lbs4AAVGQ" --project-item-id "PVTI_lADOBw6lbs4AAVGQzgF6sCo" --project-field-id "PVTF_lADOBw6lbs4AAVGQzgF6sCo" --field-type text --value "In Progress"
    ///   github-edit-cli project update-field --project-node-id "PN_kwDOBw6lbs4AAVGQ" --project-item-id "PVTI_lADOBw6lbs4AAVGQzgF6sCo" --project-field-id "PVTF_lADOBw6lbs4AAVGQzgF6sCo" --field-type single_select --value "High Priority"
    UpdateField {
        /// Project node ID (GraphQL ID from GitHub Projects)
        ///
        /// How to find: Go to your GitHub project, use browser dev tools to inspect
        /// the page source, or use GitHub's GraphQL API to query project details.
        ///
        /// Examples:
        ///   PN_kwDOBw6lbs4AAVGQ
        ///   PN_kwDOABcDEf4AAGH1
        ///   PN_kwDOXYZ123abcDEF
        #[arg(long, value_name = "NODE_ID")]
        project_node_id: String,
        /// Project item ID (GraphQL node ID for the specific item/row)
        ///
        /// This represents a specific item (issue or PR) within the project.
        /// You can get this from the GitHub GraphQL API or by inspecting the
        /// project's data structure.
        ///
        /// Examples:
        ///   PVTI_lADOBw6lbs4AAVGQzgF6sCo
        ///   PVTI_lADOABcDEf4AAGH1zgH7tDp
        ///   PVTI_lADOXYZ123abcDEFghI8uEm
        #[arg(long, value_name = "ITEM_ID")]
        project_item_id: String,
        /// Field ID (GraphQL node ID for the specific field/column)
        ///
        /// This represents a specific field (column) in your project.
        /// Each custom field has its own unique identifier.
        ///
        /// Examples:
        ///   PVTF_lADOBw6lbs4AAVGQzgF6sCo (Status field)
        ///   PVTF_lADOABcDEf4AAGH1zgH7tDp (Priority field)
        ///   PVTF_lADOXYZ123abcDEFghI8uEm (Assignee field)
        #[arg(long, value_name = "FIELD_ID")]
        project_field_id: String,
        /// Field type (determines how the value is interpreted)
        ///
        /// Valid field types:
        ///   text          - Plain text fields (e.g., "Needs Review", "Custom Note")
        ///   number        - Numeric fields (e.g., "5", "42", "100")
        ///   date          - Date fields in ISO format (e.g., "2024-01-15", "2024-12-31")
        ///   single_select - Single choice from predefined options (e.g., "High", "Medium", "Low")
        ///   multi_select  - Multiple choices from predefined options (comma-separated)
        ///
        /// Examples by type:
        ///   --field-type text --value "Ready for deployment"
        ///   --field-type number --value "85"
        ///   --field-type date --value "2024-03-15"
        ///   --field-type single_select --value "High Priority"
        ///   --field-type multi_select --value "bug,frontend,urgent"
        #[arg(long, value_name = "TYPE")]
        field_type: ProjectCustomFieldType,
        /// Field value (format depends on field type)
        ///
        /// Value formats by field type:
        ///
        /// TEXT fields:
        ///   Any string value
        ///   Examples: "In Progress", "Needs Review", "Custom description"
        ///
        /// NUMBER fields:
        ///   Numeric values (integers or decimals)
        ///   Examples: "5", "42", "3.14", "100"
        ///
        /// DATE fields:
        ///   ISO date format (YYYY-MM-DD)
        ///   Examples: "2024-01-15", "2024-12-31", "2025-06-30"
        ///
        /// SINGLE_SELECT fields:
        ///   Must match one of the predefined options exactly
        ///   Examples: "High Priority", "Medium", "Done", "In Progress"
        ///
        /// MULTI_SELECT fields:
        ///   Comma-separated list of predefined options
        ///   Examples: "bug,critical", "frontend,ui,enhancement", "docs,help-wanted"
        #[arg(long, value_name = "VALUE")]
        value: String,
    },
    /// Update project item field using raw field value
    ///
    /// Examples:
    ///   github-edit-cli project update-field-value --project-node-id "PN_kwDOBw6lbs4AAVGQ" --project-item-id "PVTI_lADOBw6lbs4AAVGQzgF6sCo" --project-field-id "PVTF_lADOBw6lbs4AAVGQzgF6sCo" --field-type text --value "In Progress"
    UpdateFieldValue {
        /// Project node ID (GraphQL ID from GitHub Projects)
        #[arg(long, value_name = "NODE_ID")]
        project_node_id: String,
        /// Project item ID (GraphQL node ID for the specific item/row)
        #[arg(long, value_name = "ITEM_ID")]
        project_item_id: String,
        /// Field ID (GraphQL node ID for the specific field/column)
        #[arg(long, value_name = "FIELD_ID")]
        project_field_id: String,
        /// Field type (determines how the value is interpreted)
        #[arg(long, value_name = "TYPE")]
        field_type: ProjectCustomFieldType,
        /// Field value (format depends on field type)
        #[arg(long, value_name = "VALUE")]
        value: String,
    },
    /// Update project item text field
    ///
    /// Examples:
    ///   github-edit-cli project update-text-field --project-node-id "PN_kwDOBw6lbs4AAVGQ" --project-item-id "PVTI_lADOBw6lbs4AAVGQzgF6sCo" --project-field-id "PVTF_lADOBw6lbs4AAVGQzgF6sCo" --text-value "Ready for review"
    UpdateTextField {
        /// Project node ID (GraphQL ID from GitHub Projects)
        #[arg(long, value_name = "NODE_ID")]
        project_node_id: String,
        /// Project item ID (GraphQL node ID for the specific item/row)
        #[arg(long, value_name = "ITEM_ID")]
        project_item_id: String,
        /// Field ID (GraphQL node ID for the specific field/column)
        #[arg(long, value_name = "FIELD_ID")]
        project_field_id: String,
        /// Text value to set
        #[arg(long, value_name = "TEXT")]
        text_value: String,
    },
    /// Update project item number field
    ///
    /// Examples:
    ///   github-edit-cli project update-number-field --project-node-id "PN_kwDOBw6lbs4AAVGQ" --project-item-id "PVTI_lADOBw6lbs4AAVGQzgF6sCo" --project-field-id "PVTF_lADOBw6lbs4AAVGQzgF6sCo" --number-value 85
    UpdateNumberField {
        /// Project node ID (GraphQL ID from GitHub Projects)
        #[arg(long, value_name = "NODE_ID")]
        project_node_id: String,
        /// Project item ID (GraphQL node ID for the specific item/row)
        #[arg(long, value_name = "ITEM_ID")]
        project_item_id: String,
        /// Field ID (GraphQL node ID for the specific field/column)
        #[arg(long, value_name = "FIELD_ID")]
        project_field_id: String,
        /// Number value to set
        #[arg(long, value_name = "NUMBER")]
        number_value: f64,
    },
    /// Update project item date field
    ///
    /// Examples:
    ///   github-edit-cli project update-date-field --project-node-id "PN_kwDOBw6lbs4AAVGQ" --project-item-id "PVTI_lADOBw6lbs4AAVGQzgF6sCo" --project-field-id "PVTF_lADOBw6lbs4AAVGQzgF6sCo" --date-value "2024-12-31T23:59:59Z"
    UpdateDateField {
        /// Project node ID (GraphQL ID from GitHub Projects)
        #[arg(long, value_name = "NODE_ID")]
        project_node_id: String,
        /// Project item ID (GraphQL node ID for the specific item/row)
        #[arg(long, value_name = "ITEM_ID")]
        project_item_id: String,
        /// Field ID (GraphQL node ID for the specific field/column)
        #[arg(long, value_name = "FIELD_ID")]
        project_field_id: String,
        /// Date value in ISO format (e.g., "2024-12-31T23:59:59Z")
        #[arg(long, value_name = "DATE")]
        date_value: String,
    },
    /// Update project item single select field
    ///
    /// Examples:
    ///   github-edit-cli project update-single-select-field --project-node-id "PN_kwDOBw6lbs4AAVGQ" --project-item-id "PVTI_lADOBw6lbs4AAVGQzgF6sCo" --project-field-id "PVTF_lADOBw6lbs4AAVGQzgF6sCo" --option-id "f75ad846"
    UpdateSingleSelectField {
        /// Project node ID (GraphQL ID from GitHub Projects)
        #[arg(long, value_name = "NODE_ID")]
        project_node_id: String,
        /// Project item ID (GraphQL node ID for the specific item/row)
        #[arg(long, value_name = "ITEM_ID")]
        project_item_id: String,
        /// Field ID (GraphQL node ID for the specific field/column)
        #[arg(long, value_name = "FIELD_ID")]
        project_field_id: String,
        /// Option ID for the selected value
        #[arg(long, value_name = "OPTION_ID")]
        option_id: String,
    },
    /// Add an issue to a project
    ///
    /// Examples:
    ///   github-edit-cli project add-issue --project-node-id "PN_kwDOBw6lbs4AAVGQ" --owner "octocat" --repo "Hello-World" --issue-number 123
    AddIssue {
        /// Project node ID (GraphQL ID from GitHub Projects)
        #[arg(long, value_name = "NODE_ID")]
        project_node_id: String,
        /// Repository owner
        #[arg(long, value_name = "OWNER")]
        owner: String,
        /// Repository name
        #[arg(long, value_name = "REPO")]
        repo: String,
        /// Issue number
        #[arg(long, value_name = "NUMBER")]
        issue_number: u32,
    },
    /// Add a pull request to a project
    ///
    /// Examples:
    ///   github-edit-cli project add-pull-request --project-node-id "PN_kwDOBw6lbs4AAVGQ" --owner "octocat" --repo "Hello-World" --pull-request-number 456
    AddPullRequest {
        /// Project node ID (GraphQL ID from GitHub Projects)
        #[arg(long, value_name = "NODE_ID")]
        project_node_id: String,
        /// Repository owner
        #[arg(long, value_name = "OWNER")]
        owner: String,
        /// Repository name
        #[arg(long, value_name = "REPO")]
        repo: String,
        /// Pull request number
        #[arg(long, value_name = "NUMBER")]
        pull_request_number: u32,
    },
}

pub async fn execute_project_action(
    github_client: &GitHubClient,
    action: ProjectAction,
) -> Result<()> {
    match action {
        ProjectAction::UpdateField {
            project_node_id,
            project_item_id,
            project_field_id,
            field_type,
            value,
        } => {
            let typed_project_node_id = ProjectNodeId::new(project_node_id);
            let typed_project_item_id = ProjectItemId::new(project_item_id);
            let typed_project_field_id = ProjectFieldId::new(project_field_id);

            // Parse field type from string to enum
            let field_type_enum = ProjectCustomFieldType::from_str(&field_type.as_ref())
                .map_err(|_| anyhow::anyhow!(
                    "Unsupported field type '{}'. Supported types: text, number, date, single_select, multi_select",
                    field_type.as_ref()
                ))?;

            // Parse field value using the ProjectFieldValue method
            let parsed_value = ProjectFieldValue::from_string_with_type(&field_type_enum, &value)?;

            project::update_project_item_field(
                github_client,
                &typed_project_node_id,
                &typed_project_item_id,
                &typed_project_field_id,
                &parsed_value,
            )
            .await?;
            println!("Updated project item field successfully");
        }
        ProjectAction::UpdateFieldValue {
            project_node_id,
            project_item_id,
            project_field_id,
            field_type,
            value,
        } => {
            let typed_project_node_id = ProjectNodeId::new(project_node_id);
            let typed_project_item_id = ProjectItemId::new(project_item_id);
            let typed_project_field_id = ProjectFieldId::new(project_field_id);

            // Parse field type from string to enum
            let field_type_enum = ProjectCustomFieldType::from_str(&field_type.as_ref())
                .map_err(|_| anyhow::anyhow!(
                    "Unsupported field type '{}'. Supported types: text, number, date, single_select, multi_select",
                    field_type.as_ref()
                ))?;

            // Parse field value using the ProjectFieldValue method
            let parsed_value = ProjectFieldValue::from_string_with_type(&field_type_enum, &value)?;

            project::update_project_item_field_value(
                github_client,
                &typed_project_node_id,
                &typed_project_item_id,
                &typed_project_field_id,
                &parsed_value,
            )
            .await?;
            println!("Updated project item field value successfully");
        }
        ProjectAction::UpdateTextField {
            project_node_id,
            project_item_id,
            project_field_id,
            text_value,
        } => {
            let typed_project_node_id = ProjectNodeId::new(project_node_id);
            let typed_project_item_id = ProjectItemId::new(project_item_id);
            let typed_project_field_id = ProjectFieldId::new(project_field_id);

            project::update_project_item_text_field(
                github_client,
                &typed_project_node_id,
                &typed_project_item_id,
                &typed_project_field_id,
                &text_value,
            )
            .await?;
            println!("Updated project item text field successfully");
        }
        ProjectAction::UpdateNumberField {
            project_node_id,
            project_item_id,
            project_field_id,
            number_value,
        } => {
            let typed_project_node_id = ProjectNodeId::new(project_node_id);
            let typed_project_item_id = ProjectItemId::new(project_item_id);
            let typed_project_field_id = ProjectFieldId::new(project_field_id);

            project::update_project_item_number_field(
                github_client,
                &typed_project_node_id,
                &typed_project_item_id,
                &typed_project_field_id,
                number_value,
            )
            .await?;
            println!("Updated project item number field successfully");
        }
        ProjectAction::UpdateDateField {
            project_node_id,
            project_item_id,
            project_field_id,
            date_value,
        } => {
            let typed_project_node_id = ProjectNodeId::new(project_node_id);
            let typed_project_item_id = ProjectItemId::new(project_item_id);
            let typed_project_field_id = ProjectFieldId::new(project_field_id);

            // Parse date string to DateTime
            let parsed_date = chrono::DateTime::parse_from_rfc3339(&date_value)
                .map_err(|e| anyhow::anyhow!("Invalid date format '{}': {}", date_value, e))?
                .with_timezone(&chrono::Utc);

            project::update_project_item_date_field(
                github_client,
                &typed_project_node_id,
                &typed_project_item_id,
                &typed_project_field_id,
                parsed_date,
            )
            .await?;
            println!("Updated project item date field successfully");
        }
        ProjectAction::UpdateSingleSelectField {
            project_node_id,
            project_item_id,
            project_field_id,
            option_id,
        } => {
            let typed_project_node_id = ProjectNodeId::new(project_node_id);
            let typed_project_item_id = ProjectItemId::new(project_item_id);
            let typed_project_field_id = ProjectFieldId::new(project_field_id);

            project::update_project_item_single_select_field(
                github_client,
                &typed_project_node_id,
                &typed_project_item_id,
                &typed_project_field_id,
                &option_id,
            )
            .await?;
            println!("Updated project item single select field successfully");
        }
        ProjectAction::AddIssue {
            project_node_id,
            owner,
            repo,
            issue_number,
        } => {
            let typed_project_node_id = ProjectNodeId::new(project_node_id);
            let repository_id = RepositoryId::new(owner, repo);
            let typed_issue_number = IssueNumber::new(issue_number);

            let project_item_id = project::add_issue_to_project(
                github_client,
                &typed_project_node_id,
                &repository_id,
                typed_issue_number,
            )
            .await?;
            println!(
                "Added issue to project successfully. Project item ID: {}",
                project_item_id.0.as_str()
            );
        }
        ProjectAction::AddPullRequest {
            project_node_id,
            owner,
            repo,
            pull_request_number,
        } => {
            let typed_project_node_id = ProjectNodeId::new(project_node_id);
            let repository_id = RepositoryId::new(owner, repo);
            let typed_pr_number = PullRequestNumber::new(pull_request_number);

            let project_item_id = project::add_pull_request_to_project(
                github_client,
                &typed_project_node_id,
                &repository_id,
                typed_pr_number,
            )
            .await?;
            println!(
                "Added pull request to project successfully. Project item ID: {}",
                project_item_id.0.as_str()
            );
        }
    }
    Ok(())
}
