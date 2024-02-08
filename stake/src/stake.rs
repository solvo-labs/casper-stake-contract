use core::char::MAX;
use core::ops::{ Add, Div, Mul, Sub };
use crate::enums::Address;
use crate::interfaces::cep18::{ self, CEP18 };
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
const NOTIFIED: &str = "notified";
const OWNER: &str = "owner";
const TOTAL_REWARD: &str = "total_reward";

// Dictionaries
const STAKES_DICT: &str = "stakes_dict";
const CLAIMED_DICT: &str = "claimed_dict";

// Entry points
const ENTRY_POINT_NOTIFY: &str = "notify";
const ENTRY_POINT_STAKE: &str = "stake";
const ENTRY_POINT_UNSTAKE: &str = "unstake";
const ENTRY_POINT_CLAIM: &str = "claim";

#[no_mangle]
pub extern "C" fn stake() {}

#[no_mangle]
pub extern "C" fn unstake() {}

#[no_mangle]
pub extern "C" fn claim() {}

#[no_mangle]
pub extern "C" fn notify() {
    only_owner();

    let notified: bool = utils::read_from(NOTIFIED);

    if notified {
        runtime::revert(Error::AlreadyNotified);
    }

    let fixed_apr: u64 = utils::read_from(FIXED_ARP);
    let max_apr: u64 = utils::read_from(MAX_APR);
    let max_cap: U256 = utils::read_from(MAX_CAP);
    let token: Key = utils::read_from(TOKEN);

    let mut prize = U256::zero();

    if fixed_apr > 0 {
        let fixed_apr_u256 = U256::from(fixed_apr);

        prize = max_cap.mul(fixed_apr_u256).div(U256::from(100));
    } else {
        let max_apr_u256 = U256::from(max_apr);
        prize = max_cap.mul(max_apr_u256).div(U256::from(100));
    }

    let owner: AccountHash = runtime::get_caller();
    let contract_address: Address = get_current_address();

    // check allowance

    let cep18: CEP18 = CEP18::new(token.into_hash().map(ContractHash::new).unwrap());
    let balance: U256 = cep18.balance_of(owner.into());

    if prize.gt(&balance) {
        runtime::revert(Error::UnsufficientBalance);
    }

    cep18.transfer_from(owner.into(), contract_address.into(), prize);

    storage::new_dictionary(STAKES_DICT).unwrap_or_default();
    storage::new_dictionary(CLAIMED_DICT).unwrap_or_default();
    runtime::put_key(TOTAL_SUPPLY, storage::new_uref(U256::zero()).into());
    runtime::put_key(TOTAL_REWARD, storage::new_uref(prize).into());
    runtime::put_key(NOTIFIED, storage::new_uref(true).into());
}

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
    let owner: AccountHash = runtime::get_caller();

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
    named_keys.insert(NOTIFIED.to_string(), storage::new_uref(false).into());
    named_keys.insert(OWNER.to_string(), storage::new_uref(owner).into());

    let notify_entry_point: EntryPoint = EntryPoint::new(
        ENTRY_POINT_NOTIFY,
        vec![],
        URef,
        EntryPointAccess::Public,
        EntryPointType::Contract
    );

    // let stake_entry_point: EntryPoint = EntryPoint::new(
    //     ENTRY_POINT_STAKE,
    //     vec![Parameter::new(AMOUNT, CLType::U256)],
    //     URef,
    //     EntryPointAccess::Public,
    //     EntryPointType::Contract
    // );

    // let unstake_entry_point: EntryPoint = EntryPoint::new(
    //     ENTRY_POINT_UNSTAKE,
    //     vec![Parameter::new(AMOUNT, CLType::U256)],
    //     URef,
    //     EntryPointAccess::Public,
    //     EntryPointType::Contract
    // );

    // let claim_entry_point: EntryPoint = EntryPoint::new(
    //     ENTRY_POINT_CLAIM,
    //     vec![],
    //     URef,
    //     EntryPointAccess::Public,
    //     EntryPointType::Contract
    // );

    let mut entry_points: EntryPoints = EntryPoints::new();

    entry_points.add_entry_point(notify_entry_point);
    // entry_points.add_entry_point(stake_entry_point);
    // entry_points.add_entry_point(unstake_entry_point);
    // entry_points.add_entry_point(claim_entry_point);

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
}

pub fn only_owner() {
    let admin: AccountHash = utils::get_key(OWNER);
    let caller: AccountHash = runtime::get_caller();
    if admin != caller {
        runtime::revert(Error::AdminError)
    }
}
