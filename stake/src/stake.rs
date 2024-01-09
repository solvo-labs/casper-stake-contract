use alloc::{ string::{ String, ToString }, vec };
use crate::{ error::Error, utils::{ get_key, self } };
use casper_contract::contract_api::{ runtime, storage, system };
use casper_types::{
    EntryPoint,
    ContractHash,
    Key,
    EntryPointAccess,
    CLType::{ U64, URef, self },
    EntryPointType,
    EntryPoints,
    contracts::NamedKeys,
    RuntimeArgs,
    runtime_args,
    account::AccountHash,
    Parameter,
    U256,
};
use crate::interfaces::cep18::CEP18;

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
const REWARD: &str = "reward";
// The stake wallet to which all stakes will be transferred.
const TOTAL_SUPPLY: &str = "total_supply";
// The token type to be staked
const STAKED_TOKEN: &str = "staked_token";
// The amount of tokens to be stake, withdraw and earned
const AMOUNT: &str = "amount";

const REWARD_RATE: &str = "reward_rate";
const REWARD_PER_TOKEN_STORED: &str = "reward_per_token_stored";
const PURSE: &str = "purse";

const DECIMALS: &str = "decimals";

// Entry Points
const ENTRY_POINT_INIT: &str = "init";
const ENTRY_POINT_NOTIFY_REWARD_AMOUNT: &str = "notify_reward_amount";
const ENTRY_POINT_SET_REWARD_DURATION: &str = "set_rewards_duration";
const ENTRY_POINT_STAKE: &str = "stake";

// #[no_mangle]
// pub extern "C" fn stake() {
//     let amount: U256 = runtime::get_named_arg(AMOUNT);

//     if amount.as_u64().le(&0u64) {
//         runtime::revert(Error::StakeAmountError);
//     }

//     let owner: Key = utils::read_from(OWNER);
//     let staker: Key = runtime::get_caller().into();

//     if staker == owner {
//         runtime::revert(Error::CannotTargetSelfUser);
//     }

//     let mut total_supply: u64 = utils::read_from(TOTAL_SUPPLY);

//     let staked_token: Key = runtime::get_named_arg(STAKED_TOKEN);
//     let purse: Key = get_key(PURSE);

//     let cep18: CEP18 = CEP18::new(staked_token.into_hash().map(ContractHash::new).unwrap());
//     cep18.transfer_from(staker, purse, amount);

//     total_supply = total_supply + amount.as_u64();
//     runtime::put_key(TOTAL_SUPPLY, storage::new_uref(total_supply).into());
// }

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

// #[no_mangle]
// pub extern "C" fn notify_reward_amount() {
//     only_owner();
//     update_reward();

//     let reward: U256 = runtime::get_named_arg(REWARD);

//     let duration: u64 = utils::read_from(DURATION);
//     let mut finish_at: u64 = utils::read_from(FINISH_AT);
//     let mut reward_rate: u64 = utils::read_from(REWARD_RATE);

//     let now: u64 = runtime::get_blocktime().into();

//     if now.ge(&finish_at) {
//         reward_rate = reward.as_u64() / duration;
//     } else {
//         let remaining_rewards: u64 = (finish_at - now) * reward_rate;
//         reward_rate = (reward.as_u64() + remaining_rewards) / duration;
//     }
//     if reward_rate.lt(&0u64) {
//         runtime::revert(Error::RewardRateError);
//     }

//     // TODO
//     // require rewardRate * duration <= rewardsToken.balanceOf(address(this))

//     finish_at = &now + duration;
//     let update_at: u64 = now;
//     runtime::put_key(REWARD_RATE, storage::new_uref(reward_rate).into());
//     runtime::put_key(UPDATE_AT, storage::new_uref(update_at).into());
//     runtime::put_key(FINISH_AT, storage::new_uref(finish_at).into());
//     runtime::put_key(REWARD, storage::new_uref(reward).into());
// }

#[no_mangle]
pub extern "C" fn init() {
    runtime::put_key(UPDATE_AT, storage::new_uref(0u64).into());
    runtime::put_key(REWARD, storage::new_uref(U256::zero()).into());
    runtime::put_key(REWARD_PER_TOKEN_STORED, storage::new_uref(0u64).into());
    runtime::put_key(PURSE, system::create_purse().into());
    runtime::put_key(TOTAL_SUPPLY, storage::new_uref(0u64).into());
}

// constructor
#[no_mangle]
pub extern "C" fn call() {
    let staked_token: Key = runtime::get_named_arg(STAKED_TOKEN);
    let finish_at: u64 = runtime::get_named_arg(FINISH_AT);
    let duration: u64 = runtime::get_named_arg(DURATION);
    let reward_rate: u8 = runtime::get_named_arg(REWARD_RATE);

    let cep18: CEP18 = CEP18::new(staked_token.into_hash().map(ContractHash::new).unwrap());
    let decimals: u8 = cep18.decimals();

    let owner: AccountHash = runtime::get_caller();

    let mut named_keys = NamedKeys::new();

    named_keys.insert(OWNER.to_string(), storage::new_uref(owner).into());
    named_keys.insert(STAKED_TOKEN.to_string(), storage::new_uref(staked_token).into());
    named_keys.insert(DECIMALS.to_string(), storage::new_uref(decimals).into());
    named_keys.insert(FINISH_AT.to_string(), storage::new_uref(finish_at).into());
    named_keys.insert(DURATION.to_string(), storage::new_uref(duration).into());
    named_keys.insert(REWARD_RATE.to_string(), storage::new_uref(reward_rate).into());

    let init_entry_point: EntryPoint = EntryPoint::new(
        ENTRY_POINT_INIT,
        vec![],
        URef,
        EntryPointAccess::Public,
        EntryPointType::Contract
    );

    // let notify_reward_amount_entry_point: EntryPoint = EntryPoint::new(
    //     ENTRY_POINT_NOTIFY_REWARD_AMOUNT,
    //     vec![Parameter::new(REWARD, CLType::U256)],
    //     URef,
    //     EntryPointAccess::Public,
    //     EntryPointType::Contract
    // );

    // let set_rewards_duration_entry_point: EntryPoint = EntryPoint::new(
    //     ENTRY_POINT_SET_REWARD_DURATION,
    //     vec![Parameter::new(DURATION, U64)],
    //     URef,
    //     EntryPointAccess::Public,
    //     EntryPointType::Contract
    // );

    // let stake_entry_point: EntryPoint = EntryPoint::new(
    //     ENTRY_POINT_STAKE,
    //     vec![Parameter::new(AMOUNT, CLType::U256)],
    //     URef,
    //     EntryPointAccess::Public,
    //     EntryPointType::Contract
    // );

    let mut entry_points: EntryPoints = EntryPoints::new();

    entry_points.add_entry_point(init_entry_point);
    // entry_points.add_entry_point(notify_reward_amount_entry_point);
    // entry_points.add_entry_point(set_rewards_duration_entry_point);
    // entry_points.add_entry_point(stake_entry_point);

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
        Some(uref_name_text)
    );

    runtime::put_key(&contract_hash_text, contract_hash.into());

    runtime::call_contract::<()>(contract_hash, ENTRY_POINT_INIT, runtime_args! {});
}

// internal functions
// pub fn only_owner() {
//     let admin: AccountHash = get_key(OWNER);
//     let caller: AccountHash = runtime::get_caller();
//     if admin != caller {
//         runtime::revert(Error::AdminError)
//     }
// }

// pub fn update_reward() {
//     let reward_per_token_stored: u64 = reward_per_token();
//     let update_at: u64 = last_time_reward_applicable();

//     runtime::put_key(REWARD_PER_TOKEN_STORED, storage::new_uref(reward_per_token_stored).into());
//     runtime::put_key(UPDATE_AT, storage::new_uref(update_at).into());
// }

// pub fn reward_per_token() -> u64 {
//     let total_supply: u64 = utils::read_from(TOTAL_SUPPLY);
//     let reward_per_token_stored: u64 = utils::read_from(REWARD_PER_TOKEN_STORED);

//     let reward_rate: u64 = utils::read_from(REWARD_RATE);
//     let update_at: u64 = utils::read_from(UPDATE_AT);

//     let token_decimal: u8 = utils::read_from(DECIMALS);
//     let decimal: f64 = u64::pow(10, token_decimal as u32) as f64;

//     if total_supply.eq(&0u64) {
//         return reward_per_token_stored;
//     }
//     reward_per_token_stored +
//         (reward_rate * (last_time_reward_applicable() - update_at) * (decimal as u64)) /
//             total_supply
// }

// pub fn last_time_reward_applicable() -> u64 {
//     let finish_at: u64 = utils::read_from(FINISH_AT);
//     let now: u64 = runtime::get_blocktime().into();
//     min(finish_at, now)
// }

// pub fn min(first_timestamp: u64, second_timestamp: u64) -> u64 {
//     if first_timestamp.le(&second_timestamp) { first_timestamp } else { second_timestamp }
// }
