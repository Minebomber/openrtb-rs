use serde::{Deserialize, Serialize};

/// API Frameworks
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum ApiFramework {
    /// VPAID 1.0
    Vpaid1 = 1,
    /// VPAID 2.0
    Vpaid2 = 2,
    /// MRAID-1
    Mraid1 = 3,
    /// ORMMA
    Ormma = 4,
    /// MRAID-2
    Mraid2 = 5,
    /// MRAID-3
    Mraid3 = 6,
    /// OMID-1
    Omid1 = 7,
}

