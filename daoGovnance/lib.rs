#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::dao_govnance::DaoGovnance;

#[ink::contract]

mod dao_govnance {
    


    use alloc::string::String;
    use alloc::vec::Vec;

    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
        },
        traits::{
            PackedLayout,
            SpreadLayout,
        }
    };

    type BehaviorId = u32;
     
    #[derive(scale::Encode,scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink_storage::traits::StorageLayout
        )
    )]
    
    pub struct Behavior{
        behavior_id: BehaviorId,
        behavior_title:String,
        contract_name:String,
        function_name:String,
    }
    #[ink(storage)]
    pub struct DaoGovnance{
        owner: AccountId,
        behavior_id:BehaviorId,
        behaviors:StorageHashMap<(String,String),Behavior> ,
        behaviors_auth: StorageHashMap<(AccountId,BehaviorId),Behavior>,
        
    }

    impl DaoGovnance {

        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self{
                owner,
                behavior_id:0,
                behaviors: StorageHashMap::new(),
                behaviors_auth: StorageHashMap::new(),

            }
        }
    #[ink(message)]
    pub fn has_pass(& self ,account_id:AccountId, contract_name: String, function_name: String) -> bool {
        return self._has_pass(account_id, contract_name,function_name);
    }
    fn _has_pass(& self ,account_id:AccountId,contract_name: String,function_name: String)  -> bool{
        if let Some(behavior) = self.behaviors.get(&(contract_name, function_name)){
            if let Some(_) = self.behaviors_auth.get(&(account_id,behavior.behavior_id)){
                return true;
            }
        }
        return false;
    }

    #[ink(message)]
    pub fn grant_pass(& mut self, account_id:AccountId ,contract_name: String, function_name: String) ->bool{
        let caller = self.env().caller();
        assert!(self.owner == caller || self._has_pass(caller, String::from("daoGovnance"), String::from("grant")));
        if let Some(behavior) = self.behaviors.get(&(contract_name, function_name)){
            let a: Behavior = Behavior{
                behavior_id:behavior.behavior_id,
                behavior_title:behavior.behavior_title.clone(),
                contract_name:behavior.contract_name.clone(),
                function_name:behavior.function_name.clone(),
            };
            self.behaviors_auth.insert((account_id,behavior.behavior_id), a);
            return true;
        }
        return false;
    }
    #[ink(message)]
    pub fn transfer_owner(
    &mut self, 
    to: AccountId,
    ) -> bool{
        assert!(self.owner == self.env().caller());
        self.owner = to;
        true
    }

    #[ink(message)]
    pub fn revoke_pass(&mut self, account_id:AccountId, contract_name: String,function_name: String) -> bool {
        let caller = self.env().caller();
        assert!(self.owner == caller || self._has_pass(caller, String::from("daoGovnance"), String::from("grant")));
        if let Some(behavior) = self.behaviors.get(&(contract_name, function_name)) {
            self.behaviors_auth.take(&(account_id,behavior.behavior_id));
            return true;

        }
        return false;
    }

    #[ink(message)]
    pub fn register_behavior(&mut self , contract_name: String,function_name: String, behavior_title:String) -> bool{
        let caller = self.env().caller();
        assert!(self.owner ==caller ||self._has_pass(caller, String::from("daoGovnance"), String::from("register")));
        let behavior_id = self.behavior_id;
        self.behavior_id+=1;
        let behavior = Behavior {
            behavior_id,
            behavior_title:behavior_title.clone(),
            contract_name:contract_name.clone(),
            function_name:function_name.clone(),
        };
        self.behaviors.insert((contract_name,function_name), behavior);
        true
    }
    #[ink(message)]
    pub fn cancel_behavior(&mut self, contract_name: String,function_name: String) -> bool{
        let caller = self.env().caller();
        assert!(self.owner == caller ||self._has_pass(caller, String::from("daoGovnance"), String::from("register")));
        self.behaviors.take(&(contract_name,function_name));
        true
    }
    #[ink(message)]
    pub fn show_behaviors_by_contract(& self, contract_name: String) ->Vec<Behavior>{

        let mut behaviors_vec:Vec<Behavior> = Vec::new();
        for ((cname,_),val) in &self.behaviors{
            if *cname == contract_name{
                let v:Behavior =Behavior{
                    behavior_id:val.behavior_id,
                    behavior_title:val.behavior_title.clone(),
                    contract_name: val.contract_name.clone(),
                    function_name:val.function_name.clone(),

                };
                behaviors_vec.push(v);

            }
        }
    behaviors_vec
    }

    #[ink(message)]
    pub fn show_behaviors_by_user(&self, owner: AccountId) -> Vec<Behavior> {
        let mut behaviors_vec : Vec<Behavior> = Vec::new();
        for((account_id,_),val) in &self.behaviors_auth{
            if *account_id ==owner{
                let v: Behavior =Behavior{
                    behavior_id:val.behavior_id,
                    behavior_title:val.behavior_title.clone(),
                    contract_name:val.contract_name.clone(),
                    function_name:val.function_name.clone(),

                };
                behaviors_vec.push(v);
            }
        }
        behaviors_vec
    }
    #[ink(message)]
    pub fn get_daogovnance_owner(&self) -> AccountId {
        return self.owner;
    }
    }
}
