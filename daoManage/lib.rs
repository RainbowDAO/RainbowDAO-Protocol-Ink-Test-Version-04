#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
pub use self::dao_manage::DaoManage;

#[ink::contract]
mod dao_manage {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use dao_base_info::DaoBaseInfo;
    use erc20_factory::Erc20Factory;
    use dao_vault::DaoVaultManager;
    use dao_user::DaoUsers;
    use dao_proposal::DaoProposal;
    use dao_vote::DaoVote;
    use rainbow_govnance::RainbowGovnance;
    use dao_category::DaoCategory;
    use reward_system::RewardSystem;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };
    const RENT_VALUE: u128 = 1000 * 1_000_000_000_000;

    #[ink(storage)]
    pub struct DaoManage {
        // dao_addr:AccountId,
        dao_manager:AccountId,
        reward_addr:AccountId,
        erc20_addr:AccountId,
        turn_or_off_reward_system:bool,
        pub contract_instance: ContractInstance,
        pub contract_addr: ContractAddr,
        contract_hash:StorageHashMap<String, Hash>,
        all_contract_addr:StorageHashMap<String,AccountId>
    }
    #[derive(Debug, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct ContractInstance {
        pub dao_base_info: Option<DaoBaseInfo>,
        pub erc20_factory:Option<Erc20Factory>,
        pub dao_vault:Option<DaoVaultManager>, 
        pub dao_user:Option<DaoUsers>,
        pub dao_proposal:Option<DaoProposal>,
        pub dao_vote:Option<DaoVote>,
        pub rainbow_govnance:Option<RainbowGovnance>,
        pub dao_category:Option<DaoCategory>,
        pub reward_system:Option<RewardSystem>,
    }
    
    #[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default)]
    #[cfg_attr(feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct ContractAddr {
        pub dao_base_info_addr: Option<AccountId>,
        pub erc20_factory_addr:Option<AccountId>,
        pub dao_vault_addr: Option<AccountId>,
        pub dao_user_addr: Option<AccountId>,
        pub dao_proposal_addr:Option<AccountId>,
        pub dao_vote_addr:Option<AccountId>,
        pub rainbow_govnance_addr:Option<AccountId>,
        pub dao_category_addr:Option<AccountId>,
        pub reward_system_addr:Option<AccountId>,
    }

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct Contract{
        contract_name:String,
        contract_hash:Hash,
    }

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct AllContractAddr{
        contract_name:String,
        contract_addr:AccountId,
    }
    impl DaoManage {
        ///plz Enter in daoBaseInfo erc20Factory daoVault daoUser ,daoProposal,daoVote ,in this order;
        #[ink(constructor)]
        pub fn new( contract_name:Vec<String>,contract_hash:Vec<Hash>,owner:AccountId) -> Self {
            let mut map : StorageHashMap<String, Hash> = StorageHashMap::new();
            for (name,hash) in contract_name.iter().zip(contract_hash){
                map.insert(name.clone(),hash);
            }
            Self {
                reward_addr:AccountId::default(),
                erc20_addr:AccountId::default(),
                dao_manager:owner,
                contract_hash:map,
                turn_or_off_reward_system:false,
                // dao_addr:Default::default(),
                all_contract_addr:StorageHashMap::new(),
                contract_instance:ContractInstance{
                    dao_base_info: None,
                    erc20_factory: None,
                    dao_vault: None,
                    dao_user:None,
                    dao_proposal:None,
                    dao_vote:None,
                    rainbow_govnance:None,
                    dao_category:None,
                    reward_system:None,
                },
                contract_addr:ContractAddr{
                    dao_base_info_addr: None,
                    erc20_factory_addr: None,
                    dao_vault_addr: None,
                    dao_user_addr:None,
                    dao_proposal_addr:None,
                    dao_vote_addr:None,
                    rainbow_govnance_addr:None,
                    dao_category_addr:None,
                    reward_system_addr:None,
                }
            }
        }
        ///plz enter in this order ! - v - ! 
        #[ink(message)]
        pub fn init(&mut self) ->bool{
            self.init_dao_base_info(String::from("daoBaseInfo"));
            self.init_erc20_factory(String::from("erc20Factory"));
            self.init_dao_vault(String::from("daoVault"));
            self.init_dao_user(String::from("daoUser"));
            self.init_dao_category(String::from("DaoCategory"));

            true
        }

        #[ink(message)]
        pub fn init_dao_base_info(&mut self,contract_name:String) -> bool {
            let total_balance = Self::env().balance();
            assert_eq!(total_balance > RENT_VALUE ,true);
            let caller = self.env().caller();
            let num:u64 = self.env().block_timestamp();
            let salt = num.to_le_bytes();
            let instance_params = DaoBaseInfo::new()
                .endowment(RENT_VALUE)
                .code_hash(*self.contract_hash.get(&contract_name).unwrap())
                .salt_bytes(salt)
                .params();
            let contract_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = contract_result.expect("failed at instantiating the `Base` contract");
            let contract_instance: DaoBaseInfo = ink_env::call::FromAccountId::from_account_id(contract_addr);
            self.all_contract_addr.insert(contract_name,contract_addr);
            self.contract_instance.dao_base_info = Some(contract_instance);
            self.contract_addr.dao_base_info_addr = Some(contract_addr);   

            true
        }
        #[ink(message)]
        pub fn init_erc20_factory(&mut self, contract_name:String) ->bool{
            let caller = self.env().caller();
            let total_balance = Self::env().balance();
            assert_eq!(total_balance > RENT_VALUE ,true);
            let num:u64 = self.env().block_timestamp();
            let salt = num.to_le_bytes();
            let instance_params = Erc20Factory::new()
                .endowment(RENT_VALUE)
                .code_hash(*self.contract_hash.get(&contract_name).unwrap())
                .salt_bytes(salt)
                .params();
            let contract_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = contract_result.expect("failed at instantiating the `Base` contract");
            let contract_instance: Erc20Factory = ink_env::call::FromAccountId::from_account_id(contract_addr);
            self.all_contract_addr.insert(contract_name,contract_addr);

            self.contract_instance.erc20_factory = Some(contract_instance);
            self.contract_addr.erc20_factory_addr = Some(contract_addr);   

            true
        }
       #[ink(message)]
       pub fn init_dao_vault(&mut self , contract_name:String) ->bool{
        let caller = self.env().caller();
        let total_balance = Self::env().balance();
        assert_eq!(total_balance > RENT_VALUE ,true);
        let num:u64 = self.env().block_timestamp();
        let salt = num.to_le_bytes();
        let instance_params = DaoVaultManager::new(caller)
            .endowment(RENT_VALUE)
            .code_hash(*self.contract_hash.get(&contract_name).unwrap())
            .salt_bytes(salt)
            .params();
        let contract_result = ink_env::instantiate_contract(&instance_params);
        let contract_addr = contract_result.expect("failed at instantiating the `Base` contract");
        let contract_instance: DaoVaultManager = ink_env::call::FromAccountId::from_account_id(contract_addr);
        self.all_contract_addr.insert(contract_name,contract_addr);

        self.contract_instance.dao_vault = Some(contract_instance);
        self.contract_addr.dao_vault_addr = Some(contract_addr);   

        true
       }
       #[ink(message)]
       pub fn init_dao_user(&mut self , contract_name:String) ->bool{
        let caller = self.env().caller();
        let total_balance = Self::env().balance();
        assert_eq!(total_balance > RENT_VALUE ,true);
        let num:u64 = self.env().block_timestamp();
        let salt = num.to_le_bytes();
        let instance_params = DaoUsers::new(caller, self.erc20_addr, self.reward_addr)
            .endowment(RENT_VALUE)
            .code_hash(*self.contract_hash.get(&contract_name).unwrap())
            .salt_bytes(salt)
            .params();
        let contract_result = ink_env::instantiate_contract(&instance_params);
        let contract_addr = contract_result.expect("failed at instantiating the `Base` contract");
        let contract_instance: DaoUsers = ink_env::call::FromAccountId::from_account_id(contract_addr);

        self.all_contract_addr.insert(contract_name,contract_addr);
        self.contract_instance.dao_user = Some(contract_instance);
        self.contract_addr.dao_user_addr = Some(contract_addr);   

        true
       }
       ///that need contract_name: daoProposal ,tokenAddr, you can't init it but manager;
       #[ink(message)]
       pub fn init_dao_proposal(&mut self , contract_name:String , token:AccountId) ->bool{
        let caller = self.env().caller();
        assert_eq!(caller == self.dao_manager,true);
        let total_balance = Self::env().balance();
        assert_eq!(total_balance > RENT_VALUE ,true);
        let num:u64 = self.env().block_timestamp();
        let salt = num.to_le_bytes();
        // let a = self.contract_instance.erc20_factory.as_ref().unwrap().get_token_by_index(1);
        let instance_params = DaoProposal::new(caller,token)
            .endowment(RENT_VALUE)
            .code_hash(*self.contract_hash.get(&contract_name).unwrap())
            .salt_bytes(salt)
            .params();
        let contract_result = ink_env::instantiate_contract(&instance_params);
        let contract_addr = contract_result.expect("failed at instantiating the `Base` contract");
        let contract_instance: DaoProposal = ink_env::call::FromAccountId::from_account_id(contract_addr);

        self.all_contract_addr.insert(contract_name,contract_addr);

        self.contract_instance.dao_proposal = Some(contract_instance);
        self.contract_addr.dao_proposal_addr = Some(contract_addr);   

        true
       }
       #[ink(message)]
       pub fn init_dao_vote(&mut self , contract_name:String , token:AccountId) ->bool{
        let caller = self.env().caller();
        assert_eq!(caller == self.dao_manager,true);
        let total_balance = Self::env().balance();
        assert_eq!(total_balance > RENT_VALUE ,true);
        let num:u64 = self.env().block_timestamp();
        let salt = num.to_le_bytes();
        // let a = self.contract_instance.erc20_factory.as_ref().unwrap().get_token_by_index(1);
        let instance_params = DaoVote::new(caller,token)
            .endowment(RENT_VALUE)
            .code_hash(*self.contract_hash.get(&contract_name).unwrap())
            .salt_bytes(salt)
            .params();
        let contract_result = ink_env::instantiate_contract(&instance_params);
        let contract_addr = contract_result.expect("failed at instantiating the `Base` contract");
        let contract_instance: DaoVote = ink_env::call::FromAccountId::from_account_id(contract_addr);
        self.all_contract_addr.insert(contract_name,contract_addr);

        self.contract_instance.dao_vote = Some(contract_instance);
        self.contract_addr.dao_vote_addr = Some(contract_addr);   

        true
       }
       #[ink(message)]
       pub fn init_rainbow_govnance(&mut self ,contract_name:String) ->bool{
        let caller = self.env().caller();
        assert_eq!(caller == self.dao_manager,true);
        let total_balance = Self::env().balance();
        assert_eq!(total_balance > RENT_VALUE ,true);
        let num:u64 = self.env().block_timestamp();
        let salt = num.to_le_bytes();
        // let a = self.contract_instance.erc20_factory.as_ref().unwrap().get_token_by_index(1);
        let instance_params = RainbowGovnance::new(self.dao_manager)
            .endowment(RENT_VALUE)
            .code_hash(*self.contract_hash.get(&contract_name).unwrap())
            .salt_bytes(salt)
            .params();
        let contract_result = ink_env::instantiate_contract(&instance_params);
        let contract_addr = contract_result.expect("failed at instantiating the `Base` contract");
        let contract_instance: RainbowGovnance = ink_env::call::FromAccountId::from_account_id(contract_addr);
        self.all_contract_addr.insert(contract_name,contract_addr);

        self.contract_instance.rainbow_govnance = Some(contract_instance);
        self.contract_addr.rainbow_govnance_addr = Some(contract_addr);   

        true
       }
       #[ink(message)]
       pub fn init_dao_category(&mut self ,contract_name:String) ->bool{
        let caller = self.env().caller();
        assert_eq!(caller == self.dao_manager,true);
        let total_balance = Self::env().balance();
        assert_eq!(total_balance > RENT_VALUE ,true);
        let num:u64 = self.env().block_timestamp();
        let salt = num.to_le_bytes();
        // let a = self.contract_instance.erc20_factory.as_ref().unwrap().get_token_by_index(1);
        let instance_params = DaoCategory::new(self.dao_manager)
            .endowment(RENT_VALUE)
            .code_hash(*self.contract_hash.get(&contract_name).unwrap())
            .salt_bytes(salt)
            .params();
        let contract_result = ink_env::instantiate_contract(&instance_params);
        let contract_addr = contract_result.expect("failed at instantiating the `Base` contract");
        let contract_instance: DaoCategory = ink_env::call::FromAccountId::from_account_id(contract_addr);
        self.all_contract_addr.insert(contract_name,contract_addr);

        self.contract_instance.dao_category = Some(contract_instance);
        self.contract_addr.dao_category_addr = Some(contract_addr);   

        true
       }

       #[ink(message)]
       pub fn init_reward_system(&mut self ,contract_name:String) ->bool{
        let caller = self.env().caller();
        assert_eq!(caller == self.dao_manager,true);
        let total_balance = Self::env().balance();
        assert_eq!(total_balance > RENT_VALUE ,true);
        let num:u64 = self.env().block_timestamp();
        let salt = num.to_le_bytes();
        // let a = self.contract_instance.erc20_factory.as_ref().unwrap().get_token_by_index(1);
        let instance_params = RewardSystem::new(self.turn_or_off_reward_system,self.dao_manager)
            .endowment(RENT_VALUE)
            .code_hash(*self.contract_hash.get(&contract_name).unwrap())
            .salt_bytes(salt)
            .params();
        let contract_result = ink_env::instantiate_contract(&instance_params);
        let contract_addr = contract_result.expect("failed at instantiating the `Base` contract");
        let contract_instance: RewardSystem = ink_env::call::FromAccountId::from_account_id(contract_addr);
        self.all_contract_addr.insert(contract_name,contract_addr);

        self.contract_instance.reward_system = Some(contract_instance);
        self.contract_addr.reward_system_addr = Some(contract_addr);   

        true
       }
       #[ink(message)]
       pub fn turn_on_or_off_reward(&mut self) ->bool{
           assert_eq!(self.env().caller() == self.dao_manager,true);
            self.turn_or_off_reward_system = !self.turn_or_off_reward_system;
            true
       }
        #[ink(message)]
        pub fn contract_hash_list(&self) ->Vec<Contract>{
            let caller = self.env().caller();
            assert_eq!(self.dao_manager == caller, true);
            let mut new_list = Vec::new();
            let mut key = self.contract_hash.keys();
            let mut hash = self.contract_hash.values();
            let mut a = key.next();
            let mut b = hash.next();
            while a.is_some() {
            new_list.push(Contract{
                contract_name: a.unwrap().clone(),
                contract_hash: b.unwrap().clone(),
            });
            a = key.next();
            b = hash.next();
            }
            new_list
        }
        #[ink(message)]
        pub fn get_contract_addr(&self) ->Vec<AllContractAddr>{
            let mut new_list = Vec::new();
            let mut key = self.all_contract_addr.keys();
            let mut hash = self.all_contract_addr.values();
            let mut a = key.next();
            let mut b = hash.next();
            while a.is_some() {
            new_list.push(AllContractAddr{
                contract_name: a.unwrap().clone(),
                contract_addr: b.unwrap().clone(),
            });
            a = key.next();
            b = hash.next();
            }
            new_list
        }
 

        #[ink(message)]
        pub fn transfer_manager(&mut self,to:AccountId) ->bool{
            let caller = self.env().caller();
            assert_eq!(caller == self.dao_manager,true);
            self.dao_manager = to;
            true
        }

        #[ink(message)]
        pub fn get_balance(&self) -> u128 {
            return Self::env().balance();
        }
        ///setting new contract addr
        #[ink(message)]
       pub fn set_new_addr(&mut self,c_name:String, c_addr:AccountId) ->bool{
           let caller =  self.env().caller();
           assert_eq!(caller == self.dao_manager,true);
           self.all_contract_addr.take(&c_name);
           self.all_contract_addr.insert(c_name, c_addr);
           true
        }

        ///setting dao category
        #[ink(message)]
        pub fn set_category(&mut self,dao_category:u64 ) -> bool{
            if dao_category < 0 || dao_category > 4 {
                return false;
            }
            let caller =self.env().account_id();
            let mut instance:DaoBaseInfo = self.contract_instance.dao_base_info.as_ref().unwrap().clone();
            instance.set_dao_category(caller, dao_category)
        }
        ///Add DAO members
        #[ink(message)]
        pub fn dao_user_add_user(&mut self, user_address1: AccountId,nick_name1: String) ->bool{
            let mut instance:DaoUsers = self.contract_instance.dao_user.as_ref().unwrap().clone();
            instance.add_user(user_address1, nick_name1 )
        }
        ///Get User Join Time
        #[ink(message)]
        pub fn get_join_start_time(&self,user_addr:AccountId ) ->u64{
            let mut instance:DaoUsers = self.contract_instance.dao_user.as_ref().unwrap().clone();
            instance.get_join_time(user_addr)
        }
        ///Inject the user joining time into the DAOUser contract
        #[ink(message)]
        pub fn insert_dao_user_join_time(&mut self ,user:AccountId) ->bool {
            let mut instance:DaoUsers = self.contract_instance.dao_user.as_ref().unwrap().clone();
            let a = self.get_join_start_time(user);
                instance.insert_join_time(user, a);
                true
        }
        ///Inject the user joining time into the DAOVote contract
        #[ink(message)]
        pub fn insert_dao_vote_join_time(&mut self, user_addr:AccountId) ->bool{
            let mut instance: DaoVote = self.contract_instance.dao_vote.as_ref().unwrap().clone();
            let a = self.get_join_start_time(user_addr);
                instance.set_user_join_time(user_addr ,a);
                true
        }
        ///Inject the treasury address into the DaoProposal contract
        #[ink(message)]
        pub fn insert_dao_proposal_addr(&mut self, vault_addr:AccountId) -> bool{
            let mut instance: DaoProposal = self.contract_instance.dao_proposal.as_ref().unwrap().clone();
                instance.set_vault_addr(vault_addr);
                true
        }
        ///Inject the time for users to join the DaoProposal contract
        #[ink(message)]
        pub fn insert_dao_proposal_user_add_time(&mut self, user_addr:AccountId) ->bool{
            let mut instance: DaoProposal = self.contract_instance.dao_proposal.as_ref().unwrap().clone();
            instance.insert_join_time(user_addr , self.get_join_start_time(user_addr));
            true

        }
        ///Check if you have voted, voted, take the user’s vote
        #[ink(message)]
        pub fn dao_porposal_check(&mut self,user_addr:AccountId) ->bool{
            let mut instance: DaoProposal = self.contract_instance.dao_proposal.as_ref().unwrap().clone();
            let mut a= instance.get_voted_user(user_addr);
            if a > 0{
            let mut instance: DaoVote = self.contract_instance.dao_vote.as_ref().unwrap().clone();
            instance.take_self_vote(user_addr);
            }
            true
        }
        ///Get tickets
        #[ink(message)]
        pub fn get_voets(&mut self, user_addr:AccountId) ->bool{
            let mut instance: DaoVote = self.contract_instance.dao_vote.as_ref().unwrap().clone();
            instance.votes1(user_addr);
            true
        }
        ///Create proposal
        #[ink(message)]
        pub fn creat_proposal(&mut self,proposal_name:String,pass_rate:u64,proposal_category:u32,vote_time:Timestamp,publicity_end_time:Timestamp , to_addr:AccountId ,to_value:u64,cancel_pass_rate:u64)->bool{
            let mut instance: DaoProposal = self.contract_instance.dao_proposal.as_ref().unwrap().clone();
            instance.creat_proposal(proposal_name,pass_rate,proposal_category,vote_time,publicity_end_time , to_addr ,to_value,cancel_pass_rate);
            true
        }
        ///vote
        #[ink(message)]
        pub fn vote(&mut self,proposal_name:String ,proposal_id:u64, agree:i32) ->bool{
            let mut instance: DaoProposal = self.contract_instance.dao_proposal.as_ref().unwrap().clone();
            instance.proposal_voting(proposal_name,proposal_id ,agree);
            true
        }
        ///Proposal status query 1 is voting 2 closed
        #[ink(message)]
        pub fn check_proposal_status(&self, proposal_name:String , proposal_id:u64) ->u32{
            let mut instance: DaoProposal = self.contract_instance.dao_proposal.as_ref().unwrap().clone();
            instance._check_status(proposal_name ,proposal_id)
        }
        #[ink(message)]
        pub fn mint_new_token(&mut self, erc20_hash: Hash,name:String ,symbol:String ,initial_supply:u64, decimals:u8, controller: AccountId) ->bool{
            let mut instance: Erc20Factory = self.contract_instance.erc20_factory.as_ref().unwrap().clone();
            instance.mint_token(erc20_hash ,name,symbol,initial_supply,decimals,controller);
            true
        }
        #[ink(message)]
        pub fn check_erc20_addr(&mut self,name:String) ->bool{
            let mut instance: Erc20Factory = self.contract_instance.erc20_factory.as_ref().unwrap().clone();
           self.erc20_addr =  instance.get_token_by_symbol(name);
            true
        }
        #[ink(message)]
        pub fn insert_reward_system_addr(&mut self,reward_system_addr:AccountId) -> bool{
            assert!(self.env().caller() == self.dao_manager);
            self.reward_addr = reward_system_addr;
            true
        }
    }
}