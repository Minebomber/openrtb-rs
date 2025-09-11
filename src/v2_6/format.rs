use serde::{Deserialize, Serialize};
use serde_json::Value;

/// This object is a sub-object of a Format object.
///
/// It defines a format as a set of width and height.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Format {
    /// Width in device independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// Height in device independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// Relative width when expressing size as a ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wratio: Option<i32>,

    /// Relative height when expressing size as a ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hratio: Option<i32>,

    /// The minimum width in device independent pixels (DIPS) at which the ad will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wmin: Option<i32>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}
