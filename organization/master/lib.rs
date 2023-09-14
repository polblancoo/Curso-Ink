#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]
mod  errors;

#[ink::contract]
mod master {

    use ink::env::emit_event;
    use ink::prelude::vec::Vec;
    use ink::{
        env::{
            call::{build_call, build_create, ExecutionInput, Selector},
            DefaultEnvironment,
        },
        selector_bytes,
    };

    use crate::errors::Error;
    use Voting::VotantesRef;
    use Voting::voting_organizacion::VotingOrganization;
    use psp34_lop::ContractRef;

    use crate::errors;
    //Eventos
    #[ink(event)]
    pub struct NewVoter_Mint {
        #[ink(topic)]
        voter: AccountId,
        count: i32,
        
    }
    
    /// Defines the storage of your contract.
     #[ink(storage)]
    pub struct Master {
        /// Ref. a los contratos votacion y nft.
        voting_contract: VotantesRef,
        psp34_contract: ContractRef
    }

    impl Master {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(admin_voters: AccountId,voting_contract_code_hash: Hash, psp34_contract_code_hash: Hash) -> Self {
            let caller = Self::env().caller();
            Self {
                voting_contract: VotantesRef::new(admin_voters)
                    .code_hash(voting_contract_code_hash)
                    .endowment(0)
                    .salt_bytes(Vec::new()) // Sequence of bytes
                    .instantiate(),
                   
                psp34_contract: ContractRef::new()
                    .code_hash(psp34_contract_code_hash)
                    .endowment(0)
                    .salt_bytes(Vec::new()) // Sequence of bytes
                    .instantiate(),    
            }
        }
        #[ink(message)]
        pub fn total_mint_psp34_lop(&mut self)->u128{
            let t = self.psp34_contract.total_psp34_lop( );
            match t{
                Ok(t)=>{t},
                Err(_)=>{0}
            }  
        }
        #[ink(message)]
        pub fn vote_with_ref(&mut self, candidate: AccountId)-> Result<(), Error>  {
            //se emite un voto por vez.
            let caller = self.env().caller();
            if self.voting_contract.vote_trait(caller.clone() , candidate.clone()){
               //se mintea al caller , quien voto 
              match  self.psp34_contract.mint_token_r( caller){
                Ok(())=>{
                    let t = self.psp34_contract.total_psp34_lop( ).unwrap();
                    //TODO (Emitir evento)
                   // let caller = self.env().caller();
                   // self.env().emit_event( NewVoter_Mint{
                   //     voter: caller.clone(),
                   //     count: t as i32
                   // })
                  // ;
                   Ok(())
                }
                Err(err)=> {
                    Err( Error::NotAdmin)
                    
                }

              }
                               
                
            }else{
                Err( Error::YouAreNotVoter)
            } 
                
        }
        #[ink(message)]
        pub fn get_votes_with_ref(& self)-> i32 {
            let candidate = Self::env().caller();
            //se emite un voto por vez.
            self.voting_contract.get_votes(candidate)
        }

        #[ink(message)]
        pub fn add_voters_with_ref(&mut self, caller: AccountId, candidate: AccountId)-> Result<(), Error> {
           // let candidate = Self::env().caller();
            if !self.voting_contract.add_voter_trait( caller, candidate){
                
                    return Err(errors::Error::VoterNotExist)
                  };
                 Ok(())
        }
        #[ink(message)]
        pub fn remove_voter_with_ref(&mut self,caller: AccountId, candidate: AccountId) ->  Result<(), Error> {
                    
            if !self.voting_contract.remove_voter_trait (caller, candidate)
              {
                return Err(errors::Error::NotAdmin)
              };
             Ok(())
            
        }
         #[ink(message)]
        pub fn change_admin_with_ref (&mut self, new_admin: AccountId) ->Result<(), Error>{
          
            match self.voting_contract.change_admin ( new_admin){
               
                Err(err)=> {
                   Err( Error::NotAdmin)
                   
               }
                _ => {
                    
                     Err( Error::NotAdmin)
                }
            }
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
            let constructor = MasterRef::default();

            // When
            let contract_account_id = client
                .instantiate("master", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<MasterRef>(contract_account_id.clone())
                .call(|master| master.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = MasterRef::new(false);
            let contract_account_id = client
                .instantiate("master", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<MasterRef>(contract_account_id.clone())
                .call(|master| master.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<MasterRef>(contract_account_id.clone())
                .call(|master| master.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<MasterRef>(contract_account_id.clone())
                .call(|master| master.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
