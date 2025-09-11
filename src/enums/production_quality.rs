use serde::{Deserialize, Serialize};

/// Production Quality
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum ProductionQuality {
    /// Unknown
    Unknown = 0,
    /// Professionally Produced
    Professional = 1,
    /// Prosumer
    Prosumer = 2,
    /// User Generated (UGC)
    UserGenerated = 3,
}

