#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub mod psp34_lop_organization;

//           nombreModulo::NombreStruct+Ref  
pub use self::psp34_lop::ContractRef;
//pub use self::psp34_lop::Contract;


#[openbrush::implementation(PSP34, PSP34Mintable)]
#[openbrush::contract]
pub mod psp34_lop {

    use crate::psp34_lop_organization::psp34_lop_organization;
    use openbrush::traits::Storage;

     
    #[ink(storage)]
    #[derive(Default, Storage, )]
    //#[derive(scale_info::TypeInfo)] 
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
        pub fn mint_token(&mut self) -> Result<(), PSP34Error> {
            psp34::Internal::_mint_to(self, Self::env().caller(), Id::U8(self.next_id))?;
            self.next_id += 1;
            Ok(())
        }
    }
    impl psp34_lop_organization for Contract  {
        
        #[ink(message)]
        fn mint_token(&mut self) -> bool {
            true
        } 
    }
  
    
    
     
}