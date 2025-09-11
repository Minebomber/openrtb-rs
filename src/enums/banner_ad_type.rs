use serde::{Deserialize, Serialize};

/// Banner Ad Type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum BannerAdType {
    /// XHTML Text Ad (usually mobile)
    XhtmlTextAd = 1,
    /// XHTML Banner Ad. (usually mobile)
    XhtmlBannerAd = 2,
    /// JavaScript Ad; must be valid XHTML (i.e., Script Tags Included)
    JavascriptAd = 3,
    /// Iframe
    Iframe = 4,
}

