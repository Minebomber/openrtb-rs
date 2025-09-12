use super::enums::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Native Markup Response Object - OpenRTB Native 1.2 Section 5.1
///
/// The native object is the top level JSON object which identifies a native response. 
/// The native creative shall be returned as a JSON-encoded string in the adm field of the Bid Object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NativeResponse {
    /// Version of the Native Markup version in use
    #[serde(default = "default_ver", skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,
    
    /// List of native ad's assets. Required if no assetsurl. 
    /// Recommended as fallback even if assetsurl is provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AssetResponse>>,
    
    /// URL of an alternate source for the assets object. The expected response is a JSON object 
    /// mirroring the assets object in the bid response, subject to certain requirements as 
    /// specified in the individual objects. Where present, overrides the asset object in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assetsurl: Option<String>,
    
    /// URL where a dynamic creative specification may be found for populating this ad, 
    /// per the Dynamic Content Ads Specification. Note this is a beta option. 
    /// Where present, overrides the asset object in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcourl: Option<String>,
    
    /// Destination Link. This is default link object for the ad. Individual assets can also have 
    /// a link object which applies if the asset is activated(clicked). If the asset doesn't have 
    /// a link object, the parent link object applies
    pub link: LinkResponse,
    
    /// Array of impression tracking URLs, expected to return a 1x1 image or 204 response - 
    /// typically only passed when using 3rd party trackers. To be deprecated - replaced with eventtrackers
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Use eventtrackers instead")]
    pub imptrackers: Option<Vec<String>>,
    
    /// Optional JavaScript impression tracker. This is a valid HTML, Javascript is already wrapped 
    /// in <script> tags. It should be executed at impression time where it can be supported. 
    /// To be deprecated - replaced with eventtrackers
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Use eventtrackers instead")]
    pub jstracker: Option<String>,
    
    /// Array of tracking objects to run with the ad, in response to the declared supported methods 
    /// in the request. Replaces imptrackers and jstracker, to be deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventtrackers: Option<Vec<EventTrackerResponse>>,
    
    /// If support was indicated in the request, URL of a page informing the user about the 
    /// buyer's targeting activity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<String>,
    
    /// This object is a placeholder that may contain custom JSON agreed to by the parties to 
    /// support flexibility beyond the standard defined in this specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Asset Response Object - OpenRTB Native 1.2 Section 5.2
///
/// Corresponds to the Asset Object in the request. The main container object for each asset 
/// requested or supported by Exchange on behalf of the rendering client
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssetResponse {
    /// Optional if assetsurl/dcourl is being used; required if embedded asset is being used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    
    /// Set to 1 if asset is required. (bidder requires it to be displayed)
    #[serde(default = "default_required", skip_serializing_if = "Option::is_none")]
    pub required: Option<u32>,
    
    /// Title object for title assets. See TitleObject definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<TitleResponse>,
    
    /// Image object for image assets. See ImageObject definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<ImageResponse>,
    
    /// Video object for video assets. See Video response object definition. 
    /// Note that in-stream video ads are not part of Native. 
    /// Native ads may contain a video as the ad creative itself
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<VideoResponse>,
    
    /// Data object for ratings, prices etc
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DataResponse>,
    
    /// Link object for call to actions. The link object applies if the asset item is 
    /// activated (clicked). If there is no link object on the asset, the parent link 
    /// object on the bid response applies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<LinkResponse>,
    
    /// This object is a placeholder that may contain custom JSON agreed to by the parties to 
    /// support flexibility beyond the standard defined in this specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Title Response Object - OpenRTB Native 1.2 Section 5.3
///
/// Corresponds to the Title Object in the request, with the value filled in
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TitleResponse {
    /// The text associated with the text element
    pub text: String,
    
    /// The length of the title being provided. Required if using assetsurl/dcourl representation, 
    /// optional if using embedded asset representation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<u32>,
    
    /// This object is a placeholder that may contain custom JSON agreed to by the parties to 
    /// support flexibility beyond the standard defined in this specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Image Response Object - OpenRTB Native 1.2 Section 5.4
///
/// Corresponds to the Image Object in the request. The Image object to be used for all image 
/// elements of the Native ad such as Icons, Main Image, etc
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageResponse {
    /// Required for assetsurl or dcourl responses, not required for embedded asset responses. 
    /// The type of image element being submitted from the Image Asset Types table
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub img_type: Option<ImageAssetType>,
    
    /// URL of the image asset
    pub url: String,
    
    /// Width of the image in pixels. Recommended for embedded asset responses. 
    /// Required for assetsurl/dcourl responses if multiple assets of same type submitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<u32>,
    
    /// Height of the image in pixels. Recommended for embedded asset responses. 
    /// Required for assetsurl/dcourl responses if multiple assets of same type submitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<u32>,
    
    /// This object is a placeholder that may contain custom JSON agreed to by the parties to 
    /// support flexibility beyond the standard defined in this specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Data Response Object - OpenRTB Native 1.2 Section 5.5
///
/// Corresponds to the Data Object in the request, with the value filled in. The Data Object 
/// is to be used for all miscellaneous elements of the native unit such as Brand Name, Ratings, 
/// Review Count, Stars, Downloads, Price count etc
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataResponse {
    /// Required for assetsurl/dcourl responses, not required for embedded asset responses. 
    /// The type of data element being submitted from the Data Asset Types table
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub data_type: Option<DataAssetType>,
    
    /// Required for assetsurl/dcourl responses, not required for embedded asset responses. 
    /// The length of the data element being submitted. Where applicable, must comply with 
    /// the recommended maximum lengths in the Data Asset Types table
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<u32>,
    
    /// The formatted string of data to be displayed. Can contain a formatted value such as 
    /// "5 stars" or "$10" or "3.4 stars out of 5"
    pub value: String,
    
    /// This object is a placeholder that may contain custom JSON agreed to by the parties to 
    /// support flexibility beyond the standard defined in this specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Video Response Object - OpenRTB Native 1.2 Section 5.6
///
/// Corresponds to the Video Object in the request, yet containing a value of a conforming VAST tag as a value
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoResponse {
    /// VAST xml
    pub vasttag: String,
}

/// Link Response Object - OpenRTB Native 1.2 Section 5.7
///
/// Used for 'call to action' assets, or other links from the Native ad. This Object should be 
/// associated to its peer object in the parent Asset Object or as the master link in the top 
/// level Native Ad response object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LinkResponse {
    /// Landing URL of the clickable link
    pub url: String,
    
    /// List of third-party tracker URLs to be fired on click of the URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicktrackers: Option<Vec<String>>,
    
    /// Fallback URL for deeplink. To be used if the URL given in url is not supported by the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<String>,
    
    /// This object is a placeholder that may contain custom JSON agreed to by the parties to 
    /// support flexibility beyond the standard defined in this specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

/// Event Tracker Response Object - OpenRTB Native 1.2 Section 5.8
///
/// The event trackers response is an array of objects and specifies the types of events the 
/// bidder wishes to track and the URLs/information to track them
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventTrackerResponse {
    /// Type of event to track. See Event Types table
    pub event: EventType,
    
    /// Type of tracking requested. See Event Tracking Methods table
    pub method: EventTrackingMethod,
    
    /// The URL of the image or js. Required for image or js, optional for custom
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    
    /// To be agreed individually with the exchange, an array of key:value objects for custom 
    /// tracking, for example the account number of the DSP with a tracking company. 
    /// IE {"accountnumber":"123"}
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customdata: Option<Value>,
    
    /// This object is a placeholder that may contain custom JSON agreed to by the parties to 
    /// support flexibility beyond the standard defined in this specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}

// Default functions
fn default_ver() -> Option<String> {
    Some("1.2".to_string())
}

fn default_required() -> Option<u32> {
    Some(0)
}