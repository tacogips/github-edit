//! Core type system and domain definitions
//!
//! This module provides the central type definitions for the Github Edit system,
//! following domain-driven design principles. All types are strongly-typed and
//! provide comprehensive validation and conversion capabilities.

pub mod issue;
pub mod label;
pub mod milestone;
pub mod project;
pub mod pull_request;
pub mod repository;
pub mod user;

pub use issue::*;
pub use label::*;
pub use milestone::*;
pub use project::*;
pub use pull_request::*;
pub use repository::*;
pub use user::*;
