use serde::{Deserialize, Serialize};

/// Content Delivery Methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum ContentDeliveryMethod {
    /// Streaming
    Streaming = 1,
    /// Progressive
    Progressive = 2,
    /// Download
    Download = 3,
}

