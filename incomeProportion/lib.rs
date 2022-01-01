#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incomeProportion {

    #[ink(storage)]
    pub struct IncomeProportion {
        manager:AccountId,
    }

    impl IncomeProportion {
        #[ink(constructor)]
        pub fn new(manager:AccountId) -> Self {
            Self { 
                manager, 
                
            }
        }

        #[ink(message)]
        pub fn change_new_manager(&mut self,to:AccountId)->bool {
            assert!(self.manager == self.env().caller());
            self.manager = to;
            true
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
