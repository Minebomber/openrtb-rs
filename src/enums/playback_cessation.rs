use serde::{Deserialize, Serialize};

/// Playback Cessation Modes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum PlaybackCessationMode {
    /// On Video Completion or when Terminated by User
    VideoCompletionOrUserTerminated = 1,
    /// On Leaving Viewport or when Terminated by User
    LeavingViewportOrUserTerminated = 2,
    /// On Leaving Viewport Continues as a Floating/Slider Unit until Video Completion or when Terminated by User
    LeavingViewportFloatingUnit = 3,
}

