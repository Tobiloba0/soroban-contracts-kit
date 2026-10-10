#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, token, Address, Env};

struct Setup<'a> {
    client: EscrowContractClient<'a>,
    token: token::Client<'a>,
    depositor: Address,
    beneficiary: Address,
    arbiter: Address,
    token_addr: Address,
}

fn setup<'a>(env: &Env) -> Setup<'a> {
    env.mock_all_auths();

    let admin = Address::generate(env);
    let token_addr = env.register_stellar_asset_contract_v2(admin).address();
    let depositor = Address::generate(env);
    let beneficiary = Address::generate(env);
    let arbiter = Address::generate(env);

    token::StellarAssetClient::new(env, &token_addr).mint(&depositor, &1_000);

    let id = env.register(EscrowContract, ());
    Setup {
        client: EscrowContractClient::new(env, &id),
        token: token::Client::new(env, &token_addr),
        depositor,
        beneficiary,
        arbiter,
        token_addr,
    }
}

fn fund(s: &Setup) {
    s.client.init(
        &s.depositor,
        &s.beneficiary,
        &s.arbiter,
        &s.token_addr,
        &400,
    );
}

#[test]
fn init_locks_funds() {
    let env = Env::default();
    let s = setup(&env);
    fund(&s);

    assert_eq!(s.token.balance(&s.depositor), 600);
    assert_eq!(s.token.balance(&s.client.address), 400);
    assert_eq!(s.client.status(), Status::Funded);
}

#[test]
fn depositor_can_release_to_beneficiary() {
    let env = Env::default();
    let s = setup(&env);
    fund(&s);

    s.client.release(&s.depositor);

    assert_eq!(s.token.balance(&s.beneficiary), 400);
    assert_eq!(s.client.status(), Status::Released);
}

#[test]
fn arbiter_can_refund_depositor() {
    let env = Env::default();
    let s = setup(&env);
    fund(&s);

    s.client.refund(&s.arbiter);

    assert_eq!(s.token.balance(&s.depositor), 1_000);
    assert_eq!(s.client.status(), Status::Refunded);
}

#[test]
fn stranger_cannot_release() {
    let env = Env::default();
    let s = setup(&env);
    fund(&s);

    let stranger = Address::generate(&env);
    assert_eq!(s.client.try_release(&stranger), Err(Ok(Error::Unauthorized)));
}

#[test]
fn cannot_release_twice() {
    let env = Env::default();
    let s = setup(&env);
    fund(&s);

    s.client.release(&s.arbiter);
    assert_eq!(s.client.try_release(&s.arbiter), Err(Ok(Error::NotFunded)));
    assert_eq!(s.client.try_refund(&s.arbiter), Err(Ok(Error::NotFunded)));
}

#[test]
fn cannot_init_twice() {
    let env = Env::default();
    let s = setup(&env);
    fund(&s);

    let res = s.client.try_init(
        &s.depositor,
        &s.beneficiary,
        &s.arbiter,
        &s.token_addr,
        &100,
    );
    assert_eq!(res, Err(Ok(Error::AlreadyInitialized)));
}
