#![cfg_attr(not(feature = "std"), no_std)]
use ink::{ env::{DefaultEnvironment, Environment},
         primitives::AccountId,
};

 //********************** */
 #[derive(PartialEq, Debug, scale::Encode, scale::Decode)]
 #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
 pub enum ContractError {
      
     YouAreNotVoter,
     AccounNotListed,
     NoAdmin,
     
 }

#[ink::trait_definition]
pub trait VotingOrganization {
    
    #[ink(message)]
    fn vote(&mut self, voter: AccountId, value: i32) -> bool;
   
    #[ink(message)]
    fn get_votes(&self, voter: AccountId) -> i32;

    #[ink(message)]
     fn add_voter(&mut self, voter: AccountId) -> bool ;
     #[ink(message)]
     fn remove_voter(&mut self, voter: AccountId) ->bool;
     #[ink(message)]
     fn change_admin(&mut self, voter: AccountId) ->bool;
}

