use cvlr::{cvlr_assert, cvlr_assume};
use cvlr::nondet::Nondet;
use cvlr_soroban_derive::rule;
use soroban_sdk::{Env, Vec};

use crate::backstop::{UserBalance, Q4W};
// Custom wrapper type around Vec<Q4W>
pub struct Q4WVec(Vec<Q4W>);
// Implement the Nondet trait for Q4W
impl Nondet for Q4W {
    fn nondet() -> Self {
        Q4W {
            amount: i128::nondet(),
            exp: u64::nondet(),
        }
    }
}

// Implement the Nondet trait for Q4WVec
impl Nondet for Q4WVec {
    fn nondet() -> Self {
        let env = Env::default(); // Create a default environment
        let mut vec = Vec::new(&env);
        let len: usize = usize::nondet();
        for _ in 0..len {
            vec.push_back(Q4W::nondet());
        }
        Q4WVec(vec)
    }
}

// Implement the Nondet trait for UserBalance
impl Nondet for UserBalance {
    fn nondet() -> Self {
        UserBalance {
            shares: i128::nondet(),
            q4w: Q4WVec::nondet().0,
        }
    }
}

// deposit should increase user shares
#[rule]
pub fn add_shares_increases_user_shares(user_balance: &mut UserBalance, shares: i128) {
    cvlr_assume!(shares >= 0);
    let user_shares_before = user_balance.shares;
    user_balance.add_shares(shares);
    let user_shares_after = user_balance.shares;
    cvlr_assert!(user_shares_after == user_shares_before + shares);
}


// queue shares for withdrawal should add to q4w
#[rule]
pub fn queue_shares_for_withdrawal_adds_to_q4w(e: &Env, user_balance: &mut UserBalance, shares: i128) {
    cvlr_assume!(shares >= 0);
    let q4w_before = user_balance.q4w.clone();
    user_balance.queue_shares_for_withdrawal(e, shares);
    let q4w_after = user_balance.q4w.clone();
    cvlr_assert!(q4w_after.len() == q4w_before.len() + 1);
    cvlr_assert!(q4w_after.last().unwrap().amount == shares);
}
/*
// dequeue shares for withdrawal should remove from q4w
#[rule]
pub fn dequeue_shares_for_withdrawal_removes_from_q4w(e: &Env, user_balance: &mut UserBalance, shares: i128) {
    cvlr_assume!(shares >= 0 && !user_balance.q4w.is_empty() && user_balance.q4w.last().unwrap().amount >= shares);
    let q4w_before = user_balance.q4w.clone();
    user_balance.dequeue_shares(e, shares);
    let q4w_after = user_balance.q4w.clone();
    cvlr_assert!(q4w_after.len() == q4w_before.len() - 1);
}
*/
