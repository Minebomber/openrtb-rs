use serde::{Deserialize, Serialize};

/// Ad Position
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum AdPosition {
    /// Unknown
    Unknown = 0,
    /// Above the Fold
    AboveTheFold = 1,
    /// DEPRECATED - May or may not be initially visible depending on screen size/resolution.
    MayOrMayNotBeVisible = 2,
    /// Below the Fold
    BelowTheFold = 3,
    /// Header
    Header = 4,
    /// Footer
    Footer = 5,
    /// Sidebar
    Sidebar = 6,
    /// Full Screen
    FullScreen = 7,
}

