#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod error;


#[ink::contract]
mod tokenSend {
    use ink::env::emit_event;   
    use crate::error::Error;

    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use ink::{
        env::{
            call::{build_call, build_create, ExecutionInput, Selector},
            DefaultEnvironment,
        },
        selector_bytes,
    };
    //use Psp22Token1::Psp22Token1Ref;
    use psp22Token1::Psp22Token1Ref;
   /*  #[derive(Debug)]
    #[ink::storage_item]
     pub struct ctaToken{
        pub id_token: AccountId,
        pub balance: Balance
    } */
    
    // to add new static storage fields to your contract.
    //#[derive(scale::Encode, scale::Decode)]
    #[ink(storage)]
    pub struct TokenSend {
        User : AccountId,
       // Almacena el saldo de cada usuario para cada token
        pub balances_tokenID_Cantidad:  Vec<(AccountId, u128)>,
       
    }

    impl TokenSend {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                User: Self::env().caller(),
                balances_tokenID_Cantidad: Vec::new(),
            }

        }

       

          
        }
        #[ink(message)]
        pub fn flip_contract_callbuildeer_sin_gas(&mut self,token_contract_address: AccountId )-> () {
           // ,token_contract_address: AccountId
            use ink::env::call::{build_call, ExecutionInput, Selector};
             
            // const GET_ADDRESS_SELECTOR: [u8; 4] = ink::selector_bytes!("flip");

        let my_return_value = build_call::<Environment>()
        .call(token_contract_address)
        //.delegate(hash)
        .exec_input(
            ExecutionInput::new(Selector::new(ink::selector_bytes!("flip")))
               // .push_arg(1)
                
        )
        .returns::<()>()
        .params()
        .invoke();
       my_return_value
    }
        #[ink(message)]
        pub fn get_flip2_buill_call(&mut self, token_contract_address: AccountId, account_id: AccountId )-> i32{

            let my_return_value = build_call::<DefaultEnvironment>()
            .call(token_contract_address)
            .gas_limit(0)
            .transferred_value(0)
            
            .exec_input(
                ExecutionInput::new(Selector::new(ink::selector_bytes!("get")))
                   // .push_arg(1i32)
                    
            )
            .returns::<i32>()
            .invoke();
            //.invoke();
        my_return_value
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
            //let tokenSend = TokenSend::default();
           // assert_eq!(tokenSend.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
           // let mut tokenSend = TokenSend::new(false);
          //  assert_eq!(tokenSend.get(), false);
          //  tokenSend.flip();
           // assert_eq!(tokenSend.get(), true);
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
            let constructor = TokenSendRef::default();

            // When
            let contract_account_id = client
                .instantiate("tokenSend", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<TokenSendRef>(contract_account_id.clone())
                .call(|tokenSend| tokenSend.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = TokenSendRef::new(false);
            let contract_account_id = client
                .instantiate("tokenSend", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<TokenSendRef>(contract_account_id.clone())
                .call(|tokenSend| tokenSend.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<TokenSendRef>(contract_account_id.clone())
                .call(|tokenSend| tokenSend.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<TokenSendRef>(contract_account_id.clone())
                .call(|tokenSend| tokenSend.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
