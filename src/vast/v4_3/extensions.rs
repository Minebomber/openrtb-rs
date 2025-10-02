//! Extension elements for custom VAST extensions

use serde::{Deserialize, Serialize};

/// Container for ad-level extensions
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Extensions")]
pub struct Extensions {
    /// List of extensions
    #[serde(rename = "Extension", default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<Extension>,
}

/// A single extension element
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Extension")]
pub struct Extension {
    /// Type identifier for the extension
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub extension_type: Option<String>,

    /// The extension content (can contain any valid XML)
    #[serde(rename = "$value")]
    pub content: String,
}

/// Container for creative-level extensions
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "CreativeExtensions")]
pub struct CreativeExtensions {
    /// List of creative extensions
    #[serde(
        rename = "CreativeExtension",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub creative_extension: Vec<CreativeExtension>,
}

/// A single creative extension element
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "CreativeExtension")]
pub struct CreativeExtension {
    /// Type identifier for the extension
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub extension_type: Option<String>,

    /// The extension content (can contain any valid XML)
    #[serde(rename = "$value")]
    pub content: String,
}
