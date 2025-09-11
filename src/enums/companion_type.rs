use serde::{Deserialize, Serialize};

/// Companion Types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum CompanionType {
    /// Static Resource
    StaticResource = 1,
    /// HTML Resource
    HtmlResource = 2,
    /// iframe Resource
    IframeResource = 3,
}

