use serde::{Deserialize, Serialize};

/// Video Placement Types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum VideoPlacementType {
    /// In-Stream: Played before, during or after the streaming video content that the consumer has requested (Pre-roll, Mid-roll, Post-roll).
    InStream = 1,
    /// In-Banner: Exists within a web banner that leverages the banner space to deliver a video experience as opposed to another static or rich media format.
    InBanner = 2,
    /// In-Article: Loads and plays dynamically between paragraphs of editorial content; existing as a standalone branded message.
    InArticle = 3,
    /// In-Feed: Found in content, social, or product feeds.
    InFeed = 4,
    /// Interstitial/Slider/Floating: Covers the entire or a portion of screen area, but is always on screen while displayed (i.e. cannot be scrolled out of view).
    InterstitialSliderFloating = 5,
}

