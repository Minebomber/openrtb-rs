use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Data Asset Types - OpenRTB Native 1.2 Section 7.4
///
/// Below is a list of common asset element types of native advertising at the time of
/// writing this spec. This list is non-exhaustive and intended to be extended by the
/// buyers and sellers as the format evolves.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum DataAssetType {
    /// Sponsored By message where response should contain the brand name of the sponsor
    /// Recommended. Max 25 or longer.
    Sponsored = 1,
    /// Descriptive text associated with the product or service being advertised
    /// Longer length of text in response may be truncated or ellipsed by the exchange
    /// Recommended. Max 140 or longer.
    Desc = 2,
    /// Rating of the product being offered to the user. For example an app's rating in an app store from 0-5
    /// Optional. 0-5 integer formatted as string.
    Rating = 3,
    /// Number of social ratings or "likes" of the product being offered to the user
    Likes = 4,
    /// Number downloads/installs of this product
    Downloads = 5,
    /// Price for product / app / in-app purchase. Value should include currency symbol in localised format
    Price = 6,
    /// Sale price that can be used together with price to indicate a discounted price compared to a regular price
    /// Value should include currency symbol in localised format
    SalePrice = 7,
    /// Phone number
    Phone = 8,
    /// Address
    Address = 9,
    /// Additional descriptive text associated with the product or service being advertised
    Desc2 = 10,
    /// Display URL for the text ad. To be used when sponsoring entity doesn't own the content
    /// IE sponsored by BRAND on SITE (where SITE is transmitted in this field)
    DisplayUrl = 11,
    /// CTA description - descriptive text describing a 'call to action' button for the destination URL
    /// Optional. Max 15 or longer.
    CtaText = 12,
    /// Reserved for Exchange specific usage numbered above 500
    ExchangeSpecific(u32),
}

impl Serialize for DataAssetType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            DataAssetType::Sponsored => serializer.serialize_u32(1),
            DataAssetType::Desc => serializer.serialize_u32(2),
            DataAssetType::Rating => serializer.serialize_u32(3),
            DataAssetType::Likes => serializer.serialize_u32(4),
            DataAssetType::Downloads => serializer.serialize_u32(5),
            DataAssetType::Price => serializer.serialize_u32(6),
            DataAssetType::SalePrice => serializer.serialize_u32(7),
            DataAssetType::Phone => serializer.serialize_u32(8),
            DataAssetType::Address => serializer.serialize_u32(9),
            DataAssetType::Desc2 => serializer.serialize_u32(10),
            DataAssetType::DisplayUrl => serializer.serialize_u32(11),
            DataAssetType::CtaText => serializer.serialize_u32(12),
            DataAssetType::ExchangeSpecific(val) => serializer.serialize_u32(val),
        }
    }
}

impl<'de> Deserialize<'de> for DataAssetType {
    fn deserialize<D>(deserializer: D) -> Result<DataAssetType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Ok(match value {
            1 => DataAssetType::Sponsored,
            2 => DataAssetType::Desc,
            3 => DataAssetType::Rating,
            4 => DataAssetType::Likes,
            5 => DataAssetType::Downloads,
            6 => DataAssetType::Price,
            7 => DataAssetType::SalePrice,
            8 => DataAssetType::Phone,
            9 => DataAssetType::Address,
            10 => DataAssetType::Desc2,
            11 => DataAssetType::DisplayUrl,
            12 => DataAssetType::CtaText,
            v if v >= 500 => DataAssetType::ExchangeSpecific(v),
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid DataAssetType value: {}",
                    value
                )));
            }
        })
    }
}

