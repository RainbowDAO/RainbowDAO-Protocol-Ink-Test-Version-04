#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::reward_system::{
    RewardSystem,
};

#[ink::contract]
mod reward_system {
    use alloc::string::String;
    use ink_storage::{collections::HashMap as StorageHashMap};

  
    #[ink(storage)]
    pub struct RewardSystem {
        turn_or_off: bool,
        owner:AccountId,
        dao_type: StorageHashMap<String, u64>,

    }

    impl RewardSystem {
        #[ink(constructor)]
        pub fn new(init: bool,owner:AccountId ) -> Self {
            Self {
                owner,
                turn_or_off: init,
                dao_type:StorageHashMap::new(),

            }
        }

        #[ink(message)]
        pub fn basic_reward(&mut self, income_proportion:u64, inconme_hierarchy:u64,income_per_floor_rate:u64) ->bool {
            self.turn_or_off = !self.turn_or_off;
        }
        #[ink(message)]
        pub fn change_owner(&mut self,to:AccountId) ->bool{
            assert!(self.env().caller() == self.owner);
            self.owner = to;
            true
        }
        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.turn_or_off
        }
    }
}
