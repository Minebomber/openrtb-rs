use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This object is associated with an impression as an array of metrics.
///
/// These metrics can offer insight to assist with decisioning such as average recent viewability,
/// click-through rate, etc.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Metric<Ext = Value> {
    /// Type of metric being presented using exchange curated string names.
    pub r#type: String,

    /// Number representing the value of the metric.
    pub value: f64,

    /// Optional vendor-specific ID for the metric to assist with logging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Ext>,
}
