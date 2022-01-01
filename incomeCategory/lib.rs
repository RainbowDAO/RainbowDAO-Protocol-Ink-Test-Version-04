#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod incomeCategory {
    use alloc::string::String;
    use ink_storage::{collections::HashMap as StorageHashMap};
    #[ink(storage)]
    pub struct IncomeCategory {
        manager:AccountId,
        category_map:StorageHashMap<(AccountId,u64), u64>,
    }

    impl IncomeCategory {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(manager:AccountId) -> Self {
            Self { 
                manager,
                category_map:StorageHashMap::new() 
            }
        }

        #[ink(message)]
        pub fn set_income_category(&mut self,user_addr:AccountId, income_category:u64, amount:u64 ) ->bool {
            assert!(self.manager == self.env().caller());
            self.category_map.insert((user_addr, income_category),amount);
            true
        }

        #[ink(message)]
        pub fn get_user_amount(&self, user_addr:AccountId, income_category:u64) -> u64 {
            *self.category_map.get(&(user_addr,income_category)).unwrap()
        }
        #[ink(message)]
        pub fn check_manager(&self) ->AccountId{
            self.manager
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let incomeCategory = IncomeCategory::default();
            assert_eq!(incomeCategory.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut incomeCategory = IncomeCategory::new(false);
            assert_eq!(incomeCategory.get(), false);
            incomeCategory.flip();
            assert_eq!(incomeCategory.get(), true);
        }
    }
}
