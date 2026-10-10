#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, token, Address, Env};

const TOTAL: i128 = 10_000;
const START: u64 = 1_000;
const CLIFF: u64 = 100;
const DURATION: u64 = 1_000;

struct Setup<'a> {
    env: Env,
    client: VestingContractClient<'a>,
    token: token::Client<'a>,
    funder: Address,
    beneficiary: Address,
    token_addr: Address,
}

fn setup<'a>() -> Setup<'a> {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_addr = env.register_stellar_asset_contract_v2(admin).address();
    let funder = Address::generate(&env);
    let beneficiary = Address::generate(&env);
    token::StellarAssetClient::new(&env, &token_addr).mint(&funder, &TOTAL);

    let id = env.register(VestingContract, ());
    let client = VestingContractClient::new(&env, &id);
    let token = token::Client::new(&env, &token_addr);

    Setup {
        env,
        client,
        token,
        funder,
        beneficiary,
        token_addr,
    }
}

fn start_schedule(s: &Setup) {
    s.client.init(
        &s.funder,
        &s.token_addr,
        &s.beneficiary,
        &TOTAL,
        &START,
        &CLIFF,
        &DURATION,
    );
}

#[test]
fn nothing_vests_before_cliff() {
    let s = setup();
    start_schedule(&s);

    s.env.ledger().set_timestamp(START + CLIFF - 1);
    assert_eq!(s.client.vested(), 0);
    assert_eq!(s.client.try_claim(), Err(Ok(Error::NothingToClaim)));
}

#[test]
fn vests_linearly_after_cliff() {
    let s = setup();
    start_schedule(&s);

    s.env.ledger().set_timestamp(START + 500);
    assert_eq!(s.client.vested(), 5_000);
    assert_eq!(s.client.claim(), 5_000);
    assert_eq!(s.token.balance(&s.beneficiary), 5_000);
}

#[test]
fn fully_vested_after_duration() {
    let s = setup();
    start_schedule(&s);

    s.env.ledger().set_timestamp(START + DURATION + 10);
    assert_eq!(s.client.vested(), TOTAL);
    assert_eq!(s.client.claim(), TOTAL);
    assert_eq!(s.token.balance(&s.beneficiary), TOTAL);
}

#[test]
fn claims_only_the_new_portion() {
    let s = setup();
    start_schedule(&s);

    s.env.ledger().set_timestamp(START + 500);
    s.client.claim();

    s.env.ledger().set_timestamp(START + DURATION);
    assert_eq!(s.client.claimable(), 5_000);
    assert_eq!(s.client.claim(), 5_000);
    assert_eq!(s.client.try_claim(), Err(Ok(Error::NothingToClaim)));
}

#[test]
fn rejects_cliff_longer_than_duration() {
    let s = setup();
    let res = s.client.try_init(
        &s.funder,
        &s.token_addr,
        &s.beneficiary,
        &TOTAL,
        &START,
        &(DURATION + 1),
        &DURATION,
    );
    assert_eq!(res, Err(Ok(Error::InvalidSchedule)));
}

#[test]
fn cannot_init_twice() {
    let s = setup();
    start_schedule(&s);

    let res = s.client.try_init(
        &s.funder,
        &s.token_addr,
        &s.beneficiary,
        &TOTAL,
        &START,
        &CLIFF,
        &DURATION,
    );
    assert_eq!(res, Err(Ok(Error::AlreadyInitialized)));
}
