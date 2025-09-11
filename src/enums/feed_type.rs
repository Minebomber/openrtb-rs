use serde::{Deserialize, Serialize};

/// Feed Types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum FeedType {
    /// Music Service
    MusicService = 1,
    /// FM/AM Broadcast
    FmAmBroadcast = 2,
    /// Podcast
    Podcast = 3,
}

