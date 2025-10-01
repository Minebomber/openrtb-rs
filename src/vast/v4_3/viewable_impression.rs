//! Viewable impression tracking

use super::*;
use hard_xml::{XmlRead, XmlWrite};

/// Container for viewable impression tracking
///
/// Viewable impression tracking allows measurement of whether an ad
/// was actually viewable according to industry standards.
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "ViewableImpression")]
pub struct ViewableImpression {
    /// Optional identifier
    #[xml(attr = "id")]
    pub id: Option<String>,

    /// Viewable event tracking
    #[xml(child = "Viewable")]
    pub viewable: Vec<Viewable>,

    /// Not viewable event tracking
    #[xml(child = "NotViewable")]
    pub not_viewable: Vec<NotViewable>,

    /// Viewability undetermined tracking
    #[xml(child = "ViewUndetermined")]
    pub view_undetermined: Vec<ViewUndetermined>,
}

/// Tracking for viewable impressions
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Viewable")]
pub struct Viewable {
    /// Tracking URI for viewable event
    #[xml(text)]
    pub uri: Uri,
}

/// Tracking for non-viewable impressions
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "NotViewable")]
pub struct NotViewable {
    /// Tracking URI for not viewable event
    #[xml(text)]
    pub uri: Uri,
}

/// Tracking for undetermined viewability
#[derive(Debug, Clone, Default, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "ViewUndetermined")]
pub struct ViewUndetermined {
    /// Tracking URI for undetermined viewability
    #[xml(text)]
    pub uri: Uri,
}
