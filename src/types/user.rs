//! User and participant types for Git resources
//!
//! This module provides types for user identification and participation
//! in Git resources like issues and pull requests.

use serde::{Deserialize, Serialize};

/// User identifier wrapper type for GitHub usernames
///
/// This type provides type-safe user identification for GitHub users,
/// storing the username and avatar URL for complete identification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct User {
    pub username: String,
    pub avatar_url: Option<String>,
}

impl User {
    /// Creates a new User with the specified username and optional avatar URL
    pub fn new(username: String, avatar_url: Option<String>) -> Self {
        Self {
            username,
            avatar_url,
        }
    }

    /// Get the username as a string
    pub fn as_str(&self) -> &str {
        &self.username
    }

    /// Get the username (for backward compatibility)
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Get the avatar URL if available
    pub fn avatar_url(&self) -> Option<&str> {
        self.avatar_url.as_deref()
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.username)
    }
}

impl From<&str> for User {
    fn from(s: &str) -> Self {
        User::new(s.to_string(), None)
    }
}

impl From<String> for User {
    fn from(s: String) -> Self {
        User::new(s, None)
    }
}

impl PartialEq<str> for User {
    fn eq(&self, other: &str) -> bool {
        self.username == other
    }
}

impl PartialEq<&str> for User {
    fn eq(&self, other: &&str) -> bool {
        self.username == *other
    }
}
