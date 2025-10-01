use super::enums::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Source<Ext = Value> {
    /// Entity responsible for the final impression sale decision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fd: Option<FinalDecision>,

    /// Transaction ID that must be common across all participants in this bid request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tid: Option<String>,

    /// Payment ID chain string containing embedded syntax described in the TAG Payment ID Protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pchain: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}
