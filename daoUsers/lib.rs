#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
pub use self::dao_user::DaoUsers;

#[ink::contract]
mod dao_user {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default)]
    #[cfg_attr(feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct Usersinfo {
        user_id:Option<u64>,
        nick_name:String,
        user_address:Option<AccountId>,
    }
    #[ink(storage)]
    pub struct DaoUsers {
        ///init users register num
        user_id: u64,
        addr_ref:AccountId,
        register_time:u64,
        nick_name:String,
        user_address:AccountId,
        user_maps:StorageHashMap<AccountId, u64>,
        referee_maps:StorageHashMap<(AccountId, AccountId),u64>,
        join_time:StorageHashMap<AccountId, u64>,
        user_info:Usersinfo,
        is_sill:bool,
    }
    #[ink(event)]
    pub struct Referee{
        #[ink(topic)]
        referee_id:u64,
        #[ink(topic)]
        refereer:AccountId,
    }

    impl DaoUsers {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(manager_address:AccountId) -> Self {
            Self { 
                user_id: 20090103,
                addr_ref:Default::default(),
                register_time:0,
                nick_name:String::from(""),
                user_address:manager_address,
                user_maps:StorageHashMap::new(),
                referee_maps:StorageHashMap::new(),
                join_time:StorageHashMap::new(),
                user_info:Usersinfo{
                    user_id:None,
                    nick_name:String::from(""),
                    user_address:None,
                },
                is_sill:false,
             }
        }

   

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn add_user(&mut self, user_address:AccountId , nick_name:String) -> bool {
            let caller = self.env().caller();
            let j_time = self.env().block_timestamp();
            if caller == self.user_address || self.is_sill == false{
                self.user_maps.insert(user_address.clone(), self.user_id);
                self.join_time.insert(user_address, j_time);
                self.user_id+=1;
                self.user_info= Usersinfo{
                    user_id:Some(self.user_id),
                    nick_name:nick_name.clone(),
                    user_address:Some(user_address),
                };
                return true;
            }
            // self.sill()
            true
            
        }
        #[ink(message)]
        pub fn remove_user(&mut self, user_address:AccountId,) -> bool {
            let caller = self.env().caller();
            // assert_eq!(caller == self.user_address,true);
            if caller == self.user_address || caller == user_address{
            self.user_maps.take(&user_address);
            }
            true
        }
        #[ink(message)]
        pub fn role_list(&self) -> Vec<AccountId> {
            let mut new_list = Vec::new();
            let mut iter = self.user_maps.keys();
            let mut users = iter.next();
            while users.is_some() {
            new_list.push(users.unwrap().clone());
            users = iter.next();
            }
            new_list
        }

        #[ink(message)]
        pub fn sill(&mut self) -> bool {
            let caller = self.env().caller();
            assert_eq!(caller == self.user_address,true);
            self.is_sill = !self.is_sill;
            true
        }
        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn transfer_manager(&mut self, to:AccountId) -> bool {
            let caller = self.env().caller();
            assert_eq!(caller == self.user_address,true);
            self.user_address = to;
            true
        }
        #[ink(message)]
        pub fn insert_join_time(&mut self,user:AccountId,j_time:u64 ) ->bool {
            self.join_time.insert(user, j_time);
            true
        }
        #[ink(message)]
        pub fn get_join_time(&self, user_addr:AccountId) ->u64{
            self.join_time.get(&user_addr).unwrap().clone()
        }
    }
}
