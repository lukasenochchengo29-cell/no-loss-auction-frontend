use soroban_sdk::{
    contracttype,
    Address,
    String,
};

#[derive(Clone)]
#[contracttype]
pub struct Auction {
    pub id: u64,
    pub seller: Address,
    pub item_name: String,
    pub token: Address,
    pub highest_bid: i128,
    pub highest_bidder: Option<Address>,
    pub deadline: u64,
    pub active: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Auction(u64),
    AuctionCount,
    Refund(Address),
}