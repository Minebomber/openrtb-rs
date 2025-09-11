use serde::{Deserialize, Serialize};

/// Device Types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum DeviceType {
    /// Mobile/Tablet - General
    MobileTablet = 1,
    /// Personal Computer
    PersonalComputer = 2,
    /// Connected TV
    ConnectedTv = 3,
    /// Phone
    Phone = 4,
    /// Tablet
    Tablet = 5,
    /// Connected Device
    ConnectedDevice = 6,
    /// Set Top Box
    SetTopBox = 7,
}

