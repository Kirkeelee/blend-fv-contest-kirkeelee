use cvlr::{cvlr_assert, cvlr_assume};
use cvlr_soroban_derive::rule;
use soroban_sdk::{Address, Env};
use crate::storage;

use crate::backstop::{execute_draw, execute_donate};

// Rule to check that executing a draw decreases the pool balance
#[rule]
pub fn execute_draw_decreases_pool_balance(e: &Env, pool_address: Address, amount: i128, to: Address) {
    cvlr_assume!(amount >= 0);
    let mut pool_balance = storage::get_pool_balance(e, &pool_address);
    let initial_tokens = pool_balance.tokens;

    execute_draw(e, &pool_address, amount, &to);

    let new_pool_balance = storage::get_pool_balance(e, &pool_address);
    cvlr_assert!(new_pool_balance.tokens == initial_tokens - amount);
}

// Rule to check that executing a donation increases the pool balance
#[rule]
pub fn execute_donate_increases_pool_balance(e: &Env, from: Address, pool_address: Address, amount: i128) {
    cvlr_assume!(amount >= 0 && from != pool_address && from != e.current_contract_address());
    let mut pool_balance = storage::get_pool_balance(e, &pool_address);
    let initial_tokens = pool_balance.tokens;

    execute_donate(e, &from, &pool_address, amount);

    let new_pool_balance = storage::get_pool_balance(e, &pool_address);
    cvlr_assert!(new_pool_balance.tokens == initial_tokens + amount);
}
