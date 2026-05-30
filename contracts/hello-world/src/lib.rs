#![no_std]

mod errors;
mod events;
mod storage;

use errors::AuctionError;
use events::*;
use storage::*;

use soroban_sdk::{contract, contractimpl, token, Address, Env, String};

#[contract]
pub struct NoLossAuction;

#[contractimpl]
impl NoLossAuction {
    pub fn create_auction(
        env: Env,
        seller: Address,
        item_name: String,
        token: Address,
        starting_bid: i128,
        deadline: u64,
    ) -> Result<u64, AuctionError> {
        seller.require_auth();

        if deadline <= env.ledger().timestamp() {
            return Err(AuctionError::InvalidDeadline);
        }

        let mut count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::AuctionCount)
            .unwrap_or(0);

        count += 1;

        let auction = Auction {
            id: count,
            seller: seller.clone(),
            item_name,
            token,
            highest_bid: starting_bid,
            highest_bidder: None,
            deadline,
            active: true,
        };

        env.storage()
            .instance()
            .set(&DataKey::Auction(count), &auction);

        env.storage()
            .instance()
            .set(&DataKey::AuctionCount, &count);

        AuctionCreated { auction_id: count }.publish(&env);

        Ok(count)
    }

    pub fn place_bid(
        env: Env,
        bidder: Address,
        auction_id: u64,
        amount: i128,
    ) -> Result<(), AuctionError> {
        bidder.require_auth();

        let mut auction: Auction = env
            .storage()
            .instance()
            .get(&DataKey::Auction(auction_id))
            .ok_or(AuctionError::AuctionNotFound)?;

        if !auction.active {
            return Err(AuctionError::AuctionClosed);
        }

        if env.ledger().timestamp() > auction.deadline {
            return Err(AuctionError::AuctionEnded);
        }

        if amount <= auction.highest_bid {
            return Err(AuctionError::BidTooLow);
        }

        let token_client = token::Client::new(&env, &auction.token);

        token_client.transfer_from(
            &bidder,
            &bidder,
            &env.current_contract_address(),
            &amount,
        );

        if let Some(prev) = auction.highest_bidder.clone() {
            let refund: i128 = env
                .storage()
                .instance()
                .get(&DataKey::Refund(prev.clone()))
                .unwrap_or(0);

            env.storage()
                .instance()
                .set(&DataKey::Refund(prev), &(refund + auction.highest_bid));
        }

        auction.highest_bid = amount;
        auction.highest_bidder = Some(bidder.clone());

        env.storage()
            .instance()
            .set(&DataKey::Auction(auction_id), &auction);

        BidPlaced {
            auction_id,
            bidder,
            amount,
        }
        .publish(&env);

        Ok(())
    }

    pub fn claim_refund(env: Env, user: Address) -> Result<i128, AuctionError> {
        user.require_auth();

        let amount: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Refund(user.clone()))
            .unwrap_or(0);

        if amount == 0 {
            return Err(AuctionError::NoRefundAvailable);
        }

        env.storage()
            .instance()
            .remove(&DataKey::Refund(user.clone()));

        RefundClaimed {
            user: user.clone(),
            amount,
        }
        .publish(&env);

        Ok(amount)
    }

    pub fn finalize_auction(env: Env, auction_id: u64) -> Result<(), AuctionError> {
        let mut auction: Auction = env
            .storage()
            .instance()
            .get(&DataKey::Auction(auction_id))
            .ok_or(AuctionError::AuctionNotFound)?;

        if env.ledger().timestamp() < auction.deadline {
            return Err(AuctionError::AuctionClosed);
        }

        auction.active = false;

        let token_client = token::Client::new(&env, &auction.token);

        if let Some(winner) = auction.highest_bidder.clone() {
            token_client.transfer(
                &env.current_contract_address(),
                &auction.seller,
                &auction.highest_bid,
            );

            AuctionFinalized {
                auction_id,
                winner,
            }
            .publish(&env);
        }

        env.storage()
            .instance()
            .set(&DataKey::Auction(auction_id), &auction);

        Ok(())
    }

    pub fn get_auction(env: Env, auction_id: u64) -> Option<Auction> {
        env.storage()
            .instance()
            .get(&DataKey::Auction(auction_id))
    }
}