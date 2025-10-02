//! Tracking events for VAST ads

use super::enums::*;
use super::*;
use hard_xml::{XmlRead, XmlWrite};

/// Container for tracking events
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "TrackingEvents")]
pub struct TrackingEvents {
    /// List of tracking events
    #[xml(child = "Tracking")]
    pub tracking: Vec<Tracking>,
}

/// A single tracking event
#[derive(Debug, Clone, PartialEq, XmlWrite, XmlRead)]
#[xml(tag = "Tracking")]
pub struct Tracking {
    /// The type of event to track
    #[xml(attr = "event")]
    pub event: TrackingEvent,

    /// Offset for progress events (percentage or time)
    #[xml(attr = "offset")]
    pub offset: Option<String>,

    /// The tracking URI
    #[xml(text)]
    pub uri: Uri,
}

impl TrackingEvents {
    /// Create a new empty TrackingEvents container
    pub fn new() -> Self {
        TrackingEvents {
            tracking: Vec::new(),
        }
    }

    /// Add a tracking event
    pub fn add_tracking(&mut self, event: TrackingEvent, uri: Uri) {
        self.tracking.push(Tracking {
            event,
            offset: None,
            uri,
        });
    }

    /// Add a tracking event with offset
    pub fn add_tracking_with_offset(&mut self, event: TrackingEvent, uri: Uri, offset: String) {
        self.tracking.push(Tracking {
            event,
            offset: Some(offset),
            uri,
        });
    }
}

impl Default for TrackingEvents {
    fn default() -> Self {
        Self::new()
    }
}
