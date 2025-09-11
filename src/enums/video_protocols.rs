use serde::{Deserialize, Serialize};

/// Video Bid Response Protocols
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum VideoBidResponseProtocol {
    /// VAST 1.0
    Vast1 = 1,
    /// VAST 2.0
    Vast2 = 2,
    /// VAST 3.0
    Vast3 = 3,
    /// VAST 1.0 Wrapper
    Vast1Wrapper = 4,
    /// VAST 2.0 Wrapper
    Vast2Wrapper = 5,
    /// VAST 3.0 Wrapper
    Vast3Wrapper = 6,
    /// VAST 4.0
    Vast4 = 7,
    /// VAST 4.0 Wrapper
    Vast4Wrapper = 8,
    /// DAAST 1.0
    Daast1 = 9,
    /// DAAST 1.0 Wrapper
    Daast1Wrapper = 10,
}

