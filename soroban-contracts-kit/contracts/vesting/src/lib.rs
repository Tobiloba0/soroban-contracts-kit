#![no_std]
//! Linear token vesting with an optional cliff.
//!
//! A funder locks `total` tokens for a beneficiary. Nothing can be claimed
//! before `start + cliff`. After the cliff, tokens unlock linearly from `start`
//! until `start + duration`, when everything is vested.

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, token, Address, Env,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Schedule {
    pub token: Address,
    pub beneficiary: Address,
    pub total: i128,
    pub start: u64,
    pub cliff: u64,
    pub duration: u64,
    pub claimed: i128,
}

#[contracttype]
pub enum DataKey {
    Schedule,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    InvalidSchedule = 3,
    NotInitialized = 4,
    NothingToClaim = 5,
    MathOverflow = 6,
}

#[contract]
pub struct VestingContract;

#[contractimpl]
impl VestingContract {
    /// Create the schedule and pull `total` tokens from the funder.
    #[allow(clippy::too_many_arguments)]
    pub fn init(
        env: Env,
        funder: Address,
        token: Address,
        beneficiary: Address,
        total: i128,
        start: u64,
        cliff: u64,
        duration: u64,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Schedule) {
            return Err(Error::AlreadyInitialized);
        }
        if total <= 0 {
            return Err(Error::InvalidAmount);
        }
        if duration == 0 || cliff > duration {
            return Err(Error::InvalidSchedule);
        }
        funder.require_auth();

        token::Client::new(&env, &token).transfer(
            &funder,
            &env.current_contract_address(),
            &total,
        );

        let schedule = Schedule {
            token,
            beneficiary,
            total,
            start,
            cliff,
            duration,
            claimed: 0,
        };
        env.storage().instance().set(&DataKey::Schedule, &schedule);
        env.events().publish((symbol_short!("created"),), total);
        Ok(())
    }

    /// Transfer all currently claimable tokens to the beneficiary.
    /// Returns the amount transferred.
    pub fn claim(env: Env) -> Result<i128, Error> {
        let mut schedule = Self::load(&env)?;
        schedule.beneficiary.require_auth();

        let vested = Self::compute_vested(&schedule, env.ledger().timestamp())?;
        let claimable = vested - schedule.claimed;
        if claimable <= 0 {
            return Err(Error::NothingToClaim);
        }

        schedule.claimed += claimable;
        env.storage().instance().set(&DataKey::Schedule, &schedule);

        token::Client::new(&env, &schedule.token).transfer(
            &env.current_contract_address(),
            &schedule.beneficiary,
            &claimable,
        );
        env.events().publish((symbol_short!("claimed"),), claimable);
        Ok(claimable)
    }

    /// Total amount vested at the current ledger time (claimed or not).
    pub fn vested(env: Env) -> Result<i128, Error> {
        let schedule = Self::load(&env)?;
        Self::compute_vested(&schedule, env.ledger().timestamp())
    }

    /// Amount the beneficiary could claim right now.
    pub fn claimable(env: Env) -> Result<i128, Error> {
        let schedule = Self::load(&env)?;
        let vested = Self::compute_vested(&schedule, env.ledger().timestamp())?;
        Ok(vested - schedule.claimed)
    }

    /// The full schedule.
    pub fn schedule(env: Env) -> Result<Schedule, Error> {
        Self::load(&env)
    }
}

impl VestingContract {
    fn load(env: &Env) -> Result<Schedule, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Schedule)
            .ok_or(Error::NotInitialized)
    }

    fn compute_vested(schedule: &Schedule, now: u64) -> Result<i128, Error> {
        if now < schedule.start.saturating_add(schedule.cliff) {
            return Ok(0);
        }
        let elapsed = now - schedule.start;
        if elapsed >= schedule.duration {
            return Ok(schedule.total);
        }
        schedule
            .total
            .checked_mul(elapsed as i128)
            .map(|v| v / schedule.duration as i128)
            .ok_or(Error::MathOverflow)
    }
}

#[cfg(test)]
mod test;
