#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

pub use self::dao_proposal::DaoProposal;

#[ink::contract]
mod dao_proposal {
    use alloc::string::String;
    use dao_vote::DaoVote;
    use erc20::Erc20;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };
    

    #[ink(storage)]
    pub struct DaoProposal {
        manager_address:AccountId,
        proposal_id:u64,
        join_start_time:u64,
        proposal_name:String,
        proposal_category:u32,///1,change contract prama ,2,change role 
        vote_time:u64,
        proposal_check_status:StorageHashMap<(String,u64), u32 >,
        erc20_address:AccountId,
        vault_address:AccountId,
        // proposal_status:u32,///1 voting 2,over 
        all_proposal:Proposal,
        a_proposals:StorageHashMap<u64,Proposal>,
        proposal_vote:StorageHashMap<String,u64>,
        voted_list:StorageHashMap<AccountId, u64>,
        cancel_voted_list:StorageHashMap<AccountId, u64>,
        join_start_time_map:StorageHashMap<AccountId,u64>,
        candidate_id:u64,
        candidate_list:StorageHashMap<u64, AccountId>,
        vote:u32,
    }
    #[derive(Debug, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct Proposal{
        index:u64,
        name:String,
        category:u32,
        start_time:u64,
        end_time:u64,
        publicity_end_time:u64,
        vote:u64,
        nay_vote:u64,
        cancel_vote:u64,
        pass_rate:u64,
        cancel_pass_rate:u64,
        to:AccountId,
        token_amount:u64,
    }
    // pub struct 
    impl DaoProposal {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(owner:AccountId, erc20_addr: AccountId) -> Self {
            Self { 
                manager_address:owner,
                proposal_id:0,
                candidate_id:0,
                join_start_time:0,
                join_start_time_map:StorageHashMap::new(),
                proposal_name:String::from(""),
                vault_address:Default::default(),
                proposal_category:0,
                vote_time:0,
                vote:0,
                proposal_check_status:StorageHashMap::new(),
                erc20_address:erc20_addr,
                all_proposal:Proposal{
                    index:0,
                    name:String::from(""),
                    category:0,
                    start_time:0,
                    end_time:0,
                    publicity_end_time:0,
                    vote:0,
                    nay_vote:0,
                    cancel_vote:0,
                    pass_rate:0,
                    cancel_pass_rate:0,
                    to:Default::default(),
                    token_amount:0,
                },
                a_proposals:StorageHashMap::new(),
                proposal_vote:StorageHashMap::new(),
                voted_list:StorageHashMap::new(),
                cancel_voted_list:StorageHashMap::new(),
                candidate_list:StorageHashMap::new(),
                // proposal_status:0,
             }
        }



        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        
        ///转移资产提案
        #[ink(message)]
        pub fn creat_proposal(&mut self ,proposal_name:String,pass_rate:u64,proposal_category:u32,vote_time:Timestamp,publicity_end_time:Timestamp , to_addr:AccountId ,to_value:u64,cancel_pass_rate:u64) -> bool {
            let caller = self.env().caller();
            let balance_of_caller: Erc20 = ink_env::call::FromAccountId::from_account_id(self.erc20_address);
            let balance = balance_of_caller.balance_of(caller);
            let time = self.env().block_timestamp();

            if balance >= 0 {
                if vote_time -time <= 0{
                    return false;
                }
                self.proposal_id+=1;
                let p_status = 1;
                let a = Proposal{ 
                    index:self.proposal_id,
                    name:proposal_name.clone(),
                    category:proposal_category,
                    start_time:time,
                    end_time:vote_time,
                    publicity_end_time:publicity_end_time,
                    vote:0,
                    nay_vote:0,
                    cancel_vote:0,
                    pass_rate:pass_rate,
                    cancel_pass_rate:cancel_pass_rate,
                    to:to_addr,
                    token_amount:to_value,
                };
                self.a_proposals.insert(self.proposal_id,a.clone());

                self.all_proposal = a;
                self.proposal_check_status.insert((proposal_name ,self.proposal_id), p_status);
            }
            true
        }
        #[ink(message)]
        pub fn proposal_voting(&mut self ,proposal_name:String ,proposal_id:u64 ,agree:i32 ) ->bool{
            assert!(self._check_status(proposal_name.clone(), proposal_id) == 1);
            let caller = self.env().caller();
            assert!(self._check_voted(caller) != &proposal_id);
            let a = self._have_erc20(caller);
            let time = self.env().block_timestamp();
            // if  a > 0 {
            //     if self.all_proposal.start_time < time && time < self.all_proposal.time{
            //     self.all_proposal.vote += 1;
            //     self.voted_list.insert(caller, proposal_id);
            //     if self.all_proposal.vote == 20 {
            //         let p_status = 2;
            //         self.proposal_check_status.insert((proposal_name,proposal_id), p_status);
            //     }
            // }
            // }
            if a > 0 {
                let mut b = self.a_proposals.get_mut(&proposal_id).unwrap();
                if b.start_time < time && time <b.end_time{
                    if agree == 1{
                    b.vote +=1;
                }else{
                    b.nay_vote +=1;
                }
                    self.cancel_voted_list.insert(caller,proposal_id);
                    if b.vote/(b.nay_vote + b.vote) >= b.pass_rate/100000 {

                        let p_status = 2;

                        self.proposal_check_status.insert((proposal_name,proposal_id),p_status);
                    }
                }
            }
            true
        }
        
        fn _check_voted(&self, user_addr:AccountId) -> &u64{
            self.voted_list.get(&user_addr).unwrap_or(&0)
        }
        fn _cancel_check_voted(&self,user_addr:AccountId) ->&u64 {
            self.cancel_voted_list.get(&user_addr).unwrap_or(&0)
        }
        fn _have_erc20(&self, user_addr:AccountId) -> u64 {
            let balance_of_caller:Erc20 =ink_env::call::FromAccountId::from_account_id(self.erc20_address);
            let balance = balance_of_caller.balance_of(user_addr);
            balance
        }
        #[ink(message)]
        pub fn show_proposal(&mut self ,proposal_id:u64,proposal_name:String,disagree:i32) -> bool{
            assert!(self._check_status(proposal_name.clone(), proposal_id) == 2);
            let caller = self.env().caller();
            assert!(self._cancel_check_voted(caller) != &proposal_id);
            let a = self._have_erc20(caller);
            let time = self.env().block_timestamp();
            if a > 0 {
                let mut b = self.a_proposals.get_mut(&proposal_id).unwrap();
                if b.end_time < time && time <b.publicity_end_time{
                    if disagree == 1{
                    b.cancel_vote +=1;
                }
                    self.cancel_voted_list.insert(caller,proposal_id);
                    if b.cancel_vote/(b.nay_vote + b.vote) <= b.cancel_pass_rate/100000{

                        let mut balance_of_caller:Erc20 =ink_env::call::FromAccountId::from_account_id(self.erc20_address);
                        let transfer_token = balance_of_caller.transfer_from(self.vault_address,self.a_proposals.get(&proposal_id).unwrap().to ,self.a_proposals.get(&proposal_id).unwrap().token_amount);
                            
                        let p_status = 3;

                        self.proposal_check_status.insert((proposal_name,proposal_id),p_status);
                    }else{
                        let p_status = 4;
                        self.proposal_check_status.insert((proposal_name,proposal_id),p_status);

                    }
                }
            }
            true
        }
        ///1 means the voting period, 2 means the end of the voting 3, the arbitration period (returns 3 successfully) 4, the arbitration failure is put on hold
        #[ink(message)]
        pub fn _check_status(&self, proposal_name:String , proposal_id:u64 ) ->u32{
            *self.proposal_check_status.get(&(proposal_name,proposal_id)).unwrap_or(&0)
        }
        #[ink(message)]
        pub fn electio_proposal_creat(&mut self ,candidate:AccountId,candidate_name:String) ->bool{
            let caller = self.env().caller();
            let a = self._have_erc20(caller);
            let time = self.env().block_timestamp();
            if a > 0 && time - self.get_time(candidate) >= 604800000 {
                self.candidate_list.insert(self.candidate_id,candidate);
                self.candidate_id+=1;
            }
            true
        }
        fn get_time(&self, user_addr:AccountId) ->Timestamp{
            *self.join_start_time_map.get(&user_addr).unwrap()
        }
        ///When the dao management contract is initialized, inject member parameters;
        #[ink(message)]
        pub fn insert_join_time(&mut self,user:AccountId, join_time:Timestamp) ->bool{
            assert_eq!(self._have_erc20(user) > 0 ,true);
            self.join_start_time_map.insert(user,join_time);
            true
        }
        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn cancle_proposal(&mut self, proposal_id:u64 ) -> bool {
            let caller = self.env().caller();
            assert_eq!(caller == self.manager_address, true);
            self.a_proposals.take(&proposal_id);
            true
        }
        fn _get_caller(&self) -> AccountId{
            return self.env().caller();
        }
        #[ink(message)]
        pub fn get_manager_addr(&self) ->AccountId{
            self.manager_address

        }
        #[ink(message)]
        pub fn set_new_manager_addr(&mut self , to:AccountId) -> bool{
            assert_eq!(self._get_caller() == self.manager_address ,true);
            self.manager_address = to;
            true
        }
        #[ink(message)]
        pub fn get_proposal_by_id(&self,proposal_id:u64) -> Proposal{
            self.a_proposals.get(&proposal_id).unwrap().clone()
        }
        #[ink(message)]
        pub fn set_vault_addr(&mut self, vault_addr:AccountId) ->bool{
            self.vault_address = vault_addr;
            true
        }
        #[ink(message)]
        pub fn get_voted_user(&self, user_addr:AccountId) -> u64{
            *self.voted_list.get(&user_addr).unwrap()
        }
    }
}
