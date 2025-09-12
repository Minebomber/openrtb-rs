use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Test Mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TestMode {
    /// Live mode (auctions are billable)
    Live,
    /// Test mode (auctions are not billable)
    Test,
}

impl Serialize for TestMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            TestMode::Live => serializer.serialize_u32(0),
            TestMode::Test => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for TestMode {
    fn deserialize<D>(deserializer: D) -> Result<TestMode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(TestMode::Live),
            1 => Ok(TestMode::Test),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid TestMode value: {}",
                value
            ))),
        }
    }
}

/// All Impressions Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AllImpsFlag {
    /// No or unknown
    NoOrUnknown,
    /// Yes, impressions offered represent all that are available
    Yes,
}

impl Serialize for AllImpsFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            AllImpsFlag::NoOrUnknown => serializer.serialize_u32(0),
            AllImpsFlag::Yes => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for AllImpsFlag {
    fn deserialize<D>(deserializer: D) -> Result<AllImpsFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(AllImpsFlag::NoOrUnknown),
            1 => Ok(AllImpsFlag::Yes),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid AllImpsFlag value: {}",
                value
            ))),
        }
    }
}

/// Final Decision
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FinalDecision {
    /// Exchange makes the final decision
    Exchange,
    /// Upstream source makes the final decision
    UpstreamSource,
}

impl Serialize for FinalDecision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            FinalDecision::Exchange => serializer.serialize_u32(0),
            FinalDecision::UpstreamSource => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for FinalDecision {
    fn deserialize<D>(deserializer: D) -> Result<FinalDecision, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(FinalDecision::Exchange),
            1 => Ok(FinalDecision::UpstreamSource),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid FinalDecision value: {}",
                value
            ))),
        }
    }
}

/// COPPA Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoppaFlag {
    /// Not subject to COPPA
    No,
    /// Subject to COPPA
    Yes,
}

impl Serialize for CoppaFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            CoppaFlag::No => serializer.serialize_u32(0),
            CoppaFlag::Yes => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for CoppaFlag {
    fn deserialize<D>(deserializer: D) -> Result<CoppaFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(CoppaFlag::No),
            1 => Ok(CoppaFlag::Yes),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid CoppaFlag value: {}",
                value
            ))),
        }
    }
}

/// GDPR Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GdprFlag {
    /// Not subject to GDPR
    No,
    /// Subject to GDPR
    Yes,
}

impl Serialize for GdprFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            GdprFlag::No => serializer.serialize_u32(0),
            GdprFlag::Yes => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for GdprFlag {
    fn deserialize<D>(deserializer: D) -> Result<GdprFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(GdprFlag::No),
            1 => Ok(GdprFlag::Yes),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid GdprFlag value: {}",
                value
            ))),
        }
    }
}

/// Interstitial Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterstitialFlag {
    /// Not interstitial
    NotInterstitial,
    /// Interstitial or full screen
    Interstitial,
}

impl Serialize for InterstitialFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            InterstitialFlag::NotInterstitial => serializer.serialize_u32(0),
            InterstitialFlag::Interstitial => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for InterstitialFlag {
    fn deserialize<D>(deserializer: D) -> Result<InterstitialFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(InterstitialFlag::NotInterstitial),
            1 => Ok(InterstitialFlag::Interstitial),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid InterstitialFlag value: {}",
                value
            ))),
        }
    }
}

/// Click Browser Type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClickBrowser {
    /// Embedded browser
    Embedded,
    /// Native browser
    Native,
}

impl Serialize for ClickBrowser {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            ClickBrowser::Embedded => serializer.serialize_u32(0),
            ClickBrowser::Native => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for ClickBrowser {
    fn deserialize<D>(deserializer: D) -> Result<ClickBrowser, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(ClickBrowser::Embedded),
            1 => Ok(ClickBrowser::Native),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid ClickBrowser value: {}",
                value
            ))),
        }
    }
}

/// Secure Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecureFlag {
    /// Non-secure (HTTP)
    NonSecure,
    /// Secure (HTTPS)
    Secure,
}

impl Serialize for SecureFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            SecureFlag::NonSecure => serializer.serialize_u32(0),
            SecureFlag::Secure => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for SecureFlag {
    fn deserialize<D>(deserializer: D) -> Result<SecureFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(SecureFlag::NonSecure),
            1 => Ok(SecureFlag::Secure),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid SecureFlag value: {}",
                value
            ))),
        }
    }
}

/// Rewarded Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RewardedFlag {
    /// User does not receive reward
    No,
    /// User receives reward for viewing
    Yes,
}

impl Serialize for RewardedFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            RewardedFlag::No => serializer.serialize_u32(0),
            RewardedFlag::Yes => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for RewardedFlag {
    fn deserialize<D>(deserializer: D) -> Result<RewardedFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(RewardedFlag::No),
            1 => Ok(RewardedFlag::Yes),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid RewardedFlag value: {}",
                value
            ))),
        }
    }
}

/// Server-Side Ad Insertion
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ServerSideAdInsertion {
    /// Status unknown
    StatusUnknown,
    /// All client-side (not server-side)
    AllClientSide,
    /// Assets stitched server-side but tracking pixels fired client-side
    AssetsServerSideTrackingClientSide,
    /// All server-side
    AllServerSide,
}

impl Serialize for ServerSideAdInsertion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            ServerSideAdInsertion::StatusUnknown => serializer.serialize_u32(0),
            ServerSideAdInsertion::AllClientSide => serializer.serialize_u32(1),
            ServerSideAdInsertion::AssetsServerSideTrackingClientSide => serializer.serialize_u32(2),
            ServerSideAdInsertion::AllServerSide => serializer.serialize_u32(3),
        }
    }
}

impl<'de> Deserialize<'de> for ServerSideAdInsertion {
    fn deserialize<D>(deserializer: D) -> Result<ServerSideAdInsertion, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(ServerSideAdInsertion::StatusUnknown),
            1 => Ok(ServerSideAdInsertion::AllClientSide),
            2 => Ok(ServerSideAdInsertion::AssetsServerSideTrackingClientSide),
            3 => Ok(ServerSideAdInsertion::AllServerSide),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid ServerSideAdInsertion value: {}",
                value
            ))),
        }
    }
}

/// Top Frame Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TopFrameFlag {
    /// In iframe
    InIframe,
    /// In top frame
    InTopFrame,
}

impl Serialize for TopFrameFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            TopFrameFlag::InIframe => serializer.serialize_u32(0),
            TopFrameFlag::InTopFrame => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for TopFrameFlag {
    fn deserialize<D>(deserializer: D) -> Result<TopFrameFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(TopFrameFlag::InIframe),
            1 => Ok(TopFrameFlag::InTopFrame),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid TopFrameFlag value: {}",
                value
            ))),
        }
    }
}

/// Video Banner Companion Mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VideoBannerMode {
    /// Concurrent with video
    Concurrent,
    /// End card after video
    EndCard,
}

impl Serialize for VideoBannerMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            VideoBannerMode::Concurrent => serializer.serialize_u32(0),
            VideoBannerMode::EndCard => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for VideoBannerMode {
    fn deserialize<D>(deserializer: D) -> Result<VideoBannerMode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(VideoBannerMode::Concurrent),
            1 => Ok(VideoBannerMode::EndCard),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid VideoBannerMode value: {}",
                value
            ))),
        }
    }
}

/// Skip Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SkipFlag {
    /// Cannot be skipped
    NoSkip,
    /// Can be skipped
    Skippable,
}

impl Serialize for SkipFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            SkipFlag::NoSkip => serializer.serialize_u32(0),
            SkipFlag::Skippable => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for SkipFlag {
    fn deserialize<D>(deserializer: D) -> Result<SkipFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(SkipFlag::NoSkip),
            1 => Ok(SkipFlag::Skippable),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid SkipFlag value: {}",
                value
            ))),
        }
    }
}

/// Boxing Allowed Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoxingAllowedFlag {
    /// Boxing not allowed
    NotAllowed,
    /// Boxing allowed
    Allowed,
}

impl Serialize for BoxingAllowedFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            BoxingAllowedFlag::NotAllowed => serializer.serialize_u32(0),
            BoxingAllowedFlag::Allowed => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for BoxingAllowedFlag {
    fn deserialize<D>(deserializer: D) -> Result<BoxingAllowedFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(BoxingAllowedFlag::NotAllowed),
            1 => Ok(BoxingAllowedFlag::Allowed),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid BoxingAllowedFlag value: {}",
                value
            ))),
        }
    }
}

/// Stitched Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StitchedFlag {
    /// Delivered independently
    Independent,
    /// Stitched with content
    Stitched,
}

impl Serialize for StitchedFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            StitchedFlag::Independent => serializer.serialize_u32(0),
            StitchedFlag::Stitched => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for StitchedFlag {
    fn deserialize<D>(deserializer: D) -> Result<StitchedFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(StitchedFlag::Independent),
            1 => Ok(StitchedFlag::Stitched),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid StitchedFlag value: {}",
                value
            ))),
        }
    }
}

/// Do Not Track Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DoNotTrackFlag {
    /// Tracking unrestricted
    TrackingUnrestricted,
    /// Do not track
    DoNotTrack,
}

impl Serialize for DoNotTrackFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            DoNotTrackFlag::TrackingUnrestricted => serializer.serialize_u32(0),
            DoNotTrackFlag::DoNotTrack => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for DoNotTrackFlag {
    fn deserialize<D>(deserializer: D) -> Result<DoNotTrackFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(DoNotTrackFlag::TrackingUnrestricted),
            1 => Ok(DoNotTrackFlag::DoNotTrack),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid DoNotTrackFlag value: {}",
                value
            ))),
        }
    }
}

/// Limit Ad Tracking Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LimitAdTrackingFlag {
    /// Tracking unrestricted
    TrackingUnrestricted,
    /// Tracking limited
    TrackingLimited,
}

impl Serialize for LimitAdTrackingFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            LimitAdTrackingFlag::TrackingUnrestricted => serializer.serialize_u32(0),
            LimitAdTrackingFlag::TrackingLimited => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for LimitAdTrackingFlag {
    fn deserialize<D>(deserializer: D) -> Result<LimitAdTrackingFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(LimitAdTrackingFlag::TrackingUnrestricted),
            1 => Ok(LimitAdTrackingFlag::TrackingLimited),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid LimitAdTrackingFlag value: {}",
                value
            ))),
        }
    }
}

/// JavaScript Support Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JavaScriptFlag {
    /// JavaScript not supported
    NotSupported,
    /// JavaScript supported
    Supported,
}

impl Serialize for JavaScriptFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            JavaScriptFlag::NotSupported => serializer.serialize_u32(0),
            JavaScriptFlag::Supported => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for JavaScriptFlag {
    fn deserialize<D>(deserializer: D) -> Result<JavaScriptFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(JavaScriptFlag::NotSupported),
            1 => Ok(JavaScriptFlag::Supported),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid JavaScriptFlag value: {}",
                value
            ))),
        }
    }
}

/// Geo Fetch Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeoFetchFlag {
    /// Geolocation API not available
    NotAvailable,
    /// Geolocation API available
    Available,
}

impl Serialize for GeoFetchFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            GeoFetchFlag::NotAvailable => serializer.serialize_u32(0),
            GeoFetchFlag::Available => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for GeoFetchFlag {
    fn deserialize<D>(deserializer: D) -> Result<GeoFetchFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(GeoFetchFlag::NotAvailable),
            1 => Ok(GeoFetchFlag::Available),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid GeoFetchFlag value: {}",
                value
            ))),
        }
    }
}

/// Mobile Optimized Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MobileOptimizedFlag {
    /// Not optimized for mobile
    NotOptimized,
    /// Optimized for mobile
    Optimized,
}

impl Serialize for MobileOptimizedFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            MobileOptimizedFlag::NotOptimized => serializer.serialize_u32(0),
            MobileOptimizedFlag::Optimized => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for MobileOptimizedFlag {
    fn deserialize<D>(deserializer: D) -> Result<MobileOptimizedFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(MobileOptimizedFlag::NotOptimized),
            1 => Ok(MobileOptimizedFlag::Optimized),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid MobileOptimizedFlag value: {}",
                value
            ))),
        }
    }
}

/// Privacy Policy Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrivacyPolicyFlag {
    /// No privacy policy
    No,
    /// Has privacy policy
    Yes,
}

impl Serialize for PrivacyPolicyFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            PrivacyPolicyFlag::No => serializer.serialize_u32(0),
            PrivacyPolicyFlag::Yes => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for PrivacyPolicyFlag {
    fn deserialize<D>(deserializer: D) -> Result<PrivacyPolicyFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(PrivacyPolicyFlag::No),
            1 => Ok(PrivacyPolicyFlag::Yes),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid PrivacyPolicyFlag value: {}",
                value
            ))),
        }
    }
}

/// Paid App Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaidAppFlag {
    /// App is free
    Free,
    /// App is paid
    Paid,
}

impl Serialize for PaidAppFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            PaidAppFlag::Free => serializer.serialize_u32(0),
            PaidAppFlag::Paid => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for PaidAppFlag {
    fn deserialize<D>(deserializer: D) -> Result<PaidAppFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(PaidAppFlag::Free),
            1 => Ok(PaidAppFlag::Paid),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid PaidAppFlag value: {}",
                value
            ))),
        }
    }
}

/// Live Stream Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LiveStreamFlag {
    /// Content is not live
    NotLive,
    /// Content is being streamed live
    Live,
}

impl Serialize for LiveStreamFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            LiveStreamFlag::NotLive => serializer.serialize_u32(0),
            LiveStreamFlag::Live => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for LiveStreamFlag {
    fn deserialize<D>(deserializer: D) -> Result<LiveStreamFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(LiveStreamFlag::NotLive),
            1 => Ok(LiveStreamFlag::Live),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid LiveStreamFlag value: {}",
                value
            ))),
        }
    }
}

/// Source Relationship
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SourceRelationship {
    /// Indirect relationship
    Indirect,
    /// Direct relationship
    Direct,
}

impl Serialize for SourceRelationship {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            SourceRelationship::Indirect => serializer.serialize_u32(0),
            SourceRelationship::Direct => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for SourceRelationship {
    fn deserialize<D>(deserializer: D) -> Result<SourceRelationship, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(SourceRelationship::Indirect),
            1 => Ok(SourceRelationship::Direct),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid SourceRelationship value: {}",
                value
            ))),
        }
    }
}

/// Embeddable Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmbeddableFlag {
    /// Content is not embeddable
    NotEmbeddable,
    /// Content is embeddable
    Embeddable,
}

impl Serialize for EmbeddableFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            EmbeddableFlag::NotEmbeddable => serializer.serialize_u32(0),
            EmbeddableFlag::Embeddable => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for EmbeddableFlag {
    fn deserialize<D>(deserializer: D) -> Result<EmbeddableFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(EmbeddableFlag::NotEmbeddable),
            1 => Ok(EmbeddableFlag::Embeddable),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid EmbeddableFlag value: {}",
                value
            ))),
        }
    }
}

/// Mobile Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MobileFlag {
    /// Not mobile
    NotMobile,
    /// Is mobile
    Mobile,
}

impl Serialize for MobileFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            MobileFlag::NotMobile => serializer.serialize_u32(0),
            MobileFlag::Mobile => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for MobileFlag {
    fn deserialize<D>(deserializer: D) -> Result<MobileFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(MobileFlag::NotMobile),
            1 => Ok(MobileFlag::Mobile),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid MobileFlag value: {}",
                value
            ))),
        }
    }
}

/// Group Flag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GroupFlag {
    /// Impressions can be won individually
    Individual,
    /// Impressions must be won or lost as a group
    Group,
}

impl Serialize for GroupFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            GroupFlag::Individual => serializer.serialize_u32(0),
            GroupFlag::Group => serializer.serialize_u32(1),
        }
    }
}

impl<'de> Deserialize<'de> for GroupFlag {
    fn deserialize<D>(deserializer: D) -> Result<GroupFlag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(GroupFlag::Individual),
            1 => Ok(GroupFlag::Group),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid GroupFlag value: {}",
                value
            ))),
        }
    }
}