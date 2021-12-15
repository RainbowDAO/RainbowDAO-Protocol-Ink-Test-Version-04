#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
pub use self::privilege_manage::{
    PrivilegeManage,
};
#[ink::contract]
mod privilege_manage {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_storage::{collections::HashMap as StorageHashMap,};
    
    #[ink(storage)]
    pub struct PrivilegeManage {
        owner:AccountId,
        index:u64,
        privilege_map:StorageHashMap<u64,String>,
    }
    impl PrivilegeManage {

        ///core accountId
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self {
                owner,
                index: 0,
                privilege_map : StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn add_privilege(&mut self, name: String) -> bool {
            self.ensure_core();
            self.privilege_map.insert(self.index, name);
            self.index += 1;
            true
        }
        #[ink(message)]
        pub fn get_privileges_list(&self) -> Vec<String> {
            let mut list = Vec::new();
            let mut iter = self.privilege_map.values();
            let mut privilege = iter.next();
            while privilege.is_some() {
                list.push(privilege.unwrap().clone());
                privilege = iter.next();
            }
            list
        }
        #[ink(message)]
        pub fn get_privilege_by_index(&self, index: u64) -> String {
            self.privilege_map.get(&index).unwrap().clone()
        }

        fn ensure_core(&self) {
            assert_eq!(self.env().caller(),self.owner);
        }
    }
}