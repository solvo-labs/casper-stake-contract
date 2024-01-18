#![allow(dead_code)]

use crate::enums::Address;
use crate::error::Error;
use casper_contract::{
    contract_api::runtime,
    contract_api::storage,
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::FromBytes,
    system::CallStackElement,
    CLTyped,
    URef,
    Key,
    ContractHash,
};
use core::convert::TryInto;
use alloc::string::{ String, ToString };

pub fn get_key<T: FromBytes + CLTyped>(name: &str) -> T {
    let key = runtime
        ::get_key(name)
        .unwrap_or_revert_with(Error::FatalError)
        .try_into()
        .unwrap_or_revert_with(Error::FatalError);
    storage
        ::read(key)
        .unwrap_or_revert_with(Error::FatalError)
        .unwrap_or_revert_with(Error::FatalError)
}

pub(crate) fn read_from<T>(name: &str) -> T where T: FromBytes + CLTyped {
    let uref = get_uref(name);
    let value: T = storage::read(uref).unwrap_or_revert().unwrap_or_revert();
    value
}

/// Gets [`URef`] under a name.
pub(crate) fn get_uref(name: &str) -> URef {
    let key = runtime::get_key(name).ok_or(Error::FatalError).unwrap_or_revert();
    key.try_into().unwrap_or_revert()
}

// ref => https://github.com/BitHotelOrg/bithotel-casper-contracts/blob/8d53dc0047bb0e28ca02c7ff688d0090ece8df77/marketplace/contract/src/utils.rs#L22
pub fn get_current_address() -> Address {
    let call_stack_element = runtime::get_call_stack().into_iter().rev().next().unwrap_or_revert();
    match call_stack_element {
        CallStackElement::Session { account_hash } => Address::from(account_hash),
        CallStackElement::StoredSession { account_hash, .. } => Address::from(account_hash),
        CallStackElement::StoredContract { contract_hash: _, contract_package_hash } =>
            Address::from(contract_package_hash),
    }
}

// ref => https://github.com/casper-ecosystem/cep-78-enhanced-nft/blob/dev/contract/src/utils.rs#L83
pub fn encode_dictionary_item_key(key: Key) -> String {
    match key {
        Key::Account(account_hash) => account_hash.to_string(),
        Key::Hash(hash_addr) => ContractHash::new(hash_addr).to_string(),
        _ => runtime::revert(Error::InvalidKey),
    }
}
