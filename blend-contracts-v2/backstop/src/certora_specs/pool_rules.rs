use crate::certora_specs::mocks::conversions::certora_convert_to_shares;
use crate::certora_specs::mocks::conversions::certora_convert_to_tokens;
use crate::storage;
use crate::PoolBalance;
use cvlr_soroban_derive::rule;
use soroban_sdk::{Address, Env};

use cvlr::asserts::{cvlr_assert, cvlr_assume};

// converting 0 units of tokens/shares will lead to 0 units
#[rule]
pub fn conversion_of_zero(pool_balance: &mut PoolBalance) {
    let tokens = pool_balance.convert_to_tokens(0);
    let shares = pool_balance.convert_to_shares(0);
    cvlr_assert!(tokens == 0 && shares == 0);
}

// token -> shares gives tokens when pool has 0 shares
#[rule]
pub fn conversion_pool_zero_shares(pool_balance: &mut PoolBalance, tokens: i128) {
    cvlr_assume!(pool_balance.shares == 0); //, "pool has 0 shares");
    let shares = pool_balance.convert_to_shares(tokens);
    cvlr_assert!(shares == tokens);
    cvlr_assert!(pool_balance.shares == 0);
}

// simpler correctness of token conversion for i64 variant
#[rule]
pub fn simple_token_roundtrip_correct(pool_shares: i64, pool_tokens: i64, tokens: i64) {
    cvlr_assume!(
        tokens >= 0 && pool_shares > 0 && pool_tokens > 0);
    let tokens_res = certora_convert_to_tokens(
        pool_shares,
        pool_tokens,
        certora_convert_to_shares(pool_shares, pool_tokens, tokens),
    );
    cvlr_assert!(tokens >= tokens_res);
}

// simpler correctness of share conversion for i64 variant
#[rule]
pub fn simple_share_roundtrip_correct(pool_shares: i64, pool_tokens: i64, shares: i64) {
    cvlr_assume!(
        shares >= 0 && pool_shares > 0 && pool_tokens > 0); 
    let shares_res = certora_convert_to_shares(
        pool_shares,
        pool_tokens,
        certora_convert_to_tokens(pool_shares, pool_tokens, shares),
    );
    cvlr_assert!(shares >= shares_res);
}


// Rule to check the effects of the withdraw function
#[rule]
pub fn check_withdraw_effects(pool_balance: &mut PoolBalance, tokens: i128, shares: i128, user: Address, e: &Env) {
    let initial_tokens = pool_balance.tokens;
    let initial_shares = pool_balance.shares;
    let initial_q4w = pool_balance.q4w;

    // Assume the pool has enough tokens and shares for the withdrawal
    cvlr_assume!(initial_tokens >= tokens && initial_shares >= shares && initial_q4w >= shares);

    // Call the withdraw function
    pool_balance.withdraw(e, tokens, shares);

    // Assert the expected outcomes
    cvlr_assert!(pool_balance.tokens == initial_tokens - tokens);
    cvlr_assert!(pool_balance.shares == initial_shares - shares);
    cvlr_assert!(pool_balance.q4w == initial_q4w - shares);
}


// Rule to check the effects of the deposit function
#[rule]
pub fn check_deposit_effects(pool_balance: &mut PoolBalance, tokens: i128, shares: i128) {
    let initial_tokens = pool_balance.tokens;
    let initial_shares = pool_balance.shares;

    // Call the deposit function
    pool_balance.deposit(tokens, shares);

    // Assert the expected outcomes
    cvlr_assert!(pool_balance.tokens == initial_tokens + tokens);
    cvlr_assert!(pool_balance.shares == initial_shares + shares);
}

// Rule to check the effects of the queue_for_withdraw function
#[rule]
pub fn check_queue_for_withdraw_effects(pool_balance: &mut PoolBalance, shares: i128) {
    let initial_q4w = pool_balance.q4w;

    // Call the queue_for_withdraw function
    pool_balance.queue_for_withdraw(shares);

    // Assert the expected outcomes
    cvlr_assert!(pool_balance.q4w == initial_q4w + shares);
}

// Rule to check the effects of the dequeue_q4w function
#[rule]
pub fn check_dequeue_q4w_effects(pool_balance: &mut PoolBalance, shares: i128, e: &Env) {
    let initial_q4w = pool_balance.q4w;

    // Assume the pool has enough shares queued for withdrawal
    cvlr_assume!(initial_q4w >= shares);

    // Call the dequeue_q4w function
    pool_balance.dequeue_q4w(e, shares);

    // Assert the expected outcomes
    cvlr_assert!(pool_balance.q4w == initial_q4w - shares);
}