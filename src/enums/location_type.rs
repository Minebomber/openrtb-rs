use serde::{Deserialize, Serialize};

/// Location Type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum LocationType {
    /// GPS/Location Services
    GpsLocationServices = 1,
    /// IP Address
    IpAddress = 2,
    /// User provided (e.g. registration data)
    UserProvided = 3,
}

