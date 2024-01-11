use core::ops::{Add, Div, Mul};

use crate::enums::Address;
use crate::interfaces::cep18::CEP18;
use crate::{
    error::Error,
    utils::{self, get_current_address, get_key},
};
use alloc::{
    string::{String, ToString},
    vec,
};
use casper_contract::contract_api::{runtime, storage};
use casper_types::{
    account::AccountHash,
    contracts::NamedKeys,
    runtime_args,
    CLType::{self, URef},
    ContractHash, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter,
    RuntimeArgs, U256,
};

// Variables
// This is the address of the who deployed the stake.
const OWNER: &str = "owner";
// Duration of rewards to be paid out (in seconds)
const DURATION: &str = "duration";
// Timestamp of when finish stake program
const FINISH_AT: &str = "finish_at";
// Minimum of last updated time and reward finish time
const UPDATE_AT: &str = "update_at";
//The reward amount determined by the owner.
const AMOUNT: &str = "amount";
// The stake wallet to which all stakes will be transferred.
const TOTAL_SUPPLY: &str = "total_supply";
// The token type to be staked
const STAKED_TOKEN: &str = "staked_token";
// The amount of tokens to be stake, withdraw and earned

const REWARD_RATE: &str = "reward_rate";
const REWARD_PER_TOKEN_STORED: &str = "reward_per_token_stored";

const DECIMALS: &str = "decimals";

// Dictionary
const BALANCE_OF: &str = "balance_of";

// Entry Points
const ENTRY_POINT_INIT: &str = "init";
const ENTRY_POINT_NOTIFY_REWARD_AMOUNT: &str = "notify_reward_amount";
// const ENTRY_POINT_SET_REWARD_DURATION: &str = "set_rewards_duration";
const ENTRY_POINT_STAKE: &str = "stake";

#[no_mangle]
pub extern "C" fn stake() {
    let amount: U256 = runtime::get_named_arg(AMOUNT);

    if amount.is_zero() {
        runtime::revert(Error::StakeAmountError);
    }

    let staker: Key = runtime::get_caller().into();
    let contract_address: Address = get_current_address();
    let staked_token: Key = utils::read_from(STAKED_TOKEN);

    let total_supply: U256 = utils::read_from(TOTAL_SUPPLY);

    let cep18: CEP18 = CEP18::new(staked_token.into_hash().map(ContractHash::new).unwrap());
    cep18.transfer_from(staker, contract_address.into(), amount);

    let balance_of_dict = *runtime::get_key(BALANCE_OF).unwrap().as_uref().unwrap();

    let staker_item_key: String = utils::encode_dictionary_item_key(staker);

    let balance: U256 = match storage::dictionary_get::<U256>(balance_of_dict, &staker_item_key) {
        Ok(Some(balance)) => balance,
        _ => U256::zero(),
    };


    storage::dictionary_put(balance_of_dict, &staker_item_key, balance.add(amount));

    runtime::put_key(
        TOTAL_SUPPLY,
        storage::new_uref(total_supply.add(amount)).into(),
    );
}

// #[no_mangle]
// pub extern "C" fn set_rewards_duration() {
//     let finish_at: u64 = utils::read_from(FINISH_AT);
//     let now: u64 = runtime::get_blocktime().into();

//     if finish_at.gt(&now) {
//         runtime::revert(Error::RewardDurationError);
//     }

//     let duration: u64 = runtime::get_named_arg(DURATION);
//     runtime::put_key(DURATION, storage::new_uref(duration).into());
// }

#[no_mangle]
pub extern "C" fn notify_reward_amount() {
    only_owner();

    let amount: U256 = runtime::get_named_arg(AMOUNT);

    let finish_at: u64 = utils::read_from(FINISH_AT);
    let now: u64 = runtime::get_blocktime().into();

    let total_supply: U256 = utils::read_from(TOTAL_SUPPLY);
    let reward_per_token_stored: U256 = utils::read_from(REWARD_PER_TOKEN_STORED);

    let reward_rate: U256 = utils::read_from(REWARD_RATE);
    let update_at: u64 = utils::read_from(UPDATE_AT);

    let token_decimal: u8 = utils::read_from(DECIMALS);

    let duration: u64 = utils::read_from(DURATION);

    let reward_per_token_stored_mutate = update_reward(
        finish_at,
        now,
        token_decimal,
        total_supply,
        reward_rate,
        reward_per_token_stored,
        update_at,
    );

    let reward_rate_mut: U256;
    if now.ge(&finish_at) {
        reward_rate_mut = amount.div(U256::from(duration));
    } else {
        let remaining_rewards: U256 = U256::from(finish_at - now).mul(reward_rate);
        reward_rate_mut = U256::from(amount + remaining_rewards).div(U256::from(duration));
    }

    if reward_rate_mut.is_zero() {
        runtime::revert(Error::RewardRateError)
    }

    runtime::put_key(
        REWARD_PER_TOKEN_STORED,
        storage::new_uref(reward_per_token_stored_mutate).into(),
    );
    runtime::put_key(REWARD_RATE, storage::new_uref(reward_rate_mut).into());
    runtime::put_key(UPDATE_AT, storage::new_uref(now).into());
    runtime::put_key(FINISH_AT, storage::new_uref(now + duration).into());
    //runtime::put_key(REWARD, storage::new_uref(reward).into());
}

#[no_mangle]
pub extern "C" fn init() {
    runtime::put_key(UPDATE_AT, storage::new_uref(0u64).into());
    runtime::put_key(
        REWARD_PER_TOKEN_STORED,
        storage::new_uref(U256::zero()).into(),
    );
    runtime::put_key(TOTAL_SUPPLY, storage::new_uref(U256::zero()).into());
    runtime::put_key(REWARD_RATE, storage::new_uref(U256::zero()).into());
    runtime::put_key(FINISH_AT, storage::new_uref(0u64).into());

    storage::new_dictionary(BALANCE_OF).unwrap_or_default();
}

// constructor
#[no_mangle]
pub extern "C" fn call() {
    let staked_token: Key = runtime::get_named_arg(STAKED_TOKEN);
    let duration: u64 = runtime::get_named_arg(DURATION);

    let cep18: CEP18 = CEP18::new(staked_token.into_hash().map(ContractHash::new).unwrap());
    let decimals: u8 = cep18.decimals();

    let owner: AccountHash = runtime::get_caller();

    let mut named_keys = NamedKeys::new();

    named_keys.insert(OWNER.to_string(), storage::new_uref(owner).into());
    named_keys.insert(
        STAKED_TOKEN.to_string(),
        storage::new_uref(staked_token).into(),
    );
    named_keys.insert(DECIMALS.to_string(), storage::new_uref(decimals).into());
    named_keys.insert(DURATION.to_string(), storage::new_uref(duration).into());

    let init_entry_point: EntryPoint = EntryPoint::new(
        ENTRY_POINT_INIT,
        vec![],
        URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let notify_reward_amount_entry_point: EntryPoint = EntryPoint::new(
        ENTRY_POINT_NOTIFY_REWARD_AMOUNT,
        vec![Parameter::new(AMOUNT, CLType::U256)],
        URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    // let set_rewards_duration_entry_point: EntryPoint = EntryPoint::new(
    //     ENTRY_POINT_SET_REWARD_DURATION,
    //     vec![Parameter::new(DURATION, U64)],
    //     URef,
    //     EntryPointAccess::Public,
    //     EntryPointType::Contract
    // );

    let stake_entry_point: EntryPoint = EntryPoint::new(
        ENTRY_POINT_STAKE,
        vec![Parameter::new(AMOUNT, CLType::U256)],
        URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let mut entry_points: EntryPoints = EntryPoints::new();

    entry_points.add_entry_point(init_entry_point);
    entry_points.add_entry_point(notify_reward_amount_entry_point);
    // entry_points.add_entry_point(set_rewards_duration_entry_point);
    entry_points.add_entry_point(stake_entry_point);

    // let contract_id: String = "ID_".to_owned() + &now.to_string();

    let ph_text: String = String::from("stake_package_hash_");
    let ch_text: String = String::from("stake_contract_hash_");
    let au_text: String = String::from("stake_access_uref_");

    let package_hash_text = ph_text;
    let contract_hash_text = ch_text;
    let uref_name_text = au_text;

    let (contract_hash, _contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(package_hash_text),
        Some(uref_name_text),
    );

    runtime::put_key(&contract_hash_text, contract_hash.into());

    runtime::call_contract::<()>(contract_hash, ENTRY_POINT_INIT, runtime_args! {});
}

// internal functions
pub fn only_owner() {
    let admin: AccountHash = get_key(OWNER);
    let caller: AccountHash = runtime::get_caller();
    if admin != caller {
        runtime::revert(Error::AdminError)
    }
}

pub fn update_reward(
    finish_at: u64,
    now: u64,
    token_decimal: u8,
    total_supply: U256,
    reward_rate: U256,
    reward_per_token_stored: U256,
    update_at: u64,
) -> (U256, u64) {
    let reward_per_token_stored: U256 = reward_per_token(
        token_decimal,
        total_supply,
        reward_rate,
        reward_per_token_stored,
        finish_at,
        now,
        update_at,
    );
    let update_at: u64 = last_time_reward_applicable(finish_at, now);

    let admin: AccountHash = get_key(OWNER);
    let caller: AccountHash = runtime::get_caller();
    if admin != caller {}

    (reward_per_token_stored, update_at)
}

fn reward_per_token(
    token_decimal: u8,
    total_supply: U256,
    reward_rate: U256,
    reward_per_token_stored: U256,
    finish_at: u64,
    now: u64,
    update_at: u64,
) -> U256 {
    let decimal: U256 = U256::from(10u64.pow(token_decimal as u32));

    if total_supply.is_zero() {
        return reward_per_token_stored;
    }

    let time_elapsed = last_time_reward_applicable(finish_at, now) - update_at;
    let reward_increase =
        (reward_rate * time_elapsed * decimal) / U256::from(total_supply.as_u64());

    reward_per_token_stored + U256::from(reward_increase.as_u64())
}

pub fn last_time_reward_applicable(finish_at: u64, now: u64) -> u64 {
    min(finish_at, now)
}

pub fn min(value_1: u64, value_2: u64) -> u64 {
    if value_1.le(&value_2) {
        value_1
    } else {
        value_2
    }
}
