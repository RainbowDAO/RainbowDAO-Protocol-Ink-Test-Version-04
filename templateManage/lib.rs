#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
pub use self::template_manage::{
    TemplateManage,
};

#[ink::contract]
mod template_manage {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadLayout,
        },
        collections::HashMap as StorageHashMap,
    };

    #[derive(Debug, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct Template {
        pub id: u64,
        pub owner: AccountId,
        pub name: String,
        pub dao_manage_code_hash: Hash,
        pub components: BTreeMap<String, Hash>,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct TemplateManage {
        owner: AccountId,
        template_index: u64,
        template_map: StorageHashMap<u64, Template>,
    }
    #[ink(event)]
    pub struct AddTemplate {
        #[ink(topic)]
        index: u64,
        #[ink(topic)]
        owner: Option<AccountId>,
    }
    impl TemplateManage {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self {
                owner,
                template_index: 0,
                template_map: StorageHashMap::new(),
            }
        }
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }
        
        #[ink(message)]
        pub fn creat_template(&mut self, name: String, dao_manage_code_hash: Hash) -> bool {
            assert_eq!(self.template_index + 1 > self.template_index, true);
            let from = self.env().caller();
            self.template_map.insert(self.template_index, Template {
                id: self.template_index,
                owner: from,
                name,
                dao_manage_code_hash,
                components: BTreeMap::new(),
            });
            self.env().emit_event(AddTemplate {
                index: self.template_index,
                owner: Some(from),
            });
            self.template_index += 1;
            true
        }

        #[ink(message)]
        pub fn add_hash_by_index(&mut self,index:u64, name: String, code_hash: Hash) -> bool {
            assert_eq!(self.owner == self.env().caller(), true);
            assert_eq!(self.template_index <= index, true);
            self.template_map.get_mut(&index).unwrap().components.insert(name,code_hash);
            true
        }

        #[ink(message)]
        pub fn list_templates(&self) -> Vec<Template> {
            let mut temp_vec = Vec::new();
            let mut iter = self.template_map.values();
            let mut temp = iter.next();
            while temp.is_some() {
                temp_vec.push(temp.unwrap().clone());
                temp = iter.next();
            }
            temp_vec
        }
        #[ink(message)]
        pub fn query_template_by_index(&self, index: u64) -> Template {
            self.template_map.get(&index).unwrap().clone()
        }
    }
}