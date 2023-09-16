mod group;

use regex::Regex;
use thiserror::Error;

pub use self::group::Group;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum CaptureError {
    #[error("The regex {regex} did not match {string:?}")]
    NoMatch { regex: String, string: String },
    #[error("Could not find capture group {group}")]
    CaptureGroupNotFound { group: Group },
    #[error("Failed to convert capture group {group}: {error}")]
    ConversionError { group: Group, error: String },
}

impl CaptureError {
    pub(crate) fn no_match(regex: &Regex, s: &str) -> Self {
        CaptureError::NoMatch {
            regex: regex.to_string(),
            string: s.to_owned(),
        }
    }
    pub(crate) fn capture_group_not_found(group: impl Into<Group>) -> Self {
        CaptureError::CaptureGroupNotFound {
            group: group.into(),
        }
    }
    pub(crate) fn conversion_error<Err: ToString>(group: impl Into<Group>, error: Err) -> Self {
        CaptureError::ConversionError {
            group: group.into(),
            error: error.to_string(),
        }
    }
}

pub type Result<A, E = CaptureError> = std::result::Result<A, E>;
