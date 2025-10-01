pub mod v4_3;

/// Serialize a VAST structure to an XML string.
///
/// # Example
/// ```ignore
/// let vast = Vast { ... };
/// let xml = openrtb::vast::to_string(&vast)?;
/// ```
pub fn to_string<T: hard_xml::XmlWrite>(value: &T) -> hard_xml::XmlResult<String> {
    T::to_string(value)
}

/// Deserialize a VAST structure from an XML string.
///
/// # Example
/// ```ignore
/// let xml = "<VAST version=\"4.3\">...</VAST>";
/// let vast: Vast = openrtb::vast::from_str(xml)?;
/// ```
pub fn from_str<'a, T: hard_xml::XmlRead<'a>>(s: &'a str) -> hard_xml::XmlResult<T> {
    T::from_str(s)
}
