//! Skip offset enumeration

use super::super::*;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Skip offset can be percentage or time
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SkipOffset {
    /// Time offset in HH:MM:SS format
    Time(Duration),
    /// Percentage offset (e.g., "25%")
    Percentage(String),
}

impl fmt::Display for SkipOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SkipOffset::Time(d) => write!(f, "{}", d),
            SkipOffset::Percentage(p) => write!(f, "{}", p),
        }
    }
}

impl std::str::FromStr for SkipOffset {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('%') {
            Ok(SkipOffset::Percentage(s.to_string()))
        } else {
            Ok(SkipOffset::Time(Duration {
                value: s.to_string(),
            }))
        }
    }
}
