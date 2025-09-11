use serde::{Deserialize, Serialize};

/// Location Service
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum LocationService {
    /// ip2location
    Ip2Location = 1,
    /// Neustar (Quova)
    Neustar = 2,
    /// MaxMind
    MaxMind = 3,
    /// NetAcuity (Digital Element)
    NetAcuity = 4,
}

