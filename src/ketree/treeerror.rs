//! Simple Error types for use with Tree.

use std::fmt::{self, Display};
use std::error;

/// Error struct.
#[derive(Debug)]
pub struct TreeError {
    kind: TreeErrorKind,
    message: String,
}

impl TreeError {
    /// Convenience function to create an error.
    pub fn create(tek: TreeErrorKind) -> TreeError {
        match tek {
            TreeErrorKind::VarNotFound => TreeError {
                kind: TreeErrorKind::VarNotFound,
                message: "Found variable in tree but not map".to_string(),
            },
            TreeErrorKind::TreeNotInScope => TreeError {
                kind: TreeErrorKind::TreeNotInScope,
                message: "Tree not defined in scope - use (define $name $tree)".to_string(),
            },
        }
    }
}

impl Display for TreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Enum for error type.
#[derive(Debug)]
pub enum TreeErrorKind {
    VarNotFound,
    TreeNotInScope,
}

impl error::Error for TreeError {
    fn description(&self) -> &str {
        &self.message
    }
}

