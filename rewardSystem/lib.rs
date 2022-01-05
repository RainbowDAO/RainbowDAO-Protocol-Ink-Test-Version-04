#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::reward_system::RewardSystem;

#[ink::contract]
mod reward_system {
    use alloc::string::String;
    use ink_storage::{collections::HashMap as StorageHashMap};

  
    #[ink(storage)]
    pub struct RewardSystem {
        turn_or_off: bool,
        owner:AccountId,
        level:u64,
        dao_type: StorageHashMap<String, u64>,
        refer_user: StorageHashMap<AccountId,AccountId>,
        my_invite_amount: StorageHashMap<(AccountId,u64), u64>,//Corresponding number of address corresponding levels
        user_amount:StorageHashMap<AccountId,u64>,
        income_proportion:u64,

    }

    impl RewardSystem {
        #[ink(constructor)]
        pub fn new(init: bool,owner:AccountId ) -> Self {
            Self {
                owner,
                turn_or_off: init,
                dao_type:StorageHashMap::new(),
                my_invite_amount:StorageHashMap::new(),
                refer_user:StorageHashMap::new(),
                user_amount:StorageHashMap::new(),
                level:1,
                income_proportion:0,
            }
        }
        #[ink(message)]
        pub fn set_level(&mut self, set:u64) ->bool{
            assert!(self.turn_or_off == true);
            assert!(self.env().caller() == self.owner);
            self.level = set;
            true
        }
        #[ink(message)]
        pub fn set_income_proportion(&mut self, proportion:u64) ->bool{
            assert!(self.turn_or_off == true);
            assert!(self.env().caller() == self.owner);
            self.income_proportion = proportion/100;
            true
        }
        #[ink(message)]
        pub fn insert_dao_type(&mut self,dao_name:String, dao_type_num:u64) ->bool{
            self.dao_type.insert(dao_name,dao_type_num);
            true
        }
        #[ink(message)]
        #[ink(selector = 0xAAAAAAAA)]
        pub fn insert_refer_user(&mut self, invite_addr:AccountId, be_invited_addr:AccountId) ->bool{
            self.refer_user.insert(invite_addr,be_invited_addr);
            true
        }
        #[ink(message)]
        pub fn basic_reward(&mut self,  sender:AccountId,amount:u64) ->u64 {
            let mut i = 1;
            let mut b = 0;
            while i <= self.level {
                let a =  self.refer_user.get(&sender);
                if a.is_some(){
                    self.my_invite_amount.insert((sender,i),amount);
                    b+=amount
                }else{
                    break;
                }
                i+=1;
                self.basic_reward(*a.unwrap(),self.user_amount.get(&a.unwrap()).unwrap().clone());
            }
            b
        }

        #[ink(message)]
        pub fn change_owner(&mut self,to:AccountId) ->bool{
            assert!(self.env().caller() == self.owner);
            self.owner = to;
            true
        }
        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn change_status(&mut self)->bool{
            self.turn_or_off = !self.turn_or_off;
            true
        }
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.turn_or_off
        }
        #[ink(message)]
        pub fn get_be_invited(&self,user_addr:AccountId) ->AccountId{
            *self.refer_user.get(&user_addr).unwrap()
        }
    }
}
