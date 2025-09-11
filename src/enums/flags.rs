use serde::{Deserialize, Serialize};

/// Test Mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum TestMode {
    /// Live mode (auctions are billable)
    Live = 0,
    /// Test mode (auctions are not billable)
    Test = 1,
}

/// All Impressions Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum AllImpsFlag {
    /// No or unknown
    NoOrUnknown = 0,
    /// Yes, impressions offered represent all that are available
    Yes = 1,
}

/// Final Decision
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum FinalDecision {
    /// Exchange makes the final decision
    Exchange = 0,
    /// Upstream source makes the final decision
    UpstreamSource = 1,
}

/// COPPA Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum CoppaFlag {
    /// Not subject to COPPA
    No = 0,
    /// Subject to COPPA
    Yes = 1,
}

/// GDPR Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum GdprFlag {
    /// Not subject to GDPR
    No = 0,
    /// Subject to GDPR
    Yes = 1,
}

/// Interstitial Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum InterstitialFlag {
    /// Not interstitial
    NotInterstitial = 0,
    /// Interstitial or full screen
    Interstitial = 1,
}

/// Click Browser Type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum ClickBrowser {
    /// Embedded browser
    Embedded = 0,
    /// Native browser
    Native = 1,
}

/// Secure Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum SecureFlag {
    /// Non-secure (HTTP)
    NonSecure = 0,
    /// Secure (HTTPS)
    Secure = 1,
}

/// Rewarded Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum RewardedFlag {
    /// User does not receive reward
    No = 0,
    /// User receives reward for viewing
    Yes = 1,
}

/// Server-Side Ad Insertion
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum ServerSideAdInsertion {
    /// Status unknown
    StatusUnknown = 0,
    /// All client-side (not server-side)
    AllClientSide = 1,
    /// Assets stitched server-side but tracking pixels fired client-side
    AssetsServerSideTrackingClientSide = 2,
    /// All server-side
    AllServerSide = 3,
}

/// Top Frame Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum TopFrameFlag {
    /// In iframe
    InIframe = 0,
    /// In top frame
    InTopFrame = 1,
}

/// Video Banner Companion Mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum VideoBannerMode {
    /// Concurrent with video
    Concurrent = 0,
    /// End card after video
    EndCard = 1,
}

/// Skip Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum SkipFlag {
    /// Cannot be skipped
    NoSkip = 0,
    /// Can be skipped
    Skippable = 1,
}

/// Boxing Allowed Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum BoxingAllowedFlag {
    /// Boxing not allowed
    NotAllowed = 0,
    /// Boxing allowed
    Allowed = 1,
}

/// Stitched Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum StitchedFlag {
    /// Delivered independently
    Independent = 0,
    /// Stitched with content
    Stitched = 1,
}

/// Do Not Track Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum DoNotTrackFlag {
    /// Tracking unrestricted
    TrackingUnrestricted = 0,
    /// Do not track
    DoNotTrack = 1,
}

/// Limit Ad Tracking Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum LimitAdTrackingFlag {
    /// Tracking unrestricted
    TrackingUnrestricted = 0,
    /// Tracking limited
    TrackingLimited = 1,
}

/// JavaScript Support Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum JavaScriptFlag {
    /// JavaScript not supported
    NotSupported = 0,
    /// JavaScript supported
    Supported = 1,
}

/// Geo Fetch Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum GeoFetchFlag {
    /// Geolocation API not available
    NotAvailable = 0,
    /// Geolocation API available
    Available = 1,
}

/// Mobile Optimized Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum MobileOptimizedFlag {
    /// Not optimized for mobile
    NotOptimized = 0,
    /// Optimized for mobile
    Optimized = 1,
}

/// Privacy Policy Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum PrivacyPolicyFlag {
    /// No privacy policy
    No = 0,
    /// Has privacy policy
    Yes = 1,
}

/// Paid App Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum PaidAppFlag {
    /// App is free
    Free = 0,
    /// App is paid
    Paid = 1,
}

/// Live Stream Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum LiveStreamFlag {
    /// Content is not live
    NotLive = 0,
    /// Content is being streamed live
    Live = 1,
}

/// Source Relationship
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum SourceRelationship {
    /// Indirect relationship
    Indirect = 0,
    /// Direct relationship
    Direct = 1,
}

/// Embeddable Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum EmbeddableFlag {
    /// Content is not embeddable
    NotEmbeddable = 0,
    /// Content is embeddable
    Embeddable = 1,
}

/// Mobile Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum MobileFlag {
    /// Not mobile
    NotMobile = 0,
    /// Is mobile
    Mobile = 1,
}

/// Group Flag
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum GroupFlag {
    /// Impressions can be won individually
    Individual = 0,
    /// Impressions must be won or lost as a group
    Group = 1,
}

