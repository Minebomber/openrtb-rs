//! OpenRTB 2.6 Specification
//!
//! This crate provides Rust structs and enums for the OpenRTB 2.6 specification,
//! enabling real-time bidding in programmatic advertising.
//!
//! The structs follow the official OpenRTB 2.6 specification from the
//! Interactive Advertising Bureau (IAB) Tech Lab.

// Modules
pub mod enums;
pub mod v2_6;

// Re-export commonly used types
pub use enums::*;
pub use v2_6::*;

// For backward compatibility, re-export main types at crate root
// pub use v2_6::App;
// pub use v2_6::Audio;
// pub use v2_6::Banner;
// pub use v2_6::Bid;
// pub use v2_6::BidRequest;
// pub use v2_6::BidResponse;
// pub use v2_6::Device;
// pub use v2_6::Geo;
// pub use v2_6::Impression;
// pub use v2_6::Native;
// pub use v2_6::SeatBid;
// pub use v2_6::Site;
// pub use v2_6::User;
// pub use v2_6::Video;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bid_request_serialization() {
        let bid_request = BidRequest {
            id: "test-request-id".to_string(),
            imp: vec![Impression {
                id: "imp-1".to_string(),
                banner: Some(Banner {
                    w: Some(728),
                    h: Some(90),
                    format: None,
                    wmax: None,
                    hmax: None,
                    wmin: None,
                    hmin: None,
                    id: None,
                    btype: None,
                    battr: None,
                    pos: None,
                    mimes: None,
                    topframe: None,
                    expdir: None,
                    api: None,
                    ext: None,
                }),
                video: None,
                audio: None,
                native: None,
                pmp: None,
                displaymanager: None,
                displaymanagerver: None,
                instl: None,
                tagid: None,
                bidfloor: Some(1.5),
                bidfloorcur: Some("USD".to_string()),
                clickbrowser: None,
                secure: None,
                iframebuster: None,
                exp: None,
                metric: None,
                ext: None,
            }],
            site: None,
            app: None,
            dooh: None,
            device: None,
            user: None,
            test: None,
            at: Some(AuctionType::SecondPricePlus),
            tmax: None,
            wseat: None,
            bseat: None,
            allimps: None,
            cur: None,
            wlang: None,
            cacheid: None,
            bcat: None,
            badv: None,
            bapp: None,
            source: None,
            regs: None,
            ext: None,
        };

        let serialized = serde_json::to_string(&bid_request).unwrap();
        let deserialized: BidRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(bid_request, deserialized);
    }

    #[test]
    fn test_bid_response_serialization() {
        let bid_response = BidResponse {
            id: "test-response-id".to_string(),
            seatbid: Some(vec![SeatBid {
                bid: vec![Bid {
                    id: "bid-1".to_string(),
                    impid: "imp-1".to_string(),
                    price: 2.50,
                    nurl: Some("https://example.com/win".to_string()),
                    burl: None,
                    lurl: None,
                    adm: Some("<html>Ad markup</html>".to_string()),
                    adid: None,
                    adomain: None,
                    bundle: None,
                    iurl: None,
                    cid: None,
                    crid: None,
                    tactic: None,
                    cat: None,
                    attr: None,
                    api: None,
                    protocol: None,
                    qagmediarating: None,
                    language: None,
                    dealid: None,
                    w: Some(728),
                    h: Some(90),
                    wratio: None,
                    hratio: None,
                    exp: None,
                    ext: None,
                }],
                seat: Some("seat-123".to_string()),
                group: None,
                ext: None,
            }]),
            bidid: None,
            cur: Some("USD".to_string()),
            customdata: None,
            nbr: None,
            ext: None,
        };

        let serialized = serde_json::to_string(&bid_response).unwrap();
        let deserialized: BidResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(bid_response, deserialized);
    }
}
