use cvlr::{cvlr_assert, cvlr_assume};
use cvlr::nondet::Nondet;
use cvlr_soroban_derive::rule;
use soroban_sdk::{Address, Env};

use crate::backstop::{execute_queue_withdrawal, execute_dequeue_withdrawal, execute_withdraw};
use crate::{PoolBalance};


// Implement the Nondet trait for PoolBalance
impl Nondet for PoolBalance {
    fn nondet() -> Self {
        PoolBalance {
            shares: i128::nondet(),
            tokens: i128::nondet(),
            q4w: i128::nondet(),
        }
    }
}

// shares to withdraw must be nonnegative
#[rule]
pub fn amount_to_withdraw_nonnegative(e: &Env, from: Address, pool_address: Address, shares: i128) {
    cvlr_assume!(shares < 0);
    execute_withdraw(e, &from, &pool_address, shares);
    cvlr_assert!(false);
}

// withdraw queue entries are all positive
// needs -smt_preciseBitwiseOps
#[rule]
pub fn withdraw_queue_only_positive(e: Env, from: Address, pool_address: Address, amount: i128) {
    cvlr_assume!(amount < 0);
    execute_queue_withdrawal(&e, &from, &pool_address, amount);
    cvlr_assert!(false);
}

/*


// Rule to check the effects of the execute_queue_withdrawal function
#[rule]
pub fn check_execute_queue_withdrawal(e: &Env, from: Address, pool_address: Address, amount: i128) {
    let mut pool_balance: PoolBalance = cvlr::nondet();

    let initial_q4w = pool_balance.q4w;

    // Assume the amount is non-negative
    cvlr_assume!(amount >= 0);

    // Call the execute_queue_withdrawal function
    execute_queue_withdrawal(e, &from, &pool_address, amount);

    // Assert the expected outcomes
    cvlr_assert!(pool_balance.q4w == initial_q4w + amount);
}

// Rule to check the effects of the execute_dequeue_withdrawal function
#[rule]
pub fn check_execute_dequeue_withdrawal(e: &Env, from: Address, pool_address: Address, amount: i128) {
    let mut pool_balance: PoolBalance = cvlr::nondet();

    let initial_q4w = pool_balance.q4w;

    // Assume the amount is non-negative and there are enough shares queued for withdrawal
    cvlr_assume!(amount >= 0 && initial_q4w >= amount);

    // Call the execute_dequeue_withdrawal function
    execute_dequeue_withdrawal(e, &from, &pool_address, amount);

    // Assert the expected outcomes
    cvlr_assert!(pool_balance.q4w == initial_q4w - amount);
}

// Rule to check the effects of the execute_withdraw function
#[rule]
pub fn check_execute_withdraw(e: &Env, from: Address, pool_address: Address, shares: i128) {
    let mut pool_balance: PoolBalance = cvlr::nondet();

    let initial_tokens = pool_balance.tokens;
    let initial_shares = pool_balance.shares;

    // Assume the amount is non-negative
    cvlr_assume!(shares >= 0);

    // Call the execute_withdraw function
    let withdrawn_tokens = execute_withdraw(e, &from, &pool_address, shares);

    // Assert the expected outcomes
    cvlr_assert!(pool_balance.tokens == initial_tokens - withdrawn_tokens);
    cvlr_assert!(pool_balance.shares == initial_shares - shares);
}*/