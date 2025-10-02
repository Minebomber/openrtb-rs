//! Ad type enumeration

use super::super::*;
use hard_xml::{XmlRead, XmlWrite};

/// The type of ad - either InLine or Wrapper
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
pub enum AdType {
    /// InLine ad containing all necessary creative files and tracking URIs
    #[xml(tag = "InLine")]
    InLine(InLine),

    /// Wrapper ad that redirects to another VAST response
    #[xml(tag = "Wrapper")]
    Wrapper(Wrapper),
}
