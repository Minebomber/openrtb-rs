use super::enums::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Native Markup Request Object - OpenRTB Native 1.2 Section 4.1
///
/// The Native Object defines the native advertising opportunity available for bid via this bid request. 
/// It will be included as a JSON-encoded string in the bid request's imp.native field or as a direct 
/// JSON object, depending on the choice of the exchange.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NativeRequest {
    /// Version of the Native Markup version in use
    #[serde(default = "default_ver", skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,
    
    /// The context in which the ad appears. See Table of Context IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ContextType>,
    
    /// A more detailed context in which the ad appears. See Table of Context SubType IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contextsubtype: Option<ContextSubType>,
    
    /// The design/format/layout of the ad unit being offered. See Table of Placement Type IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plcmttype: Option<PlacementType>,
    
    /// The number of identical placements in this Layout. Refer Section 8.1 Multiplacement Bid Requests
    #[serde(default = "default_plcmtcnt", skip_serializing_if = "Option::is_none")]
    pub plcmtcnt: Option<u32>,
    
    /// 0 for the first ad, 1 for the second ad, and so on. Note this would generally NOT be used 
    /// in combination with plcmtcnt - either you are auctioning multiple identical placements 
    /// (in which case plcmtcnt>1, seq=0) or you are holding separate auctions for distinct items 
    /// in the feed (in which case plcmtcnt=1, seq=>=1)
    #[serde(default = "default_seq", skip_serializing_if = "Option::is_none")]
    pub seq: Option<u32>,
    
    /// An array of Asset Objects. Any bid response must comply with the array of elements 
    /// expressed in the bid request
    pub assets: Vec<AssetRequest>,
    
    /// Whether the supply source / impression supports returning an assetsurl instead of an 
    /// asset object. 0 or the absence of the field indicates no such support
    #[serde(default = "default_aurlsupport", skip_serializing_if = "Option::is_none")]
    pub aurlsupport: Option<u32>,
    
    /// Whether the supply source / impression supports returning a dco url instead of an asset 
    /// object. 0 or the absence of the field indicates no such support. Beta feature
    #[serde(default = "default_durlsupport", skip_serializing_if = "Option::is_none")]
    pub durlsupport: Option<u32>,
    
    /// Specifies what type of event tracking is supported - see Event Trackers Request Object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventtrackers: Option<Vec<EventTrackersRequest>>,
    
    /// Set to 1 when the native ad supports buyer-specific privacy notice. Set to 0 (or field absent) 
    /// when the native ad doesn't support custom privacy links or if support is unknown
    #[serde(default = "default_privacy", skip_serializing_if = "Option::is_none")]
    pub privacy: Option<u32>,
    
    /// This object is a placeholder that may contain custom JSON agreed to by the parties to 
    /// support flexibility beyond the standard defined in this specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Asset Request Object - OpenRTB Native 1.2 Section 4.2
///
/// The main container object for each asset requested or supported by Exchange on behalf of the 
/// rendering client. Any object that is required is to be flagged as such. Only one of the 
/// {title,img,video,data} objects should be present in each object.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssetRequest {
    /// Unique asset ID, assigned by exchange. Typically a counter for the array
    pub id: u32,
    
    /// Set to 1 if asset is required (exchange will not accept a bid without it)
    #[serde(default = "default_required", skip_serializing_if = "Option::is_none")]
    pub required: Option<u32>,
    
    /// Title object for title assets. See TitleObject definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<TitleRequest>,
    
    /// Image object for image assets. See ImageObject definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<ImageRequest>,
    
    /// Video object for video assets. See the Video request object definition. 
    /// Note that in-stream (ie preroll, etc) video ads are not part of Native. 
    /// Native ads may contain a video as the ad creative itself
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<VideoRequest>,
    
    /// Data object for brand name, description, ratings, prices etc. See DataObject definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DataRequest>,
    
    /// This object is a placeholder that may contain custom JSON agreed to by the parties to 
    /// support flexibility beyond the standard defined in this specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Title Request Object - OpenRTB Native 1.2 Section 4.3
///
/// The Title object is to be used for title element of the Native ad
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TitleRequest {
    /// Maximum length of the text in the title element. Recommended to be 25, 90, or 140
    pub len: u32,
    
    /// This object is a placeholder that may contain custom JSON agreed to by the parties to 
    /// support flexibility beyond the standard defined in this specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Image Request Object - OpenRTB Native 1.2 Section 4.4
///
/// The Image object to be used for all image elements of the Native ad such as Icons, Main Image, etc. 
/// Recommended sizes and aspect ratios are included in the Image Asset Types section
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageRequest {
    /// Type ID of the image element supported by the publisher. See Table Image Asset Types
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub img_type: Option<ImageAssetType>,
    
    /// Width of the image in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<u32>,
    
    /// The minimum requested width of the image in pixels. This option should be used for any 
    /// rescaling of images by the client. Either w or wmin should be transmitted. 
    /// If only w is included, it should be considered an exact requirement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wmin: Option<u32>,
    
    /// Height of the image in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<u32>,
    
    /// The minimum requested height of the image in pixels. This option should be used for any 
    /// rescaling of images by the client. Either h or hmin should be transmitted. 
    /// If only h is included, it should be considered an exact requirement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmin: Option<u32>,
    
    /// Whitelist of content MIME types supported. Popular MIME types include, but are not limited to 
    /// "image/jpg" "image/gif". If blank, assume all types are allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimes: Option<Vec<String>>,
    
    /// This object is a placeholder that may contain custom JSON agreed to by the parties to 
    /// support flexibility beyond the standard defined in this specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Video Request Object - OpenRTB Native 1.2 Section 4.5
///
/// The video object to be used for all video elements supported in the Native Ad. 
/// This corresponds to the Video object of OpenRTB
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoRequest {
    /// Content MIME types supported. Popular MIME types include, but are not limited to 
    /// "video/x-ms-wmv" for Windows Media, and "video/x-flv" for Flash Video, or "video/mp4". 
    /// Note that native frequently does not support flash
    pub mimes: Vec<String>,
    
    /// Minimum video ad duration in seconds
    pub minduration: u32,
    
    /// Maximum video ad duration in seconds
    pub maxduration: u32,
    
    /// An array of video protocols the publisher can accept in the bid response. 
    /// See OpenRTB Table 'Video Bid Response Protocols' for a list of possible values
    pub protocols: Vec<u32>,
    
    /// This object is a placeholder that may contain custom JSON agreed to by the parties to 
    /// support flexibility beyond the standard defined in this specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Data Request Object - OpenRTB Native 1.2 Section 4.6
///
/// The Data Object is to be used for all non-core elements of the native unit such as Brand Name, 
/// Ratings, Review Count, Stars, Download count, descriptions etc. It is also generic for future 
/// native elements not contemplated at the time of the writing of this document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataRequest {
    /// Type ID of the element supported by the publisher. The publisher can display this 
    /// information in an appropriate format. See Data Asset Types table for commonly used examples
    #[serde(rename = "type")]
    pub data_type: DataAssetType,
    
    /// Maximum length of the text in the element's response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<u32>,
    
    /// This object is a placeholder that may contain custom JSON agreed to by the parties to 
    /// support flexibility beyond the standard defined in this specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Event Trackers Request Object - OpenRTB Native 1.2 Section 4.7
///
/// The event trackers object specifies the types of events the bidder can request to be tracked 
/// in the bid response, and which types of tracking are available for each event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventTrackersRequest {
    /// Type of event available for tracking. See Event Types table
    pub event: EventType,
    
    /// Array of the types of tracking available for the given event. See Event Tracking Methods table
    pub methods: Vec<EventTrackingMethod>,
    
    /// This object is a placeholder that may contain custom JSON agreed to by the parties to 
    /// support flexibility beyond the standard defined in this specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

// Default functions
fn default_ver() -> Option<String> {
    Some("1.2".to_string())
}

fn default_plcmtcnt() -> Option<u32> {
    Some(1)
}

fn default_seq() -> Option<u32> {
    Some(0)
}

fn default_aurlsupport() -> Option<u32> {
    Some(0)
}

fn default_durlsupport() -> Option<u32> {
    Some(0)
}

fn default_privacy() -> Option<u32> {
    Some(0)
}

fn default_required() -> Option<u32> {
    Some(0)
}