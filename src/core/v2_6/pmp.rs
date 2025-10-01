use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This object is the private marketplace container for direct deals between buyers and sellers
/// that may pertain to this impression.
///
/// The actual deals are represented as a collection of Deal objects.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Pmp<Ext = Value> {
    /// Indicator of auction eligibility to seats named in the Direct Deals object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_auction: Option<u32>,

    /// Array of `Deal` objects that convey the specific deals applicable to this impression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deals: Option<Vec<Deal>>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}
