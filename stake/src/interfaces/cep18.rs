#![allow(dead_code)]
extern crate alloc;

use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, ContractHash, Key, RuntimeArgs, U256};

pub struct CEP18 {
    pub contract_hash: ContractHash,
}

impl CEP18 {
    pub fn new(contract_hash: ContractHash) -> Self {
        CEP18 { contract_hash }
    }

    pub fn transfer(&self, recipient: Key, amount: U256) -> () {
        runtime::call_contract::<()>(
            self.contract_hash,
            "transfer",
            runtime_args! {
                "recipient" => recipient,
                "amount" => amount,
            },
        )
    }

    pub fn transfer_from(&self, sender: Key, recipient: Key, amount: U256) -> () {
        runtime::call_contract::<()>(
            self.contract_hash,
            "transfer_from",
            runtime_args! {
              "owner" => sender,
              "recipient" => recipient,
              "amount" => amount,
          },
        )
    }

    pub fn approve(&self, spender: Key, amount: U256) -> () {
        runtime::call_contract::<()>(
            self.contract_hash,
            "approve",
            runtime_args! {
                "spender" => spender,
                "amount" => amount,
            },
        )
    }

    pub fn decimals(&self) -> u8 {
        runtime::call_contract::<u8>(
            self.contract_hash,
            "decimals",
            runtime_args! {},
        )
    }
}
