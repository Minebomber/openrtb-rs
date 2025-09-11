use serde::{Deserialize, Serialize};

/// Video Linearity
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum VideoLinearity {
    /// Linear / In-Stream
    Linear = 1,
    /// Non-Linear / Overlay
    NonLinear = 2,
}

