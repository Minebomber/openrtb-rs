use serde::{Deserialize, Serialize};

/// Creative Attributes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum CreativeAttribute {
    /// Audio Ad (Auto-Play)
    AudioAdAutoPlay = 1,
    /// Audio Ad (User Initiated)
    AudioAdUserInitiated = 2,
    /// Expandable (Automatic)
    ExpandableAutomatic = 3,
    /// Expandable (User Initiated - Click)
    ExpandableUserInitiatedClick = 4,
    /// Expandable (User Initiated - Rollover)
    ExpandableUserInitiatedRollover = 5,
    /// In-Banner Video Ad (Auto-Play)
    InBannerVideoAdAutoPlay = 6,
    /// In-Banner Video Ad (User Initiated)
    InBannerVideoAdUserInitiated = 7,
    /// Pop (e.g., Over, Under, or upon Exit)
    Pop = 8,
    /// Provocative or Suggestive Imagery
    ProvocativeOrSuggestiveImagery = 9,
    /// Shaky, Flashing, Flickering, Extreme Animation, Smileys
    ShakyFlashingFlickering = 10,
    /// Surveys
    Surveys = 11,
    /// Text Only
    TextOnly = 12,
    /// User Interactive (e.g., Embedded Games)
    UserInteractive = 13,
    /// Windows Dialog or Alert Style
    WindowsDialog = 14,
    /// Has Audio On/Off Button
    HasAudioOnOffButton = 15,
    /// Ad Provides Skip Button (e.g. VPAID-rendered skip button on pre-roll video)
    AdProvidesSkipButton = 16,
    /// Adobe Flash
    AdobeFlash = 17,
}

