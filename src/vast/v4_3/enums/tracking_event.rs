//! Types of tracking events as defined in VAST 4.3

use super::*;
use serde::{Deserialize, Serialize};

/// Types of tracking events as defined in VAST 4.3
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrackingEvent {
    /// Triggered when the creative is viewed (visibility requirements met)
    #[serde(rename = "creativeView")]
    CreativeView,
    /// Start of video playback
    #[serde(rename = "start")]
    Start,
    /// 25% of video duration reached
    #[serde(rename = "firstQuartile")]
    FirstQuartile,
    /// 50% of video duration reached
    #[serde(rename = "midpoint")]
    Midpoint,
    /// 75% of video duration reached
    #[serde(rename = "thirdQuartile")]
    ThirdQuartile,
    /// Video playback completed
    #[serde(rename = "complete")]
    Complete,
    /// User muted the video
    #[serde(rename = "mute")]
    Mute,
    /// User unmuted the video
    #[serde(rename = "unmute")]
    Unmute,
    /// User paused the video
    #[serde(rename = "pause")]
    Pause,
    /// User resumed the video
    #[serde(rename = "resume")]
    Resume,
    /// User rewound the video
    #[serde(rename = "rewind")]
    Rewind,
    /// User skipped the ad
    #[serde(rename = "skip")]
    Skip,
    /// User clicked the player bar slider
    #[serde(rename = "playerExpand")]
    PlayerExpand,
    /// User collapsed the player from expanded state
    #[serde(rename = "playerCollapse")]
    PlayerCollapse,
    /// Triggered when not at least 50% of the pixels in the creative are viewable
    #[serde(rename = "notUsed")]
    NotUsed,
    /// User entered fullscreen mode
    #[serde(rename = "fullscreen")]
    Fullscreen,
    /// User exited fullscreen mode
    #[serde(rename = "exitFullscreen")]
    ExitFullscreen,
    /// Ad was accepted for playback
    #[serde(rename = "acceptInvitation")]
    AcceptInvitation,
    /// Ad playback was initiated
    #[serde(rename = "adExpand")]
    AdExpand,
    /// Ad was collapsed from expanded state
    #[serde(rename = "adCollapse")]
    AdCollapse,
    /// Ad was minimized
    #[serde(rename = "minimize")]
    Minimize,
    /// Ad interaction occurred
    #[serde(rename = "adInteraction")]
    AdInteraction,
    /// User closed the ad
    #[serde(rename = "close")]
    Close,
    /// User closed linear creative
    #[serde(rename = "closeLinear")]
    CloseLinear,
    /// Overlay was viewed
    #[serde(rename = "overlayViewDuration")]
    OverlayViewDuration,
    /// Progress through video at specific offset
    #[serde(rename = "progress")]
    Progress,
    /// Other events as needed
    #[serde(rename = "otherAdInteraction")]
    OtherAdInteraction,
}

impl std::fmt::Display for TrackingEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TrackingEvent::CreativeView => "creativeView",
            TrackingEvent::Start => "start",
            TrackingEvent::FirstQuartile => "firstQuartile",
            TrackingEvent::Midpoint => "midpoint",
            TrackingEvent::ThirdQuartile => "thirdQuartile",
            TrackingEvent::Complete => "complete",
            TrackingEvent::Mute => "mute",
            TrackingEvent::Unmute => "unmute",
            TrackingEvent::Pause => "pause",
            TrackingEvent::Resume => "resume",
            TrackingEvent::Rewind => "rewind",
            TrackingEvent::Skip => "skip",
            TrackingEvent::PlayerExpand => "playerExpand",
            TrackingEvent::PlayerCollapse => "playerCollapse",
            TrackingEvent::NotUsed => "notUsed",
            TrackingEvent::Fullscreen => "fullscreen",
            TrackingEvent::ExitFullscreen => "exitFullscreen",
            TrackingEvent::AcceptInvitation => "acceptInvitation",
            TrackingEvent::AdExpand => "adExpand",
            TrackingEvent::AdCollapse => "adCollapse",
            TrackingEvent::Minimize => "minimize",
            TrackingEvent::AdInteraction => "adInteraction",
            TrackingEvent::Close => "close",
            TrackingEvent::CloseLinear => "closeLinear",
            TrackingEvent::OverlayViewDuration => "overlayViewDuration",
            TrackingEvent::Progress => "progress",
            TrackingEvent::OtherAdInteraction => "otherAdInteraction",
        };
        write!(f, "{}", s)
    }
}

impl std::str::FromStr for TrackingEvent {
    type Err = VastError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "creativeView" => TrackingEvent::CreativeView,
            "start" => TrackingEvent::Start,
            "firstQuartile" => TrackingEvent::FirstQuartile,
            "midpoint" => TrackingEvent::Midpoint,
            "thirdQuartile" => TrackingEvent::ThirdQuartile,
            "complete" => TrackingEvent::Complete,
            "mute" => TrackingEvent::Mute,
            "unmute" => TrackingEvent::Unmute,
            "pause" => TrackingEvent::Pause,
            "resume" => TrackingEvent::Resume,
            "rewind" => TrackingEvent::Rewind,
            "skip" => TrackingEvent::Skip,
            "playerExpand" => TrackingEvent::PlayerExpand,
            "playerCollapse" => TrackingEvent::PlayerCollapse,
            "notUsed" => TrackingEvent::NotUsed,
            "fullscreen" => TrackingEvent::Fullscreen,
            "exitFullscreen" => TrackingEvent::ExitFullscreen,
            "acceptInvitation" => TrackingEvent::AcceptInvitation,
            "adExpand" => TrackingEvent::AdExpand,
            "adCollapse" => TrackingEvent::AdCollapse,
            "minimize" => TrackingEvent::Minimize,
            "adInteraction" => TrackingEvent::AdInteraction,
            "close" => TrackingEvent::Close,
            "closeLinear" => TrackingEvent::CloseLinear,
            "overlayViewDuration" => TrackingEvent::OverlayViewDuration,
            "progress" => TrackingEvent::Progress,
            "otherAdInteraction" => TrackingEvent::OtherAdInteraction,
            _ => return Err(VastError::InvalidTrackingEvent(s.to_string())),
        })
    }
}

