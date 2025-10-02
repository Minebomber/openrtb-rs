//! Resource types for icons

use super::super::*;
use hard_xml::{XmlRead, XmlWrite};

/// Resource types for icons
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
pub enum IconResource {
    /// Static resource (image)
    #[xml(tag = "StaticResource")]
    StaticResource(IconStaticResource),

    /// IFrame resource
    #[xml(tag = "IFrameResource")]
    IFrameResource(IconIFrameResource),

    /// HTML resource
    #[xml(tag = "HTMLResource")]
    HTMLResource(IconHTMLResource),
}
