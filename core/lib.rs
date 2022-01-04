#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod core {
    use alloc::string::String;
    use role_manage::RoleManage;
    use route_manage::RouteManage;
    use privilege_manage::PrivilegeManage;
    ///rent value
    const RENT_VALUE: u128 = 1000 * 1_000_000_000_000;

    use ink_storage::{
        traits::{PackedLayout, SpreadLayout},
    };

    #[derive(Debug, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct ContractInstance {
        pub role_manage: Option<RoleManage>,
        pub privilege_manage:Option<PrivilegeManage>,
        pub route_manage: Option<RouteManage>,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default)]
    #[cfg_attr(feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct ContractAddr {
        pub role_manage_addr: Option<AccountId>,
        pub privilege_manage_addr:Option<AccountId>,
        pub route_manage_addr: Option<AccountId>,
    }

    #[ink(storage)]
    pub struct Core {
        pub owner:AccountId,
        pub contract_instance: ContractInstance,
        pub contract_addr: ContractAddr,
    }
    impl Core {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self {
                owner,
                contract_instance:ContractInstance{
                    role_manage: None,
                    privilege_manage: None,
                    route_manage: None,
                },
                contract_addr:ContractAddr{
                    role_manage_addr: None,
                    privilege_manage_addr: None,
                    route_manage_addr: None,
                }
            }
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId{
            self.owner
        }

        #[ink(message)]
        pub fn init(&mut self, version: u32,role_code_hash: Hash,privilege_code_hash: Hash,route_code_hash: Hash) -> bool {
            let salt = version.to_le_bytes();
            let role_manage = RoleManage::new(self.env().account_id())
                .endowment(RENT_VALUE)
                .code_hash(role_code_hash)
                .salt_bytes(salt)
                .params();
            let init_role_result = ink_env::instantiate_contract(&role_manage);
            let role_manage_addr = init_role_result.expect("failed at instantiating the `roleManager` contract");
            let role_contract_instance = ink_env::call::FromAccountId::from_account_id(role_manage_addr);
            self.contract_instance.role_manage = Some(role_contract_instance);
            self.contract_addr.role_manage_addr = Some(role_manage_addr);

            let privilege_manage = PrivilegeManage::new(self.env().account_id())
                .endowment(RENT_VALUE)
                .code_hash(privilege_code_hash)
                .salt_bytes(salt)
                .params();
            let init_privilege_result = ink_env::instantiate_contract(&privilege_manage);
            let privilege_manage_addr = init_privilege_result.expect("failed at instantiating the `PrivilegeManage` contract");
            let privilege_contract_instance = ink_env::call::FromAccountId::from_account_id(role_manage_addr);
            self.contract_instance.privilege_manage = Some(privilege_contract_instance);
            self.contract_addr.privilege_manage_addr = Some(privilege_manage_addr);

            let route_manage = RouteManage::new(self.env().account_id(),self.env().account_id())
                .endowment(RENT_VALUE)
                .code_hash(route_code_hash)
                .salt_bytes(salt)
                .params();
            let init_route_result = ink_env::instantiate_contract(&route_manage);
            let route_manage_addr = init_route_result.expect("failed at instantiating the `RouteManage` contract");
            let route_contract_instance = ink_env::call::FromAccountId::from_account_id(role_manage_addr);
            self.contract_instance.route_manage = Some(route_contract_instance);
            self.contract_addr.route_manage_addr = Some(route_manage_addr);
            true
        }

        #[ink(message)]
        pub fn add_role(&mut self, name: String) {
            self.contract_instance.role_manage.as_mut().unwrap().add_role(name);
        }
        #[ink(message)]
        pub fn add_privilege(&mut self, name: String) {
            self.contract_instance.privilege_manage.as_mut().unwrap().add_privilege(name);
        }
        #[ink(message)]
        pub fn add_route(&mut self, name: String,value: String) {
            self.contract_instance.route_manage.as_mut().unwrap().add_route(name,value);
        }

        #[ink(message)]
        pub fn get_balance(&self) -> u128 {
            return Self::env().balance();
        }

        #[ink(message)]
        pub fn get_role_manage_address(&self) -> Option<AccountId> {
            self.contract_addr.role_manage_addr
        }
    }
   
}