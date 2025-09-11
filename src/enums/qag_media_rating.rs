use serde::{Deserialize, Serialize};

/// QAG Media Ratings
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum QagMediaRating {
    /// All Audiences
    AllAudiences = 1,
    /// Everyone Over 12
    EveryoneOver12 = 2,
    /// Mature Audiences
    MatureAudiences = 3,
}

/// IQG Media Ratings
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum IqgMediaRating {
    /// All Audiences
    AllAudiences = 1,
    /// Everyone Over 12
    EveryoneOver12 = 2,
    /// Mature Audiences
    MatureAudiences = 3,
}

