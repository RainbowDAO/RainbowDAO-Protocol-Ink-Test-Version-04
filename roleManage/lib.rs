#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
pub use self::role_manage::{
    RoleManage,
};

use ink_lang as ink;

#[ink::contract]
mod role_manage {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_storage::{collections::HashMap as StorageHashMap, };

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct RoleManage {
        owner: AccountId,
        index: i32,
        role_map: StorageHashMap<i32,String>,
        user_role: StorageHashMap<AccountId,Vec<String>>,
        role_privileges: StorageHashMap<i32,Vec<String>>,
    }
    impl RoleManage {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self {
                owner,
                index: 0,
                role_map : StorageHashMap::new(),
                user_role: StorageHashMap::new(),
                role_privileges: StorageHashMap::new(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }
        ///add a new role
        #[ink(message)]
        pub fn add_role(&mut self, name: String) -> bool {
            self.ensure_from_owner();
            self.role_map.insert(self.index, name);
            self.index += 1;
            true
        }
        
        #[ink(message)]
        pub fn list_roles(&self) -> Vec<String> {
            let mut role_vec = Vec::new();
            let mut iter = self.role_map.values();
            let mut role = iter.next();
            while role.is_some() {
                role_vec.push(role.unwrap().clone());
                role = iter.next();
            }
            role_vec
        }
        #[ink(message)]
        pub fn query_role_by_index(&self, index: i32) -> String {
            self.role_map.get(&index).unwrap().clone()
        }
        #[ink(message)]
        pub fn role_insert_privilege(&mut self ,index:i32,privilege:String) -> bool {
            let role_privilege_list = self.role_privileges.entry(index.clone()).or_insert(Vec::new());
            role_privilege_list.push(privilege);
            true
        }
        #[ink(message)]
        pub fn list_role_privileges(&self,index:i32) -> Vec<String> {
           let v =  self.role_privileges.get(&index).unwrap().clone();
            v
        }
        #[ink(message)]
        pub fn add_user_role(&mut self,user:AccountId,role:String) -> bool {
            let user_role_list = self.user_role.entry(user.clone()).or_insert(Vec::new());
            user_role_list.push(role);
            true
        }
        #[ink(message)]
        pub fn check_user_role(&self,user:AccountId,role:String) -> bool {
            let list =  self.user_role.get(&user).unwrap().clone();
            for i in  list{
                if i == role {
                    return true
                }
            }
            false
        }
        #[ink(message)]
        pub fn get_user_roles(&self,user:AccountId) -> Vec<String> {
           let list =  self.user_role.get(&user).unwrap().clone();
            list
        }

        /// Panic the sender is owner.
        fn ensure_from_owner(&self) {
            assert_eq!(self.env().caller(), self.owner);
        }
    }
}