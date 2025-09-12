use super::enums::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Regs {
    /// Flag indicating if this request is subject to the COPPA regulations established by the USA FTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coppa: Option<CoppaFlag>,

    /// Flag indicating if regulations require restricted processing of personal data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gdpr: Option<GdprFlag>,

    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
}
