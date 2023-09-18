#![cfg_attr(not(feature = "std"), no_std, no_main)]

//pub use crate::psp34_lop::ContractRef;

pub use self::psp34_bis::ContractbisRef;


#[openbrush::implementation(PSP34, PSP34Mintable)]
#[openbrush::contract]
pub mod psp34_bis {
   
  // use ink::primitives::AccountId;

// use crate::psp34_lop_organization::psp34_lop_organization;
    use openbrush::{traits::Storage, contracts::psp34,  };

   
    #[ink(storage)]
    #[derive(Default, Storage, )]
    pub struct Contractbis {
    	#[storage_field]
		psp34: psp34::Data,
        
        #[storage_field]
		next_id: u8,
    }
     // Implementa el trait psp34_lop_organization para el contrato Votantes
    impl Contractbis {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
			psp34::Internal::_mint_to(&mut _instance, Self::env().caller(), Id::U8(1)).expect("Can mint");
			_instance
        }


        ///Busca el ultimo nft minteado y suma uno para crear uno nuevo
        #[ink(message)]
        pub fn mint_to(&mut self, to: AccountId) -> Result<(), PSP34Error> {
            PSP34Mintable::mint(self, to, Id::U8(self.next_id+2))?;
            self.next_id += 2;
            Ok(())
        }
		
         ///total minteados
         #[ink(message)]
         pub fn total_nft(&self) -> Result<u8, PSP34Error> {
                          
             Ok(self.next_id)
         }
        
        
       
        
    }
     

    
}

   