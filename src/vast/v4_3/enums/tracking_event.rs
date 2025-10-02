//! Types of tracking events as defined in VAST 4.3

use super::super::*;

/// Types of tracking events as defined in VAST 4.3
#[derive(Debug, Clone, PartialEq)]
pub enum TrackingEvent {
    /// Triggered when the creative is viewed (visibility requirements met)
    CreativeView,
    /// Start of video playback
    Start,
    /// 25% of video duration reached
    FirstQuartile,
    /// 50% of video duration reached
    Midpoint,
    /// 75% of video duration reached
    ThirdQuartile,
    /// Video playback completed
    Complete,
    /// User muted the video
    Mute,
    /// User unmuted the video
    Unmute,
    /// User paused the video
    Pause,
    /// User resumed the video
    Resume,
    /// User rewound the video
    Rewind,
    /// User skipped the ad
    Skip,
    /// User clicked the player bar slider
    PlayerExpand,
    /// User collapsed the player from expanded state
    PlayerCollapse,
    /// Triggered when not at least 50% of the pixels in the creative are viewable
    NotUsed,
    /// User entered fullscreen mode
    Fullscreen,
    /// User exited fullscreen mode
    ExitFullscreen,
    /// Ad was accepted for playback
    AcceptInvitation,
    /// Ad playback was initiated
    AdExpand,
    /// Ad was collapsed from expanded state
    AdCollapse,
    /// Ad was minimized
    Minimize,
    /// Ad interaction occurred
    AdInteraction,
    /// User closed the ad
    Close,
    /// User closed linear creative
    CloseLinear,
    /// Overlay was viewed
    OverlayViewDuration,
    /// Progress through video at specific offset
    Progress,
    /// Other events as needed
    OtherAdInteraction,
}

impl hard_xml::XmlWrite for TrackingEvent {
    fn to_writer<W: std::io::Write>(
        &self,
        _writer: &mut hard_xml::XmlWriter<W>,
    ) -> hard_xml::XmlResult<()> {
        Ok(())
    }
}

impl hard_xml::XmlRead<'_> for TrackingEvent {
    fn from_reader(_reader: &mut hard_xml::XmlReader<'_>) -> hard_xml::XmlResult<Self> {
        Ok(TrackingEvent::Start)
    }
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "creativeView" => Ok(TrackingEvent::CreativeView),
            "start" => Ok(TrackingEvent::Start),
            "firstQuartile" => Ok(TrackingEvent::FirstQuartile),
            "midpoint" => Ok(TrackingEvent::Midpoint),
            "thirdQuartile" => Ok(TrackingEvent::ThirdQuartile),
            "complete" => Ok(TrackingEvent::Complete),
            "mute" => Ok(TrackingEvent::Mute),
            "unmute" => Ok(TrackingEvent::Unmute),
            "pause" => Ok(TrackingEvent::Pause),
            "resume" => Ok(TrackingEvent::Resume),
            "rewind" => Ok(TrackingEvent::Rewind),
            "skip" => Ok(TrackingEvent::Skip),
            "playerExpand" => Ok(TrackingEvent::PlayerExpand),
            "playerCollapse" => Ok(TrackingEvent::PlayerCollapse),
            "notUsed" => Ok(TrackingEvent::NotUsed),
            "fullscreen" => Ok(TrackingEvent::Fullscreen),
            "exitFullscreen" => Ok(TrackingEvent::ExitFullscreen),
            "acceptInvitation" => Ok(TrackingEvent::AcceptInvitation),
            "adExpand" => Ok(TrackingEvent::AdExpand),
            "adCollapse" => Ok(TrackingEvent::AdCollapse),
            "minimize" => Ok(TrackingEvent::Minimize),
            "adInteraction" => Ok(TrackingEvent::AdInteraction),
            "close" => Ok(TrackingEvent::Close),
            "closeLinear" => Ok(TrackingEvent::CloseLinear),
            "overlayViewDuration" => Ok(TrackingEvent::OverlayViewDuration),
            "progress" => Ok(TrackingEvent::Progress),
            "otherAdInteraction" => Ok(TrackingEvent::OtherAdInteraction),
            _ => Err(format!("Unknown tracking event: {}", s)),
        }
    }
}
