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
   // use crate::errors;
    use crate::errors::Error;
    use Voting::VotantesRef;
    use Voting::voting_organizacion::VotingOrganization;
 //   use psp34_lop::ContractRef;
    use psp34_bis::ContractbisRef;
    

    


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
      
       psp34_bis: ContractbisRef,
		
    }

    impl Master {
        /// Constructor that initializes Master / voting / psp34.
        #[ink(constructor)]
        pub fn new(admin_voters: AccountId,voting_contract_code_hash: Hash, psp34_contract_code_hash: Hash) -> Self {
            let caller = Self::env().caller();
            Self {
                voting_contract: VotantesRef::new(admin_voters)
                    .code_hash(voting_contract_code_hash)
                    .endowment(0)
                    .salt_bytes(Vec::new()) // Sequence of bytes
                    .instantiate(),
                   
               
                 psp34_bis: ContractbisRef::new()
                    .code_hash(psp34_contract_code_hash)
                    .endowment(0)
                    .salt_bytes(Vec::new()) // Sequence of bytes
                    .instantiate(),          
                
            }
        }
        /// Devuelve total de psp34 minteados
        #[ink(message)]
        pub fn total_mint_psp34_lop(&mut self)->u8{
            psp34_bis::ContractbisRef::total_nft(&mut self.psp34_bis).unwrap_or(0)
            
            
        }
        ///Votacion de caller a candidate
        #[ink(message)]
        pub fn vote_with_ref(&mut self, caller_id: AccountId, candidate: AccountId)-> Result<(), Error>  {
               //se emite un voto por vez.
                if candidate == caller_id {return Err( Error::CannotVoteItself)};
                match self.voting_contract.vote_trait(caller_id , candidate){
                   Ok(T) => {true},
                   Err(E)=>return Err( Error::NotVoterError)
                };
                //if a == false {return Err( Error::NotVoterError)}; 

               //se mintea al caller , quien voto 
              //psp34_lop::ContractRef::mint(caller_id, self.next_id+1)
           
              if let Err(err) =psp34_bis::ContractbisRef::mint_to(&mut self.psp34_bis , caller_id)  {
                Err( Error::NftNotSent)
    
            } else {
               
                               // let t = self.psp34_contract.total_psp34_lop( ).unwrap();
                    
                                //TODO (Emitir evento)
                               // let caller = self.env().caller();
                               // self.env().emit_event( NewVoter_Mint{
                               //     voter: caller.clone(),
                               //     count: t as i32
                               // })
                              // ;
                               Ok(())
                            }
                              
           // Ok(())
           
        }
        /// Obiene votos solo el dueño puede ver sus votos
        #[ink(message)]
        pub fn get_votes_with_ref(& self)-> i32 {
            let candidate = Self::env().caller();
            //se emite un voto por vez.
            self.voting_contract.get_votes(candidate)
        }
        ///Agrega votantes a la lista de aptos para votar a otros candidatos 
        #[ink(message)]
        pub fn add_voters_with_ref(&mut self, caller: AccountId, candidate: AccountId)-> Result<(), Error> {
           // let candidate = Self::env().caller();
            if !self.voting_contract.add_voter_trait( caller, candidate){
                
                    return Err(Error::VoterNotExist)
                  };
                 Ok(())
        }
        /// Elimina de la lista de votantes para votar
        #[ink(message)]
        pub fn remove_voter_with_ref(&mut self,caller: AccountId, candidate: AccountId) ->  Result<(), Error> {
                    
            if !self.voting_contract.remove_voter_trait (caller, candidate)
              {
                return Err(Error::NotAdmin)
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

  

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        use ink::primitives::AccountId;
        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

 /// Helper to get Bob's account_id from `ink_e2e::bob()` PairSigner
    
 fn get_alice_account_id() -> AccountId {

    let alice = ink_e2e::alice();
    let alice_id = AccountId::from(alice.public_key().0);
    alice_id

 }
 fn get_bob_account_id() -> AccountId {

    let bob = ink_e2e::bob();
    let bob_id = AccountId::from(bob.public_key().0);
    bob_id

 } 
 fn get_charlie_account_id() -> AccountId {

    let charlie = ink_e2e::charlie();
    let charlie_id = AccountId::from(charlie.public_key().0);
    charlie_id

 } 



        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Arrange
            
            let psp34_nft_Hash =client
            .upload("psp34_lop", &ink_e2e::alice(), None)
            .await
            .expect("nft contract upload failed")
            .code_hash; 
           
            
           
           let alice_id = get_alice_account_id();
            let votantes_Hash =client
            .upload("Voting", &ink_e2e::alice(), None)
            .await
            .expect("votantes contract upload failed")
            .code_hash; 
      
      
            let alice_id = get_alice_account_id();
            let constructor_Master= MasterRef::new(alice_id, 
                                                         votantes_Hash ,
                                                        psp34_nft_Hash);
            let contract_Master_id = client                                                        
            .instantiate("Organizator", &ink_e2e::alice(), constructor_Master, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;    
                
        
            //Agrega a bob y  charlie  a la lista de votantes
            let alice_id = get_alice_account_id();    
            let bob =  get_bob_account_id();
             let message = build_message::<MasterRef>(contract_Master_id.clone())
             .call (|MasterRef| MasterRef.add_voters_with_ref(alice_id , bob));  
            let get_result = client.call_dry_run(&ink_e2e::alice(), &message, 0, None).await;
            
            let alice_id = get_alice_account_id();
            let charlie =  get_charlie_account_id();
             let message = build_message::<MasterRef>(contract_Master_id.clone())
             .call (|MasterRef| MasterRef.add_voters_with_ref(alice_id ,charlie));  
            let get_result = client.call_dry_run(&ink_e2e::alice(), &message, 0, None).await;
            
            //bob vota por charlie
            let charlie =  get_charlie_account_id();
             let message = build_message::<MasterRef>(contract_Master_id.clone())
             .call (|MasterRef| MasterRef.vote_with_ref(charlie));  
            let get_result = client.call_dry_run(&ink_e2e::bob(), &message, 0, None).await;
                
            
            //Veo si charlie tiene votos
            let message = build_message::<MasterRef>(contract_Master_id.clone())
            .call (|MasterRef| MasterRef.get_votes_with_ref());  
           let get_result = client.call_dry_run(&ink_e2e::charlie(), &message, 0, None).await;
               let r = get_result as i32;
               assert_eq!(r , 1);

            Ok(())
        }

    }
}
