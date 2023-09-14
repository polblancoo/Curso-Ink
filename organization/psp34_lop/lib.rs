#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use crate::psp34_lop::ContractRef;



#[openbrush::implementation(PSP34, PSP34Mintable)]
#[openbrush::contract]
pub mod psp34_lop {
   
  // use ink::primitives::AccountId;

// use crate::psp34_lop_organization::psp34_lop_organization;
    use openbrush::{traits::Storage, contracts::psp34};

   
    #[ink(storage)]
    #[derive(Default, Storage, )]
    pub struct Contract {
    	#[storage_field]
		psp34: psp34::Data,
        
		next_id: u8,
    }
     // Implementa el trait psp34_lop_organization para el contrato Votantes
    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
			psp34::Internal::_mint_to(&mut _instance, Self::env().caller(), Id::U8(1)).expect("Can mint");
			_instance
        }

		#[ink(message)]
        pub fn mint_token_r(&mut self, Voter_de_openb: AccountId) -> Result<(), PSP34Error> {
            psp34::Internal::_mint_to(self, Self::env().caller(), Id::U8(self.next_id))?;
            self.next_id += 1;
            
            Ok(())
        }
        #[ink(message)]
        pub fn total_psp34_lop(& self)-> Result<u128, PSP34Error> {
        let a:u128 = psp34::BalancesManager::_total_supply( self);
        
            Ok(a)
        }
        
       
        
    }
    /* impl psp34_lop_organization for Contract  {
        
        #[ink(message)]
        fn mint_token(&mut self,Voter_de: AccountId) -> bool {
            let Voter_de_openb = Voter_de  as AccountId;
            match  self.mint_token_r( Voter_de_openb ){
                 Ok(_)=> {true}
                Err(_)=> {false}
                _=> {false}
            }
        } 
    } */
}