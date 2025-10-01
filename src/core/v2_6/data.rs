use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The data and segment objects together allow additional data about the related object to be specified.
///
/// This applies to the user, device, or other objects as specified by the parent.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Data<Ext = Value> {
    /// Exchange-specific ID for the data provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Exchange-specific name for the data provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Array of `Segment` objects that contain the actual data values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<Vec<Segment>>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}
