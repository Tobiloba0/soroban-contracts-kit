#![no_std]
//! Three-party token escrow.
//!
//! A depositor locks `amount` of a token in the contract. The funds can then be
//! released to the beneficiary (by the depositor or the arbiter) or refunded to
//! the depositor (by the arbiter or the beneficiary).

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, token, Address, Env,
};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    Funded,
    Released,
    Refunded,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub depositor: Address,
    pub beneficiary: Address,
    pub arbiter: Address,
    pub token: Address,
    pub amount: i128,
}

#[contracttype]
pub enum DataKey {
    Config,
    Status,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    NotInitialized = 3,
    NotFunded = 4,
    Unauthorized = 5,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Create the escrow and pull `amount` tokens from the depositor.
    pub fn init(
        env: Env,
        depositor: Address,
        beneficiary: Address,
        arbiter: Address,
        token: Address,
        amount: i128,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Config) {
            return Err(Error::AlreadyInitialized);
        }
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        depositor.require_auth();

        token::Client::new(&env, &token).transfer(
            &depositor,
            &env.current_contract_address(),
            &amount,
        );

        let config = Config {
            depositor,
            beneficiary,
            arbiter,
            token,
            amount,
        };
        env.storage().instance().set(&DataKey::Config, &config);
        env.storage()
            .instance()
            .set(&DataKey::Status, &Status::Funded);
        env.events().publish((symbol_short!("funded"),), amount);
        Ok(())
    }

    /// Send the funds to the beneficiary. Callable by the depositor or arbiter.
    pub fn release(env: Env, caller: Address) -> Result<(), Error> {
        let config = Self::load_config(&env)?;
        Self::ensure_funded(&env)?;
        caller.require_auth();
        if caller != config.depositor && caller != config.arbiter {
            return Err(Error::Unauthorized);
        }
        Self::payout(&env, &config, &config.beneficiary, Status::Released);
        env.events()
            .publish((symbol_short!("released"),), config.amount);
        Ok(())
    }

    /// Return the funds to the depositor. Callable by the arbiter or beneficiary.
    pub fn refund(env: Env, caller: Address) -> Result<(), Error> {
        let config = Self::load_config(&env)?;
        Self::ensure_funded(&env)?;
        caller.require_auth();
        if caller != config.arbiter && caller != config.beneficiary {
            return Err(Error::Unauthorized);
        }
        Self::payout(&env, &config, &config.depositor, Status::Refunded);
        env.events()
            .publish((symbol_short!("refunded"),), config.amount);
        Ok(())
    }

    /// Current escrow status.
    pub fn status(env: Env) -> Result<Status, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Status)
            .ok_or(Error::NotInitialized)
    }

    /// Escrow parameters.
    pub fn config(env: Env) -> Result<Config, Error> {
        Self::load_config(&env)
    }
}

impl EscrowContract {
    fn load_config(env: &Env) -> Result<Config, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Config)
            .ok_or(Error::NotInitialized)
    }

    fn ensure_funded(env: &Env) -> Result<(), Error> {
        let status: Status = env
            .storage()
            .instance()
            .get(&DataKey::Status)
            .ok_or(Error::NotInitialized)?;
        if status != Status::Funded {
            return Err(Error::NotFunded);
        }
        Ok(())
    }

    // Update state first, then move tokens (checks-effects-interactions).
    fn payout(env: &Env, config: &Config, to: &Address, new_status: Status) {
        env.storage().instance().set(&DataKey::Status, &new_status);
        token::Client::new(env, &config.token).transfer(
            &env.current_contract_address(),
            to,
            &config.amount,
        );
    }
}

#[cfg(test)]
mod test;
