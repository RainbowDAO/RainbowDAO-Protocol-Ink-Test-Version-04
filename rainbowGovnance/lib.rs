#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

pub use self::rainbow_govnance::RainbowGovnance;

#[ink::contract]
mod rainbow_govnance {
    use alloc::string::String;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };
  
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std", 
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DisplayDaoGovnanceInfo {
        manager: AccountId,
        g_name: String,
        desc: String,
    }
    #[ink(storage)]
    pub struct RainbowGovnance {
        manager:AccountId,
        name: String,
        desc: String,
        authority:StorageHashMap<AccountId, u64>,
    }
 

    impl RainbowGovnance {
        #[ink(constructor)]
        pub fn new(manager:AccountId) -> Self {
            Self { 
                manager,
                name:String::default(),
                desc:String::default(),
                authority:StorageHashMap::new(),
            }
        }
        #[ink(message)]
        pub fn set_name(&mut self,name: String){
            self.name = String::from(name);
        }
        #[ink(message)]
        pub fn get_name(&self) -> String{
            self.name.clone()
        }
        #[ink(message)]
        pub fn set_desc(&mut self ,desc:String){
            self.desc = String::from(desc);
        }
        #[ink(message)]
        pub fn get_desc(&self) ->String{
            self.desc.clone()
        }

        #[ink(message)]
        pub fn set_authority(&mut self, user_addr:AccountId, authority_id:u64) ->bool{
            assert!(self.env().caller() == self.manager);
            self.authority.insert(user_addr,authority_id);
            true
        }
        #[ink(message)]
        pub fn set_new_manager(&mut self, to:AccountId) ->bool {
            assert!(self.env().caller() == self.manager);
            self.manager = to;
            true
        }
        #[ink(message)]
        pub fn get_manager(&self) ->AccountId{
            self.manager
        }
        #[ink(message)]
        pub fn get_authority_id(&self,user_addr:AccountId) -> u64 {
            *self.authority.get(&user_addr).unwrap()
        }
        #[ink(message)]
        pub fn get_baseInfo(&self) ->DisplayDaoGovnanceInfo{
            DisplayDaoGovnanceInfo{
                manager: self.manager,
                g_name: self.name.clone(),
                desc: self.desc.clone(),
            }
        }
    }
}
