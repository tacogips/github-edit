//! Project domain types and URL parsing
//!
//! This module contains the Project domain types with comprehensive URL parsing
//! capabilities. Following domain-driven design principles, all project-specific
//! URL parsing logic is contained within this module.

use anyhow;
use chrono::{DateTime, Utc};
use clap::ValueEnum;
use once_cell::sync::Lazy;
use regex::Regex;
use schemars::JsonSchema;
use strum::{AsRefStr, Display, EnumString};

use crate::types::label::Label;
use crate::types::user::User;
use serde::{Deserialize, Serialize};

use crate::types::{issue::IssueId, pull_request::PullRequestId, repository::Owner};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectUrl(pub String);

impl std::fmt::Display for ProjectUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

static PROJECT_URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?:https?://)?github\.com/(orgs|users)/([^/]+)/projects/(\d+)")
        .expect("Failed to compile project URL regex")
});

/// Project type to distinguish between user and organization projects
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    EnumString,
    Display,
    ValueEnum,
)]
#[strum(serialize_all = "lowercase")]
pub enum ProjectType {
    /// User project (github.com/users/owner/projects/123)
    User,
    /// Organization project (github.com/orgs/owner/projects/123)
    Organization,
}

/// Project number wrapper for type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct ProjectNumber(pub u64);

impl ProjectNumber {
    /// Create new project number
    pub fn new(number: u64) -> Self {
        Self(number)
    }

    /// Get the numeric value
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for ProjectNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Strong-typed project identifier with URL parsing capabilities.
///
/// This struct encapsulates all project identification logic and URL parsing
/// specific to projects. Following domain-driven design, all project URL
/// parsing logic is self-contained within this domain.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct ProjectId {
    pub owner: Owner,
    pub number: ProjectNumber,
    pub project_type: ProjectType,
}

impl ProjectId {
    /// Create new project identifier
    pub fn new(owner: Owner, number: ProjectNumber, project_type: ProjectType) -> Self {
        Self {
            owner,
            number,
            project_type,
        }
    }
    pub fn url(&self) -> String {
        let path_type = match self.project_type {
            ProjectType::Organization => "orgs",
            ProjectType::User => "users",
        };
        format!(
            "https://github.com/{}/{}/projects/{}",
            path_type, self.owner, self.number
        )
    }

    /// Parse GitHub project URL to extract owner, project number, and project type
    ///
    /// Domain-specific URL parsing moved from utils to maintain domain boundaries.
    /// Supports both user and organization project URLs.
    pub fn parse_url(url: &ProjectUrl) -> Result<(String, u64, ProjectType), String> {
        let url = url.0.to_string();
        let url = url.trim_end_matches('/');

        // Parse GitHub project URL patterns:
        // https://github.com/orgs/owner/projects/123
        // https://github.com/users/owner/projects/123
        if let Some(captures) = PROJECT_URL_REGEX.captures(url) {
            let project_type = match captures.get(1).unwrap().as_str() {
                "orgs" => ProjectType::Organization,
                "users" => ProjectType::User,
                _ => return Err("Invalid project type".to_string()),
            };
            let owner = captures.get(2).unwrap().as_str().to_string();
            let number = captures
                .get(3)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .map_err(|_| "Invalid project number")?;

            return Ok((owner, number, project_type));
        }

        Err(format!("Invalid GitHub project URL format: {}", url))
    }

    /// Returns the owner part of the project
    pub fn owner(&self) -> &Owner {
        &self.owner
    }

    /// Returns the project number
    pub fn project_number(&self) -> ProjectNumber {
        self.number
    }

    /// Returns the project type
    pub fn project_type(&self) -> ProjectType {
        self.project_type
    }
}

impl std::fmt::Display for ProjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct ProjectNodeId(pub String);

impl ProjectNodeId {
    /// Create new project node ID
    pub fn new(id: String) -> Self {
        Self(id)
    }

    /// Get the string value
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ProjectNodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Git project with resources and custom fields.
///
/// Contains comprehensive project information including custom fields,
/// project items, and resource management capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub project_id: ProjectId,
    pub project_node_id: ProjectNodeId,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents the state of a GitHub project
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display, ValueEnum,
)]
#[strum(serialize_all = "lowercase")]
pub enum ProjectState {
    /// Project is open and active
    Open,
    /// Project is closed
    Closed,
}

/// Represents the visibility of a GitHub project
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display, ValueEnum,
)]
#[strum(serialize_all = "lowercase")]
pub enum ProjectVisibility {
    /// Project is public
    Public,
    /// Project is private
    Private,
}

impl Project {
    /// Create new project with basic metadata
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        project_id: ProjectId,
        project_node_id: ProjectNodeId,
        title: String,
        description: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            project_id,
            project_node_id,
            title,
            description,
            created_at,
            updated_at,
        }
    }
}

/// Individual project item/resource within a GitHub project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectResource {
    pub resource_id: String,
    pub title: Option<String>,
    pub author: User,
    pub assignees: Vec<User>,
    pub labels: Vec<Label>,
    pub state: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub column_name: Option<String>,
    pub custom_field_values: Vec<ProjectCustomFieldValue>,
    /// Reference to the original issue or PR
    pub original_resource: ProjectOriginalResource,
}

/// Type of resource in a project
/// Reference to the original resource (issue or PR)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectOriginalResource {
    /// Reference to an issue
    Issue(IssueId),
    /// Reference to a pull request
    PullRequest(PullRequestId),
    /// Draft issue exists only in project
    DraftIssue,
}

/// Custom field definition for a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCustomField {
    pub field_id: String,
    pub field_name: String,
    pub field_type: ProjectCustomFieldType,
    pub options: Vec<String>,
}

/// Type of custom field in a project
#[derive(
    Debug, Clone, Serialize, Deserialize, JsonSchema, EnumString, Display, AsRefStr, ValueEnum,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ProjectCustomFieldType {
    /// Text field
    Text,
    /// Number field
    Number,
    /// Date field
    Date,
    /// Single select field
    SingleSelect,
    /// Multi select field
    MultiSelect,
}

/// Value of a custom field for a specific resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCustomFieldValue {
    pub field_id: String,
    pub field_name: String,
    pub value: ProjectFieldValue,
}

/// Actual value of a custom field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectFieldValue {
    /// Text value
    Text(String),
    /// Number value
    Number(f64),
    /// Date value
    Date(DateTime<Utc>),
    /// Single select value
    SingleSelect(String),
    /// Multi select values
    MultiSelect(Vec<String>),
}

impl ProjectFieldValue {
    /// Parse a string value into the appropriate ProjectFieldValue based on field type
    ///
    /// # Arguments
    /// * `field_type` - The custom field type determining how to parse the value
    /// * `value` - The string value to parse
    ///
    /// # Returns
    /// Returns the parsed ProjectFieldValue or an error if parsing fails
    ///
    /// # Examples
    /// ```
    /// use github_edit::types::project::{ProjectCustomFieldType, ProjectFieldValue};
    ///
    /// let text_value = ProjectFieldValue::from_string_with_type(
    ///     &ProjectCustomFieldType::Text,
    ///     "Hello World"
    /// ).unwrap();
    ///
    /// let number_value = ProjectFieldValue::from_string_with_type(
    ///     &ProjectCustomFieldType::Number,
    ///     "42.5"
    /// ).unwrap();
    /// ```
    pub fn from_string_with_type(
        field_type: &ProjectCustomFieldType,
        value: &str,
    ) -> anyhow::Result<Self> {
        use anyhow::anyhow;

        match field_type {
            ProjectCustomFieldType::Text => Ok(ProjectFieldValue::Text(value.to_string())),
            ProjectCustomFieldType::Number => {
                let num = value
                    .parse::<f64>()
                    .map_err(|e| anyhow!("Failed to parse number value '{}': {}", value, e))?;
                Ok(ProjectFieldValue::Number(num))
            }
            ProjectCustomFieldType::Date => {
                let date = value
                    .parse::<DateTime<Utc>>()
                    .map_err(|e| anyhow!("Failed to parse date value '{}': {}", value, e))?;
                Ok(ProjectFieldValue::Date(date))
            }
            ProjectCustomFieldType::SingleSelect => {
                Ok(ProjectFieldValue::SingleSelect(value.to_string()))
            }
            ProjectCustomFieldType::MultiSelect => {
                // Parse comma-separated values for multi-select
                let values: Vec<String> = value
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                Ok(ProjectFieldValue::MultiSelect(values))
            }
        }
    }
}

impl ProjectResource {
    /// Create new project resource
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        resource_id: String,
        title: String,
        author: String,
        assignees: Vec<String>,
        labels: Vec<String>,
        state: String,
        column_name: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        original_resource: ProjectOriginalResource,
    ) -> Self {
        Self {
            resource_id,
            title: Some(title),
            author: User::from(author),
            assignees: assignees.into_iter().map(User::from).collect(),
            labels: labels.into_iter().map(Label::from).collect(),
            state,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
            column_name,
            custom_field_values: Vec::new(),
            original_resource,
        }
    }

    /// Get the original issue ID if this resource is an issue
    pub fn as_issue_id(&self) -> Option<&IssueId> {
        match &self.original_resource {
            ProjectOriginalResource::Issue(issue_id) => Some(issue_id),
            _ => None,
        }
    }

    /// Get the original pull request ID if this resource is a PR
    pub fn as_pull_request_id(&self) -> Option<&PullRequestId> {
        match &self.original_resource {
            ProjectOriginalResource::PullRequest(pr_id) => Some(pr_id),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectItemId(pub String);

impl ProjectItemId {
    /// Create new project item ID
    pub fn new(id: String) -> Self {
        Self(id)
    }

    /// Get the string value
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ProjectItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFieldId(pub String);

impl ProjectFieldId {
    /// Create new project field ID
    pub fn new(id: String) -> Self {
        Self(id)
    }

    /// Get the string value
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ProjectFieldId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFieldName(pub String);

impl ProjectFieldName {
    /// Create new project field name
    pub fn new(name: String) -> Self {
        Self(name)
    }

    /// Get the string value
    pub fn value(&self) -> &str {
        &self.0
    }
}
