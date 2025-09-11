use serde::{Deserialize, Serialize};

/// Playback Methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum PlaybackMethod {
    /// Initiates on Page Load with Sound On
    AutoPlaySoundOn = 1,
    /// Initiates on Page Load with Sound Off by Default
    AutoPlaySoundOff = 2,
    /// Initiates on Click with Sound On
    ClickToPlay = 3,
    /// Initiates on Mouse-Over with Sound On
    MouseOver = 4,
    /// Initiates on Entering Viewport with Sound On
    EnterViewportSoundOn = 5,
    /// Initiates on Entering Viewport with Sound Off by Default
    EnterViewportSoundOff = 6,
}

