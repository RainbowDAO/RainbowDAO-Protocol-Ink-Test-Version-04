#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod dao_factory {
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

    use template_manage::TemplateManage;
    use dao_manage::DaoManage;
    const RENT_VALUE: u128 = 1000 * 1_000_000_000_000;

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    pub struct DAOInstance {
        id: u64,
        owner: AccountId,
        size: u64,
        name: String,
        logo: String,
        desc: String,
        dao_manage: DaoManage,
        dao_manage_addr: AccountId,
    }
    
    #[ink(storage)]
    pub struct DaoFactory {
        owner: AccountId,
        template_addr: AccountId,
        template: TemplateManage,
        instance_index:u64,
        instance_map: StorageHashMap<u64, DAOInstance>,
        instance_map_by_owner: StorageHashMap<AccountId, Vec<u64>>,
    }
    #[ink(event)]
    pub struct InstanceDAO {
        #[ink(topic)]
        index: u64,
        #[ink(topic)]
        owner: Option<AccountId>,
        #[ink(topic)]
        dao_addr: AccountId,
    }
    impl DaoFactory {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                template_addr: Default::default(),
                template: Default::default(),
                instance_index:0,
                instance_map: StorageHashMap::new(),
                instance_map_by_owner: StorageHashMap::new(),
            }
        }
        #[ink(message)]
        pub fn  init_factory (&mut self, template_code_hash: Hash, version:u8) -> bool
        {
            // instance template_manage
            let salt = version.to_le_bytes();
            let instance_params = TemplateManage::new(self.owner)
                .endowment(RENT_VALUE)
                .code_hash(template_code_hash)
                .salt_bytes(&salt)
                .params();
            let init_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = init_result.expect("failed at instantiating the `TemplateManage` contract");
            self.template = ink_env::call::FromAccountId::from_account_id(contract_addr);
            self.template_addr = contract_addr;
            true
        }
        pub fn init_dao_by_template(&mut self, index: u64, contract_name:Vec<String>,contract_hash:Vec<Hash>,controller: AccountId,version: u8) -> bool {
            assert_eq!(self.instance_index + 1 > self.instance_index, true);
            
            let template = self.template.query_template_by_index(index);
            let dao_manage_code_hash = template.dao_manage_code_hash;
            let salt = version.to_le_bytes();
            let dao_instance_params = DaoManage::new(contract_name,contract_hash,controller)
                .endowment(RENT_VALUE)
                .code_hash(dao_manage_code_hash)
                .salt_bytes(salt)
                .params();
            let dao_init_result = ink_env::instantiate_contract(&dao_instance_params);
            let dao_addr = dao_init_result.expect("failed at instantiating the `DAO Instance` contract");
            let mut dao_instance: DaoManage = ink_env::call::FromAccountId::from_account_id(dao_addr);
            // dao_instance.set_template(template);
            self.env().emit_event(InstanceDAO {
                index: self.instance_index,
                owner: Some(controller),
                dao_addr: dao_addr,
            });
            let id_list = self.instance_map_by_owner.entry(controller.clone()).or_insert(Vec::new());
            id_list.push(self.instance_index);
            self.instance_map.insert(self.instance_index, DAOInstance {
                id: self.instance_index,
                owner: controller,
                size: 0,
                name: String::from(""),
                logo: String::from(""),
                desc: String::from(""),
                dao_manage: dao_instance,
                dao_manage_addr: dao_addr,
            });
            self.instance_index += 1;
            true
        }
    
    }
   
}