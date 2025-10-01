//! Extension elements for custom VAST extensions

use hard_xml::{XmlRead, XmlWrite};

/// Container for ad-level extensions
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Extensions")]
pub struct Extensions {
    /// List of extensions
    #[xml(child = "Extension")]
    pub extension: Vec<Extension>,
}

/// A single extension element
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Extension")]
pub struct Extension {
    /// Type identifier for the extension
    #[xml(attr = "type")]
    pub extension_type: Option<String>,

    /// The extension content (can contain any valid XML)
    #[xml(text)]
    pub content: String,
}

/// Container for creative-level extensions
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "CreativeExtensions")]
pub struct CreativeExtensions {
    /// List of creative extensions
    #[xml(child = "CreativeExtension")]
    pub creative_extension: Vec<CreativeExtension>,
}

/// A single creative extension element
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "CreativeExtension")]
pub struct CreativeExtension {
    /// Type identifier for the extension
    #[xml(attr = "type")]
    pub extension_type: Option<String>,

    /// The extension content (can contain any valid XML)
    #[xml(text)]
    pub content: String,
}

