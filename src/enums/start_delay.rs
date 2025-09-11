use serde::{Deserialize, Serialize};

/// Start Delay
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum StartDelay {
    /// Pre-Roll
    PreRoll = 0,
    /// Generic Mid-Roll
    GenericMidRoll = -1,
    /// Generic Post-Roll
    GenericPostRoll = -2,
}

