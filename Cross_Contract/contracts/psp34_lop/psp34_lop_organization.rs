#![cfg_attr(not(feature = "std"), no_std)]
use ink::{ env::{DefaultEnvironment, Environment},
         primitives::AccountId,
};
 //********************** */
 #[derive(PartialEq, Debug, scale::Encode, scale::Decode)]
 #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
 pub enum ContractError {
     
    YouCantMint,
        
}

 #[ink::trait_definition]
pub trait psp34_lop_organization {

    #[ink(message)]
    fn mint_token(&mut self) -> bool ;

}
