use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum AuctionError {
    AuctionNotFound = 1,
    AuctionClosed = 2,
    AuctionEnded = 3,
    BidTooLow = 4,
    Unauthorized = 5,
    NoRefundAvailable = 6,
    AuctionHasBids = 7,
    InvalidDeadline = 8,
    InvalidBidAmount = 9,
}