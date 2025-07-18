//! Milestone domain types
//!
//! This module contains the Milestone domain types for GitHub milestones.

use crate::types::repository::MilestoneNumber;
use chrono::{DateTime, Utc};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// Milestone state enumeration
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum, Display, EnumString,
)]
#[strum(serialize_all = "lowercase")]
pub enum MilestoneState {
    /// Milestone is open and accepting issues
    #[value(name = "open")]
    Open,
    /// Milestone is closed
    #[value(name = "closed")]
    Closed,
}

/// Complete milestone information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    /// The numeric milestone identifier
    pub id: MilestoneNumber,
    /// The milestone title
    pub title: String,
    /// Optional description text
    pub description: Option<String>,
    /// Current state of the milestone
    pub state: MilestoneState,
    /// Number of open issues associated with this milestone
    pub open_issues: u32,
    /// Number of closed issues associated with this milestone
    pub closed_issues: u32,
    /// Due date for the milestone
    pub due_on: Option<DateTime<Utc>>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// Closure timestamp (if closed)
    pub closed_at: Option<DateTime<Utc>>,
}

impl Milestone {
    /// Create a new milestone
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: MilestoneNumber,
        title: String,
        description: Option<String>,
        state: MilestoneState,
        open_issues: u32,
        closed_issues: u32,
        due_on: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        closed_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            state,
            open_issues,
            closed_issues,
            due_on,
            created_at,
            updated_at,
            closed_at,
        }
    }

    /// Get milestone ID
    pub fn id(&self) -> MilestoneNumber {
        self.id
    }

    /// Get milestone title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get milestone description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Get milestone state
    pub fn state(&self) -> MilestoneState {
        self.state
    }

    /// Get total issue count (open + closed)
    pub fn total_issues(&self) -> u32 {
        self.open_issues + self.closed_issues
    }

    /// Calculate completion percentage
    pub fn completion_percentage(&self) -> f64 {
        let total = self.total_issues();
        if total == 0 {
            0.0
        } else {
            (self.closed_issues as f64 / total as f64) * 100.0
        }
    }

    /// Check if milestone is overdue
    pub fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_on {
            Utc::now() > due_date && self.state == MilestoneState::Open
        } else {
            false
        }
    }
}
