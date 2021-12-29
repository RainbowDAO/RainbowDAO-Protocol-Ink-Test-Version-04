#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::erc20_factory::{
    Erc20Factory,
};

#[ink::contract]
mod erc20_factory {
    use alloc::string::String;
    use erc20::Erc20;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
    };
    const RENT_VALUE: u128 = 1000 * 1_000_000_000_000;

    #[ink(storage)]
    pub struct Erc20Factory {
        owner: AccountId,
        index: u64,
        token: StorageHashMap<u64,AccountId>,
        symbol_token: StorageHashMap<String, AccountId>,
        test_ins: Erc20,
    }

    impl Erc20Factory {
        // #[ink(constructor)]
        // pub fn new(owner: AccountId) -> Self {
        //     Self {
        //         owner,
        //         index: 0,
        //         token: StorageHashMap::new(),
        //         symbol_token: StorageHashMap::new(),
        //     }
        // }

        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Default::default(),
                index: 0,
                token: StorageHashMap::new(),
                symbol_token: StorageHashMap::new(),
                test_ins: Default::default(),
            }
        }

        #[ink(message)]
        pub fn test_ins_fn(&self) -> String {
            self.test_ins.name()
        }

        #[ink(message)]
        pub fn mint_token(&mut self, erc20_hash: Hash,name:String ,symbol:String ,initial_supply:u64, decimals:u8, controller: AccountId) -> bool {
            let total_balance = Self::env().balance();
            let num: i32 = 1;
            let salt = num.to_le_bytes();
            let instance_params = Erc20::new(name,symbol.clone(),initial_supply,decimals, controller)
                .endowment(RENT_VALUE)
                .code_hash(erc20_hash)
                .salt_bytes(salt)
                .params();
            let test_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = test_result.expect("failed at instantiating the `ERC20` contract");
            self.token.insert(self.index,contract_addr);
            self.symbol_token.insert(symbol, contract_addr);
            self.index += 1;
            true
        }

        #[ink(message)]
        pub fn get_token_by_index(&self,index: u64) -> AccountId {
            self.token.get(&index).unwrap().clone()
        }
        #[ink(message)]
        pub fn get_token_by_symbol(&self, name:String) -> AccountId {
            self.symbol_token.get(&name).unwrap().clone()
        }   
        

        #[ink(message)]
        pub fn get_Block(&self) -> Timestamp {
            self.env().block_timestamp()
        }
        }
    }