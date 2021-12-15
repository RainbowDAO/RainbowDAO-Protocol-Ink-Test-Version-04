#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

pub use self::dao_vault::DaoVaultManager;

#[ink::contract]
mod dao_vault {
    use alloc::string::String;
    use alloc::vec::Vec;
    use erc20::Erc20;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout,SpreadLayout},
    };

    #[ink(storage)]
    pub struct DaoVaultManager {
        vault_manager:AccountId,
        amount_of_user:u64,
        allow_tokens:StorageHashMap<String,AccountId>,
        in_out_tokens:StorageHashMap<(AccountId,String), u64>,
    }
    #[ink(event)]
    pub struct AddTokenEvent{
        #[ink(topic)]
        token_address:AccountId,

    }
    #[ink(event)]
    pub struct RemoveTokenEvent{
        #[ink(topic)]
        token_address:Option<AccountId>,
    }
    #[ink(event)]
    pub struct GetTokensBalancesEvent{
        #[ink(topic)]
        token_address:AccountId,
        #[ink(topic)]
        balance:u64,
    }
    #[ink(event)]
    pub struct DepositTokenEvent{
        #[ink(topic)]
        token_address:AccountId,
        #[ink(topic)]
        depositer:AccountId,
        #[ink(topic)]
        amount:u64,

    }
    #[ink(event)]
    pub struct WithdrawTokenEvent{
        #[ink(topic)]
        token_address:AccountId,
        #[ink(topic)]
        withdrawer:AccountId,
        #[ink(topic)]
        amount:u64,

    }

    impl DaoVaultManager {
        #[ink(constructor)]
        pub fn new(owner:AccountId,) -> Self {
            Self {
                vault_manager :owner,
                amount_of_user:0,
                allow_tokens:StorageHashMap::default(),
                in_out_tokens:StorageHashMap::default(),
             }
        }
        #[ink(message)]
        pub fn deposit_token(& mut self, token_name:String,token_address1:AccountId,amount1:u64) -> bool {
            assert_eq!(self.allow_tokens.get(&token_name) == Some(&token_address1),true);
            let caller = self.env().caller();
            let vault_addr = self.env().account_id();
            let a =self.get_erc20_by_address(token_address1).allowance(caller, vault_addr);
            if a == 0 || a < amount1 {
                self.get_erc20_by_address(token_address1).approve(vault_addr, u64::max_value());
            }
            let mut erc20_instance:Erc20 = ink_env::call::FromAccountId::from_account_id(token_address1);
            erc20_instance.transfer_from(caller, vault_addr, amount1);
            self.amount_of_user+=amount1;
            self.in_out_tokens.insert((caller,token_name), self.amount_of_user);
            self.env().emit_event(DepositTokenEvent{
                token_address:token_address1,
                depositer:caller,
                amount:amount1,
            });
            true
        }
        #[ink(message)]
        pub fn withdrawer_token(&mut self, token_name:String, token_address1:AccountId, amount1:u64) -> bool{
            assert_eq!(self.allow_tokens.get(&token_name.clone()) == Some(&token_address1) , true);
            let caller = self.env().caller();
            let vault_addr = self.env().account_id();
            assert_eq!(self.in_out_tokens.get(&(caller,token_name.clone())) != Some(&0), true);
            if self.in_out_tokens.get(&(caller, token_name.clone())) < Some(&amount1) {
                return false
            }
            // let a = self.in_out_tokens.get(&(caller,token_name.clone())).unwrap();

           self.amount_of_user -= amount1;
            self.in_out_tokens.insert((caller,token_name.clone()), self.amount_of_user);
            let mut erc20_instance:Erc20 = ink_env::call::FromAccountId::from_account_id(token_address1);
            erc20_instance.transfer_from(vault_addr, caller, amount1);
            self.env().emit_event(WithdrawTokenEvent{
                token_address:token_address1,
                withdrawer:caller,
                amount:amount1,
            });
            true
        }
        #[ink(message)]
        pub fn check_deposit_token_balance(&self, dao_user:AccountId, token_name:String) ->u64{
            *self.in_out_tokens.get(&(dao_user, token_name)).unwrap_or(&0)
        }
        #[ink(message)]
        pub fn add_allow_token_addr(&mut self,erc20_address:AccountId) ->bool {
            let caller = self.env().caller();
            assert_eq!(caller == self.vault_manager,true);
            let symbol = self.get_erc20_by_address(erc20_address).symbol();
            self.allow_tokens.insert(symbol, erc20_address);
            self.env().emit_event(
                AddTokenEvent{
                    token_address:erc20_address,
                }
            );
            true
            }
            fn get_erc20_by_address(&self ,address:AccountId) ->Erc20{
                let erc20_instance:Erc20 = ink_env::call::FromAccountId::from_account_id(address);
                erc20_instance
            }
        #[ink(message)]
        pub fn remove_allow_token_addr(&mut self,token_name1:String) ->bool{
            let caller = self.env().caller();
            assert_eq!(caller == self.vault_manager,true);
            let a = self.check_token_addr(token_name1.clone());
            self.allow_tokens.take(&token_name1);
            self.env().emit_event(
                RemoveTokenEvent{
                token_address:a,
                }
            );
            true
        }
        fn check_token_addr(&self, token_name:String) ->Option<AccountId>{
           Some( *self.allow_tokens.get(&token_name).unwrap())
        }
        #[ink(message)]
        pub fn allow_token_list(&self) -> Vec<AccountId> {
         let mut new_list = Vec::new();
         let mut token_addr = self.allow_tokens.values();
         let mut token = token_addr.next();
         while token.is_some() {
         new_list.push(token.unwrap().clone());
         token = token_addr.next();
        }
        new_list
        }
   
        #[ink(message)]
        pub fn set_new_manager(&mut self, to:AccountId) ->bool{
            let caller = self.env().caller();
            assert_eq!(caller == self.vault_manager,true);
            self.vault_manager = to;
            true
        }
        #[ink(message)]
        pub fn check_manager(&self) ->AccountId{
            self.vault_manager
        }
        // #[ink(message)]
        // pub fn get_vault_balance(&self,token_addr:AccountId) ->u64{
        //     let caller = self.env().account_id();
        //    let a= self.get_erc20_by_address(token_addr).balance_of(caller);
        //     self.env().emit_event(
        //         GetTokensBalancesEvent{
        //             token_address:token_addr,
        //             balance:a,
        //         }
        //     );
        //     return a;
        // }
        }
    }
