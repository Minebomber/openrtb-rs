//! Resource types for companion ads

use super::super::*;
use hard_xml::{XmlRead, XmlWrite};

/// Resource types for companion ads
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
pub enum CompanionResource {
    /// Static resource (image)
    #[xml(tag = "StaticResource")]
    StaticResource(CompanionStaticResource),

    /// IFrame resource
    #[xml(tag = "IFrameResource")]
    IFrameResource(CompanionIFrameResource),

    /// HTML resource
    #[xml(tag = "HTMLResource")]
    HTMLResource(CompanionHTMLResource),
}
