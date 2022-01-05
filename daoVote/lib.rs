#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

pub use self::dao_vote::DaoVote;


#[ink::contract]
mod dao_vote {
    use erc20::Erc20;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout,SpreadLayout},
    };

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct DaoVote {
        vote_manager:AccountId,
        erc20_address:AccountId,
        pub contract_instance:ContractInstance,
        vote2:StorageHashMap<AccountId, u64>,
        vote1:StorageHashMap<AccountId, u64>,
        // vote_category:StorageHashMap<>, 
        join_time:StorageHashMap<AccountId,u64>,
    }
    #[derive(Debug, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct ContractInstance{
        erc20:Option<Erc20>,
    }

    impl DaoVote {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(owner:AccountId , erc20_addr:AccountId ) -> Self {
            Self { 
                vote_manager: owner,
                erc20_address:erc20_addr,
                vote1:StorageHashMap::new(),
                vote2:StorageHashMap::new(),
                join_time:StorageHashMap::new(),
                contract_instance:ContractInstance{
                    erc20:None,
                }

             }
        }

        ///Instantiate erc20
        #[ink(message)]
        pub fn erc20_ins(&mut self) ->bool{
            let contract_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(self.erc20_address);
            self.contract_instance.erc20 = Some(contract_instance);
            true
        }
        #[ink(message)]
        pub fn balance_of(&self, owner_addr:AccountId) -> u64{
           let mut instance:Erc20 = self.contract_instance.erc20.as_ref().unwrap().clone();
           instance.balance_of(owner_addr)
        }
        // #[ink(message)]
        // pub fn dao_user_ins(&mut self) ->bool{
        //     let contract_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(self.dao_user_addr);
        //     self.contract_instance.erc20 = contract_instance;
        //     true
        // }
        #[ink(message)]
        pub fn set_user_join_time(&mut self,user_addr:AccountId , join_time:Timestamp) ->bool{
            // let user_addr = self.env().caller();
            // let user_start_j_time = self.env().block_timestamp();
            self.join_time.insert(user_addr, join_time);
            true
        }
        ///Get joining time
        #[ink(message)]
        pub fn get_join_time(&self, user:AccountId) ->u64{
            *self.join_time.get(&user).unwrap()
        }

        // #[ink(message)]
        // pub fn set_vote_category(&mut self, vote_category:u64) ->bool{
        //     let caller = self.env().caller();
        //     assert_eq!(caller == self.vote_manager,true);
        //     if vote_category == 1{
        //         votes1(); 
        //     }
        // }

        ///One person one vote
        #[ink(message)]
        pub fn votes1(&mut self,user_addr:AccountId) ->bool{
            let join_start_time = self.join_time.get(&user_addr).unwrap();
            let join = self.env().block_timestamp();
            let mut instance:Erc20 = self.contract_instance.erc20.as_ref().unwrap().clone();
            let a = instance.balance_of(user_addr);
            assert_eq!(a >= 0 && join - join_start_time >= 604800,true);
            if join - join_start_time >= 604800 {
            self.vote1.insert(user_addr, 1);
            return true;
            }else{
            return false;
            }
        }


        ///One coin, one vote
        #[ink(message)]
        pub fn votes2(&mut self) ->bool{
            let caller = self.env().caller();
            let mut instance:Erc20 = self.contract_instance.erc20.as_ref().unwrap().clone();
            let mut a = instance.balance_of(caller);
            assert_eq!(a >= 0 ,true);
            self.vote2.insert(caller , a);
            true
        }

        
        #[ink(message)]
        pub fn get_self_votes_one(&self,user:AccountId) ->u64{
            *self.vote1.get(&user).unwrap()
        }
        #[ink(message)]
        pub fn get_self_vote_two(&self,user:AccountId) ->u64{
            *self.vote2.get(&user).unwrap()
        }
        #[ink(message)]
        pub fn take_self_vote(&mut self,user:AccountId) ->bool{
            self.vote1.take(&user);
            true
        }
        #[ink(message)]
        pub fn transfer_manager(&mut self, to:AccountId) -> bool {
            assert_eq!(self.env().caller() == self.vote_manager,true);
            self.vote_manager =to;
            true
        }
    }
}
