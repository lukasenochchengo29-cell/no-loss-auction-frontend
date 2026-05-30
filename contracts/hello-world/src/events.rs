use soroban_sdk::{contractevent, Address, Env};

#[contractevent]
pub struct AuctionCreated {
    pub auction_id: u64,
}

#[contractevent]
pub struct BidPlaced {
    pub auction_id: u64,
    pub bidder: Address,
    pub amount: i128,
}

#[contractevent]
pub struct RefundClaimed {
    pub user: Address,
    pub amount: i128,
}

#[contractevent]
pub struct AuctionFinalized {
    pub auction_id: u64,
    pub winner: Address,
}

#[contractevent]
pub struct AuctionCancelled {
    pub auction_id: u64,
}