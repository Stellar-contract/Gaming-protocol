#![no_std]
use soroban_sdk::{contract, contractimpl, Symbol, Env};

#[contract]
pub struct GamingContract;

#[contractimpl]
impl GamingContract {
    pub fn hello(env: Env, to: Symbol) -> Symbol {
        to
    }
}
