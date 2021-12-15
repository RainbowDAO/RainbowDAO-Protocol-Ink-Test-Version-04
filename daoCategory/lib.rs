#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use dao_category::DaoCategory;


#[ink::contract]
mod dao_category {
    use alloc::string::String;
    use ink_lang::EmitEvent;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout,SpreadLayout},
    };

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct DaoCategory {
        dao_address:AccountId,
        manager_address:AccountId,
        category:u64,
        dao_category:StorageHashMap<AccountId,u64>,
        dao_relationship:StorageHashMap<(AccountId, AccountId), u64>,//1 独立dao 2子母dao 3dao联盟
    }

    // #[derive(Debug, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    // #[cfg_attr(
    // feature = "std",
    // derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    // )]
    // pub struct DaoRelation{
    //     dao1:AccountId,
    //     dao2:AccountId,
        
    // }
    
    #[ink(event)]
    pub struct DaoRelationships{
        #[ink(topic)]
        caller:AccountId,
        #[ink(topic)]
        dao_address:AccountId,
        #[ink(topic)]
        category:u64,
    }
    impl DaoCategory {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(owner:AccountId) -> Self {
            Self { 
                manager_address:owner,
                dao_address:Default::default(),
                category:0,
                dao_category:StorageHashMap::new(),
                dao_relationship:StorageHashMap::new(),

            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        fn _set_category(&mut self, dao_address:AccountId,dao_address1:AccountId ,category:u64) ->bool {
            if category == 1 {
                self.dao_category.insert(dao_address, category);
                return true;
         }
            if category == 2 {
                self.dao_category.insert(dao_address, category);
                return true;
        }
            if category == 3 {
                self.dao_category.insert(dao_address, category);
                return true;
            }
            true
        }
        #[ink(message)]
        pub fn get_category(&self , dao_address:AccountId) -> u64 {
            *self.dao_category.get(&dao_address).unwrap_or(&0)
        }
        // #[ink(message)]
        // pub fn role_list(&self) -> Vec<DaoRelation> {
        //     let mut new_list = Vec::new();
        //     let mut iter = self.dao_category.keys();
        //     let mut rela=self.dao_category.values();

        //     let mut daos = iter.next();
        //     let mut relas=rela.next();
        //     while daos.is_some() {
        //     new_list.push(daos.unwrap().clone());
            
        //     daos = iter.next();
        //     }
        //     new_list
        // }
        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get_dao_relationship(&self, dao_address:AccountId, dao_address1:AccountId) -> u64 {
            *self.dao_relationship.get(&(dao_address,dao_address1)).unwrap_or(&0)
        }
        
        // #[ink(message)]
        // pub fn change_status(&mut self, category:u64)->bool{
        //     let caller = self.env().caller();
            
        // }
    }
}
