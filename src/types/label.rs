use serde::{Deserialize, Serialize};

/// GitHub label with comprehensive metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Label {
    pub name: String,
    pub color: Option<String>,
    pub description: Option<String>,
}

impl Label {
    /// Create a new label with name and color
    pub fn new(name: String, color: Option<String>) -> Self {
        Label {
            name,
            color,
            description: None,
        }
    }

    /// Create a new label with name, color, and description
    pub fn new_with_description(
        name: String,
        color: Option<String>,
        description: Option<String>,
    ) -> Self {
        Label {
            name,
            color,
            description,
        }
    }

    /// Get the label name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the label color
    pub fn color(&self) -> &str {
        self.color.as_deref().unwrap_or("ffffff")
    }

    /// Get the label description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl From<String> for Label {
    fn from(name: String) -> Self {
        Label::new(name, None)
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
