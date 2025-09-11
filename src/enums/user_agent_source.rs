use serde::{Deserialize, Serialize};

/// User Agent Source
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum UserAgentSource {
    /// Unknown or higher than 3
    Unknown = 0,
    /// User agent string in User-Agent header
    UserAgentString = 1,
    /// User agent data from User-Agent Client Hints
    UserAgentClientHints = 2,
    /// User agent string in User-Agent header enhanced with user agent data from User-Agent Client Hints
    Enhanced = 3,
}

