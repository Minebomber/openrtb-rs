use serde::{Deserialize, Serialize};

/// Expandable Direction
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum ExpandableDirection {
    /// Left
    Left = 1,
    /// Right
    Right = 2,
    /// Up
    Up = 3,
    /// Down
    Down = 4,
    /// Full Screen
    FullScreen = 5,
}

