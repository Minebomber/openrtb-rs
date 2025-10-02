//! Tracking events for VAST ads

use super::*;
use serde::{Deserialize, Serialize};

/// Container for tracking events
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "TrackingEvents")]
pub struct TrackingEvents {
    /// List of tracking events
    #[serde(rename = "Tracking", default, skip_serializing_if = "Vec::is_empty")]
    pub tracking: Vec<Tracking>,
}

/// A single tracking event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Tracking")]
pub struct Tracking {
    /// The type of event to track
    #[serde(rename = "@event")]
    pub event: TrackingEvent,

    /// Offset for progress events (percentage or time)
    #[serde(rename = "@offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,

    /// The tracking URI
    #[serde(rename = "$value")]
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

