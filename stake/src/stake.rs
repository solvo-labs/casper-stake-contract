use core::ops::{ Add, Div, Mul, Sub };

use crate::enums::Address;
use crate::interfaces::cep18::CEP18;
use crate::{ error::Error, utils::{ self, get_current_address } };
use alloc::{ string::{ String, ToString }, vec };
use casper_contract::contract_api::{ runtime, storage };
use casper_types::{
    account::AccountHash,
    contracts::NamedKeys,
    runtime_args,
    CLType::{ self, URef },
    ContractHash,
    EntryPoint,
    EntryPointAccess,
    EntryPointType,
    EntryPoints,
    Key,
    Parameter,
    RuntimeArgs,
    U256,
};

// Variables
const TOKEN: &str = "token";
const FIXED_ARP: &str = "fixed_apr";
const MIN_APR: &str = "min_apr";
const MAX_APR: &str = "max_apr";
const MAX_CAP: &str = "max_cap";
const MIN_STAKE: &str = "min_stake";
const MAX_STAKE: &str = "max_stake";
const LOCK_PERIOD: &str = "lock_period";
const DEPOSIT_START_TIME: &str = "deposit_start_time";
const DEPOSIT_END_TIME: &str = "deposit_end_time";
const AMOUNT: &str = "amount";
const TOTAL_SUPPLY: &str = "total_supply";
const STORAGE_KEY: &str = "storage_key";

// Dictionaries
const STAKES_DICT: &str = "stakes_dict";
const LAST_CLAIM_TIME_DICT: &str = "last_claim_time_dict";
const TOTAL_STAKED_DICT: &str = "total_staked_dict";

// Entry points
const ENTRY_POINT_INIT: &str = "init";
const ENTRY_POINT_STAKE: &str = "stake";
const ENTRY_POINT_UNSTAKE: &str = "unstake";
const ENTRY_POINT_CLAIM: &str = "claim_reward";

#[no_mangle]
pub extern "C" fn stake() {
    let now: u64 = runtime::get_blocktime().into();
    let deposit_start_time: u64 = utils::read_from(DEPOSIT_START_TIME);
    let deposit_end_time: u64 = utils::read_from(DEPOSIT_END_TIME);

    if deposit_start_time.gt(&now) || now.gt(&deposit_end_time) {
        runtime::revert(Error::DepositPeriodEnded);
    }

    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let min_stake: U256 = utils::read_from(MIN_STAKE);
    let max_stake: U256 = utils::read_from(MAX_STAKE);
    let max_cap: U256 = utils::read_from(MAX_CAP);

    if min_stake.gt(&amount) || amount.gt(&max_stake) {
        runtime::revert(Error::StakeAmountError);
    }

    let staker: AccountHash = runtime::get_caller();
    let staker_item_key: String = utils::encode_dictionary_item_key(staker.into());
    let stake_dict = *runtime::get_key(STAKES_DICT).unwrap().as_uref().unwrap();

    let stake: U256 = match storage::dictionary_get::<U256>(stake_dict, &staker_item_key) {
        Ok(Some(stake)) => stake,
        _ => U256::zero(),
    };

    let total_stake_dict = *runtime::get_key(TOTAL_STAKED_DICT).unwrap().as_uref().unwrap();

    let total_stake: U256 = match
        storage::dictionary_get::<U256>(total_stake_dict, &staker_item_key)
    {
        Ok(Some(total_stake_amount)) => total_stake_amount,
        _ => U256::zero(),
    };

    let token: Key = utils::read_from(TOKEN);
    let cep18: CEP18 = CEP18::new(token.into_hash().map(ContractHash::new).unwrap());
    let staker_balance: U256 = cep18.balance_of(staker.into());

    if staker_balance.lt(&amount) {
        runtime::revert(Error::InsufficientBalance);
    }

    let last_claim_time = *runtime::get_key(LAST_CLAIM_TIME_DICT).unwrap().as_uref().unwrap();
    let contract_address: Address = get_current_address();

    cep18.transfer_from(staker.into(), contract_address.into(), amount);

    let total_staked_balance: U256 = total_stake.add(amount);

    if total_staked_balance.gt(&max_cap) {
        runtime::revert(Error::MaxCapacityError);
    }

    storage::dictionary_put(total_stake_dict, &staker_item_key, total_staked_balance);
    storage::dictionary_put(stake_dict, &staker_item_key, stake.add(amount));
    storage::dictionary_put(last_claim_time, &staker_item_key, now);

    let total_supply: U256 = utils::read_from(TOTAL_SUPPLY);
    runtime::put_key(TOTAL_SUPPLY, storage::new_uref(total_supply.add(amount)).into());
}

#[no_mangle]
pub extern "C" fn unstake() {
    let deposit_end_time: u64 = utils::read_from(DEPOSIT_END_TIME);
    let lock_period: u64 = utils::read_from(LOCK_PERIOD);
    let now: u64 = runtime::get_blocktime().into();

    if now.le(&deposit_end_time) {
        runtime::revert(Error::DepositPeriodEnded);
    }

    let amount: U256 = runtime::get_named_arg(AMOUNT);

    let staker: AccountHash = runtime::get_caller();
    let staker_item_key: String = utils::encode_dictionary_item_key(staker.into());
    let stake_dict = *runtime::get_key(STAKES_DICT).unwrap().as_uref().unwrap();
    let last_claim_time_dict = *runtime::get_key(LAST_CLAIM_TIME_DICT).unwrap().as_uref().unwrap();

    let user_stake: U256 = match storage::dictionary_get::<U256>(stake_dict, &staker_item_key) {
        Ok(Some(user_stake)) => user_stake,
        _ => U256::zero(),
    };

    let user_last_claim_time: u64 = match
        storage::dictionary_get::<u64>(last_claim_time_dict, &staker_item_key)
    {
        Ok(Some(user_last_claim_time)) => user_last_claim_time,
        _ => 0u64,
    };

    if user_last_claim_time.add(lock_period).gt(&now) {
        runtime::revert(Error::LocktimeError);
    }

    if amount.is_zero() && user_stake.lt(&amount) {
        runtime::revert(Error::InvalidUnstakeAmount);
    }

    let locked_period: u64 = utils::read_from(LOCK_PERIOD);
    let deposit_start_time: u64 = utils::read_from(DEPOSIT_START_TIME);
    let fixed_apr: u64 = utils::read_from(FIXED_ARP);
    let min_apr: u64 = utils::read_from(MIN_APR);
    let max_apr: u64 = utils::read_from(MAX_APR);

    let reward: U256 = calculate_reward(
        now,
        deposit_end_time,
        deposit_start_time,
        user_stake,
        user_last_claim_time,
        locked_period,
        fixed_apr,
        min_apr,
        max_apr
    );

    let token: Key = utils::read_from(TOKEN);

    let cep18: CEP18 = CEP18::new(token.into_hash().map(ContractHash::new).unwrap());
    cep18.transfer(staker.into(), amount.add(reward));

    storage::dictionary_put(stake_dict, &staker_item_key, user_stake.sub(amount));
    storage::dictionary_put(last_claim_time_dict, &staker_item_key, now);

    let total_supply: U256 = utils::read_from(TOTAL_SUPPLY);
    runtime::put_key(TOTAL_SUPPLY, storage::new_uref(total_supply.sub(amount)).into());
}

#[no_mangle]
pub extern "C" fn claim_reward() {
    let deposit_end_time: u64 = utils::read_from(DEPOSIT_END_TIME);
    let now: u64 = runtime::get_blocktime().into();
    let staker: AccountHash = runtime::get_caller();
    let staker_item_key: String = utils::encode_dictionary_item_key(staker.into());
    let stake_dict = *runtime::get_key(STAKES_DICT).unwrap().as_uref().unwrap();
    let last_claim_time_dict = *runtime::get_key(LAST_CLAIM_TIME_DICT).unwrap().as_uref().unwrap();

    let user_stake: U256 = match storage::dictionary_get::<U256>(stake_dict, &staker_item_key) {
        Ok(Some(user_stake)) => user_stake,
        _ => U256::zero(),
    };

    let user_last_claim_time: u64 = match
        storage::dictionary_get::<u64>(last_claim_time_dict, &staker_item_key)
    {
        Ok(Some(user_last_claim_time)) => user_last_claim_time,
        _ => 0u64,
    };

    let locked_period: u64 = utils::read_from(LOCK_PERIOD);
    let deposit_start_time: u64 = utils::read_from(DEPOSIT_START_TIME);
    let fixed_apr: u64 = utils::read_from(FIXED_ARP);
    let min_apr: u64 = utils::read_from(MIN_APR);
    let max_apr: u64 = utils::read_from(MAX_APR);

    let reward: U256 = calculate_reward(
        now,
        deposit_end_time,
        deposit_start_time,
        user_stake,
        user_last_claim_time,
        locked_period,
        fixed_apr,
        min_apr,
        max_apr
    );

    let token: Key = utils::read_from(TOKEN);

    let cep18: CEP18 = CEP18::new(token.into_hash().map(ContractHash::new).unwrap());
    cep18.transfer(staker.into(), reward);
}

#[no_mangle]
pub extern "C" fn init() {
    storage::new_dictionary(STAKES_DICT).unwrap_or_default();
    storage::new_dictionary(LAST_CLAIM_TIME_DICT).unwrap_or_default();
    storage::new_dictionary(TOTAL_STAKED_DICT).unwrap_or_default();
    runtime::put_key(TOTAL_SUPPLY, storage::new_uref(U256::zero()).into());
}

// constructor
#[no_mangle]
pub extern "C" fn call() {
    let token: Key = runtime::get_named_arg(TOKEN);
    let max_cap: U256 = runtime::get_named_arg(MAX_CAP);
    let min_stake: U256 = runtime::get_named_arg(MIN_STAKE);
    let max_stake: U256 = runtime::get_named_arg(MAX_STAKE);

    let fixed_apr: u64 = runtime::get_named_arg(FIXED_ARP);
    let min_apr: u64 = runtime::get_named_arg(MIN_APR);
    let max_apr: u64 = runtime::get_named_arg(MAX_APR);

    let lock_period: u64 = runtime::get_named_arg(LOCK_PERIOD);
    let deposit_start_time: u64 = runtime::get_named_arg(DEPOSIT_START_TIME);
    let deposit_end_time: u64 = runtime::get_named_arg(DEPOSIT_END_TIME);
    let storage_key: ContractHash = runtime::get_named_arg(STORAGE_KEY);

    let mut named_keys = NamedKeys::new();

    named_keys.insert(TOKEN.to_string(), storage::new_uref(token).into());
    named_keys.insert(FIXED_ARP.to_string(), storage::new_uref(fixed_apr).into());
    named_keys.insert(MIN_APR.to_string(), storage::new_uref(min_apr).into());
    named_keys.insert(MAX_APR.to_string(), storage::new_uref(max_apr).into());
    named_keys.insert(MAX_CAP.to_string(), storage::new_uref(max_cap).into());
    named_keys.insert(MIN_STAKE.to_string(), storage::new_uref(min_stake).into());
    named_keys.insert(MAX_STAKE.to_string(), storage::new_uref(max_stake).into());
    named_keys.insert(LOCK_PERIOD.to_string(), storage::new_uref(lock_period).into());
    named_keys.insert(DEPOSIT_START_TIME.to_string(), storage::new_uref(deposit_start_time).into());
    named_keys.insert(DEPOSIT_END_TIME.to_string(), storage::new_uref(deposit_end_time).into());

    //
    let init_entry_point: EntryPoint = EntryPoint::new(
        ENTRY_POINT_INIT,
        vec![],
        URef,
        EntryPointAccess::Public,
        EntryPointType::Contract
    );

    let stake_entry_point: EntryPoint = EntryPoint::new(
        ENTRY_POINT_STAKE,
        vec![Parameter::new(AMOUNT, CLType::U256)],
        URef,
        EntryPointAccess::Public,
        EntryPointType::Contract
    );

    let unstake_entry_point: EntryPoint = EntryPoint::new(
        ENTRY_POINT_UNSTAKE,
        vec![Parameter::new(AMOUNT, CLType::U256)],
        URef,
        EntryPointAccess::Public,
        EntryPointType::Contract
    );

    let claim_entry_point: EntryPoint = EntryPoint::new(
        ENTRY_POINT_CLAIM,
        vec![],
        URef,
        EntryPointAccess::Public,
        EntryPointType::Contract
    );

    let mut entry_points: EntryPoints = EntryPoints::new();

    entry_points.add_entry_point(init_entry_point);
    entry_points.add_entry_point(stake_entry_point);
    entry_points.add_entry_point(unstake_entry_point);
    entry_points.add_entry_point(claim_entry_point);

    let ph_text: String = String::from("stake_package_hash_");
    let ch_text: String = String::from("stake_contract_hash_");
    let au_text: String = String::from("stake_access_uref_");
    //
    let package_hash_text = ph_text;
    let contract_hash_text = ch_text;
    let uref_name_text = au_text;
    //
    let (contract_hash, _contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(package_hash_text),
        Some(uref_name_text)
    );

    runtime::put_key(&contract_hash_text, contract_hash.into());

    runtime::call_contract::<()>(
        storage_key,
        "insert",
        runtime_args! {
        "data" => contract_hash.to_string(),
    }
    );

    runtime::call_contract::<()>(contract_hash, ENTRY_POINT_INIT, runtime_args! {});
}

pub fn calculate_reward(
    now: u64,
    deposit_end_time: u64,
    deposit_start_time: u64,
    user_stake_amount: U256,
    user_last_claim_time: u64,
    locked_period: u64,
    fixed_apr: u64,
    min_apr: u64,
    max_apr: u64
) -> U256 {
    if now.lt(&deposit_end_time) {
        runtime::revert(Error::RewardCalculationPeriodError);
    }

    let elapsed_time = now.sub(user_last_claim_time);
    let dynamic_apr = calculate_dynamic_apr(
        now,
        deposit_start_time,
        locked_period,
        fixed_apr,
        min_apr,
        max_apr
    );

    // let a_year: u64 = 31557600000;

    user_stake_amount.mul(dynamic_apr).mul(elapsed_time).div(locked_period)
}

pub fn calculate_dynamic_apr(
    now: u64,
    deposit_start_time: u64,
    locked_period: u64,
    fixed_apr: u64,
    min_apr: u64,
    max_apr: u64
) -> u64 {
    if locked_period.eq(&0u64) {
        return fixed_apr;
    }

    let elapsed_time: u64 = now.sub(deposit_start_time);
    let total_apr_increase: u64 = max_apr.sub(min_apr);

    if elapsed_time.ge(&locked_period) {
        return min_apr.add(total_apr_increase);
    }

    let apr_increase_per_second = total_apr_increase.div(locked_period);

    return apr_increase_per_second.mul(elapsed_time).add(min_apr);
}
