use serde::{Deserialize, Serialize};

/// Connection Types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum ConnectionType {
    /// Unknown
    Unknown = 0,
    /// Ethernet
    Ethernet = 1,
    /// WIFI
    Wifi = 2,
    /// Cellular Network  Unknown Generation
    CellularUnknown = 3,
    /// Cellular Network  2G
    Cellular2G = 4,
    /// Cellular Network  3G
    Cellular3G = 5,
    /// Cellular Network  4G
    Cellular4G = 6,
}

