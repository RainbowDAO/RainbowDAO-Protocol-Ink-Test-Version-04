#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::income_proportion::IncomeProportion;

#[ink::contract]
mod income_proportion {
    use alloc::string::String;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };

    #[ink(storage)]
    pub struct IncomeProportion {
        manager:AccountId,
        income_proportion:StorageHashMap<String,u64>,
    }

    impl IncomeProportion {
        #[ink(constructor)]
        pub fn new(manager:AccountId) -> Self {
            Self { 
                manager,
                income_proportion:StorageHashMap::new(),
                
            }
        }

        #[ink(message)]
        pub fn change_new_manager(&mut self,to:AccountId)->bool {
            assert!(self.manager == self.env().caller());
            self.manager = to;
            true
        }

        #[ink(message)]
        pub fn income_proportion(&mut self,proportion:String, amount:u64) -> bool{
            let caller = self.env().caller();
            assert_eq!(caller == self.manager, true);
            self.income_proportion.insert(proportion, amount);
            true

        }
        #[ink(message)]
        pub fn check_income_proportion(&self,proportion:String) ->u64{
            *self.income_proportion.get(&proportion).unwrap()
        }
        #[ink(message)]
        pub fn get_manager_addr(&self) ->AccountId {
            self.manager
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let incomeProportion = IncomeProportion::default();
            assert_eq!(incomeProportion.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut incomeProportion = IncomeProportion::new(false);
            assert_eq!(incomeProportion.get(), false);
            incomeProportion.flip();
            assert_eq!(incomeProportion.get(), true);
        }
    }
}
