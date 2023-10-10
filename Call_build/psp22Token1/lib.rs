#![cfg_attr(not(feature = "std"), no_std, no_main)]
//pub use self::psp22Token1::ContractRef;
pub use self::psp22Token1::Psp22Token1Ref;
//pub use self::Psp22Token1Ref;


#[openbrush::implementation(PSP22, PSP22Burnable,PSP22Metadata)]
#[openbrush::contract]
mod psp22Token1 {

    use openbrush::{traits::Storage, contracts::psp22,  };
    
    #[derive(Default, Storage, )]
    #[ink(storage)]
    pub struct Psp22Token1 {
        #[storage_field]
         psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl Psp22Token1 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(initial_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut _instance = Self::default();
            psp22::Internal::_mint_to(&mut _instance, Self::env().caller(), initial_supply).expect("Should mint"); 
            _instance.metadata.name.set(&name);
            _instance.metadata.symbol.set(&symbol);
            _instance.metadata.decimals.set(&decimal);
            _instance

        }

       //Total de tokens prices
       #[ink(message)] 
       pub fn totalsupplypricesByCount(&self, contract: AccountId) -> Balance {
               
       psp22::InternalImpl::_balance_of (self, &contract)
       
       
      }
       // transferencia
       #[ink(message)] 
       pub fn transferTo(&mut self,  from: AccountId,  to: AccountId, amount: Balance, _data: Vec<u8>, ) -> Result<(), PSP22Error>{
         psp22::PSP22Impl::allowance(self,from, to);
         //transfiero
         psp22::PSP22Impl::transfer( self ,to, amount, _data)?;
     
        
   
      Ok(()) 
      }

        //Total de tokens prices
        #[ink(message)] 
         pub fn totalsupplyToken(&self) -> Balance {
                 
            psp22::PSP22Impl::total_supply(self)
         
         
        }

       
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let psp22Token1 = Psp22Token1::default();
            //assert_eq!(psp22Token1.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
           // let mut psp22Token1 = Psp22Token1::new(false);
           // assert_eq!(psp22Token1.get(), false);
           // psp22Token1.flip();
           // assert_eq!(psp22Token1.get(), true);
        }
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = Psp22Token1Ref::default();

            // When
            let contract_account_id = client
                .instantiate("psp22Token1", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<Psp22Token1Ref>(contract_account_id.clone())
                .call(|psp22Token1| psp22Token1.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = Psp22Token1Ref::new(false);
            let contract_account_id = client
                .instantiate("psp22Token1", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<Psp22Token1Ref>(contract_account_id.clone())
                .call(|psp22Token1| psp22Token1.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<Psp22Token1Ref>(contract_account_id.clone())
                .call(|psp22Token1| psp22Token1.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<Psp22Token1Ref>(contract_account_id.clone())
                .call(|psp22Token1| psp22Token1.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
