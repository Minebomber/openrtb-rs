use serde::{Deserialize, Serialize};
use serde_json::Value;

#[cfg(feature = "native")]
use {
    crate::native::v1_2::NativeRequest,
    serde::{Deserializer, Serializer},
};

/// This object represents a native type impression.
///
/// Native ad units are intended to blend seamlessly into the surrounding content. The Native Subcommittee
/// has developed a companion specification to OpenRTB called the Dynamic Native Ads API.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Native<Ext = Value> {
    /// Request payload complying with the Native Ad Specification.
    #[cfg(feature = "native")]
    #[serde(
        serialize_with = "serialize_native_request",
        deserialize_with = "deserialize_native_request"
    )]
    pub request: NativeRequest,

    /// Request payload complying with the Native Ad Specification.
    #[cfg(not(feature = "native"))]
    pub request: String,

    /// Version of the Dynamic Native Ads API to which request complies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,

    /// List of supported API frameworks for this impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<u32>>,

    /// Blocked creative attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<u32>>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}

/// Custom serializer to convert NativeRequest to JSON string
#[cfg(feature = "native")]
fn serialize_native_request<S>(request: &NativeRequest, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let json_string = serde_json::to_string(request).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&json_string)
}

/// Custom deserializer to convert JSON string to NativeRequest
#[cfg(feature = "native")]
fn deserialize_native_request<'de, D>(deserializer: D) -> Result<NativeRequest, D::Error>
where
    D: Deserializer<'de>,
{
    let json_string = String::deserialize(deserializer)?;
    serde_json::from_str(&json_string).map_err(serde::de::Error::custom)
}

#[cfg(test)]
#[cfg(feature = "native")]
mod tests {
    use super::*;
    use crate::native::v1_2::{AssetRequest, TitleRequest};

    #[test]
    fn test_native_request_serialization() {
        // Create a NativeRequest object
        let native_request = NativeRequest {
            ver: Some("1.2".to_string()),
            context: None,
            contextsubtype: None,
            plcmttype: None,
            plcmtcnt: Some(1),
            seq: Some(0),
            assets: vec![AssetRequest {
                id: 1,
                required: Some(1),
                title: Some(TitleRequest { len: 90, ext: None }),
                img: None,
                video: None,
                data: None,
                ext: None,
            }],
            aurlsupport: Some(0),
            durlsupport: Some(0),
            eventtrackers: None,
            privacy: Some(1),
            ext: None,
        };

        // Create a Native object with the NativeRequest
        let native: Native = Native {
            request: native_request.clone(),
            ver: Some("1.2".to_string()),
            api: None,
            battr: None,
            ext: None,
        };

        // Serialize the Native object to JSON
        let json = serde_json::to_value(&native).unwrap();

        // Verify that the request field is serialized as a string
        assert!(json["request"].is_string());

        // The request field should contain a JSON string
        let request_json_str = json["request"].as_str().unwrap();
        let parsed_request: NativeRequest = serde_json::from_str(request_json_str).unwrap();

        // Verify the parsed request matches the original
        assert_eq!(parsed_request.assets.len(), 1);
        assert_eq!(parsed_request.assets[0].id, 1);
    }

    #[test]
    fn test_native_request_deserialization() {
        // Create a JSON string with a Native object where request is a JSON string
        let json_str = r#"{
            "request": "{\"ver\":\"1.2\",\"plcmtcnt\":1,\"seq\":0,\"assets\":[{\"id\":1,\"required\":1,\"title\":{\"len\":90}}],\"aurlsupport\":0,\"durlsupport\":0,\"privacy\":1}",
            "ver": "1.2"
        }"#;

        // Deserialize the JSON string to a Native object
        let native: Native = serde_json::from_str(json_str).unwrap();

        // Verify the NativeRequest was properly deserialized
        assert_eq!(native.request.assets.len(), 1);
        assert_eq!(native.request.assets[0].id, 1);
        assert_eq!(native.request.assets[0].title.as_ref().unwrap().len, 90);
        assert_eq!(native.request.privacy, Some(1));
    }

    #[test]
    fn test_roundtrip_serialization() {
        // Create a Native object with a NativeRequest
        let native_request = NativeRequest {
            ver: Some("1.2".to_string()),
            context: None,
            contextsubtype: None,
            plcmttype: None,
            plcmtcnt: Some(1),
            seq: Some(0),
            assets: vec![AssetRequest {
                id: 1,
                required: Some(1),
                title: Some(TitleRequest { len: 90, ext: None }),
                img: None,
                video: None,
                data: None,
                ext: None,
            }],
            aurlsupport: Some(0),
            durlsupport: Some(0),
            eventtrackers: None,
            privacy: Some(1),
            ext: None,
        };

        let native: Native = Native {
            request: native_request,
            ver: Some("1.2".to_string()),
            api: Some(vec![1, 2]),
            battr: Some(vec![3, 4]),
            ext: None,
        };

        // Serialize to JSON string
        let json_string = serde_json::to_string(&native).unwrap();

        // Deserialize back to Native object
        let deserialized: Native = serde_json::from_str(&json_string).unwrap();

        // Verify they match
        assert_eq!(native, deserialized);
    }
}
