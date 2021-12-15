#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::dao_base_info::DaoBaseInfo;

#[ink::contract]
mod dao_base_info {
    // use route_manage::RouteManage;
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

    pub struct DisplayDaoBaseInfo {
        creator: AccountId,
        name: String,
        logo: String,
        desc: String,
    }
    #[ink(storage)]
    pub struct DaoBaseInfo {
        creator: AccountId,
        name: String,
        logo: String,
        desc: String,
        category:StorageHashMap<AccountId, u64>,
    }

    impl DaoBaseInfo {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                name:String::default(),
                logo:String::default(),
                desc:String::default(),
                category:StorageHashMap::new(),
                creator:Default::default(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }
        #[ink(message)]
        pub fn init_daoBaseInfo(&mut self, name:String, logo:String, desc:String){

            self.set_name(name);
            self.set_logo(logo);
            self.set_desc(desc);
            let caller = self.env().caller();
            self._set_creator(caller);
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
        pub fn set_logo(&mut self,logo: String){
            self.logo = String::from(logo);
        }
        #[ink(message)]
        pub fn get_logo(&self) -> String{
            self.logo.clone()
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
        pub fn set_dao_category(&mut self ,dao_addr:AccountId, dao_category:u64) ->bool{
            if dao_category == 1 {
                self.category.insert(dao_addr, dao_category);
            }
            if dao_category == 2 {
                self.category.insert(dao_addr, dao_category);
            }
            if dao_category == 3 {
                self.category.insert(dao_addr, dao_category);
            }
         true
        }
        #[ink(message)]
        pub fn get_dao_category(&self,dao_addr:AccountId) ->u64{
            *self.category.get(&dao_addr).unwrap()
        }
        #[ink(message)]
        pub fn _set_creator(&mut self, creator:AccountId){
            let caller = self.env().caller();
            if self.creator == AccountId::default() || caller == self.creator{
                self.creator = creator;
            }
        }

        #[ink(message)]
        pub fn get_creator(&self) ->AccountId {
            self.creator
        }
        #[ink(message)]
        pub fn get_baseInfo(&self) ->DisplayDaoBaseInfo{
            DisplayDaoBaseInfo{
                creator: self.creator,
                name: self.name.clone(),
                logo: self.logo.clone(),
                desc: self.desc.clone(),
            }
        }
    }
}