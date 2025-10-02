//! Resource types for non-linear creatives

use super::super::*;
use hard_xml::{XmlRead, XmlWrite};

/// Resource types for non-linear creatives
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
pub enum NonLinearResource {
    /// Static resource (image)
    #[xml(tag = "StaticResource")]
    StaticResource(StaticResource),

    /// IFrame resource
    #[xml(tag = "IFrameResource")]
    IFrameResource(IFrameResource),

    /// HTML resource
    #[xml(tag = "HTMLResource")]
    HTMLResource(HTMLResource),
}
