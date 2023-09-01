#![cfg_attr(not(feature = "std"), no_std, no_main)]




#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]
#[ink::contract]
mod organization {
    
    //use openbrush::{contracts::psp34::*};

    use ink::prelude::vec::Vec;
    use ink::{
        env::{
            call::{build_call, build_create, ExecutionInput, Selector},
            DefaultEnvironment,
        },
        selector_bytes,
    };

     
    //use voting::VotantesgRef;
    use Voting::VotantesRef;
    use Voting::voting_organizacion::VotingOrganization;
    
    //Reference to psp34_lop
    use psp34_lop::ContractRef;
    
    //Eventos
    #[ink(event)]
    pub struct NewVoter_Mint {
        #[ink(topic)]
          voter: AccountId,
        
    }
   
    #[ink(storage)]
    pub struct Organization {
        voting_contract: VotantesRef,
        psp34_contract: ContractRef
    }

    impl Organization {

        

        #[ink(constructor)]
        pub fn new_with_ref(admin_voters: AccountId,voting_contract_code_hash: Hash, psp34_contract_code_hash: Hash) -> Self {
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
                Ok((t))=>{t},
                Err(_)=>{0}
            }  
        }
        
        #[ink(message)]
        pub fn vote_with_ref(&mut self, candidate: AccountId)-> bool {
            //se emite un voto por vez.
            let caller = self.env().caller();
            if self.voting_contract.vote_trait(caller , candidate){
               //se mintea al caller , quien voto 
                let r: Result<(), _> = self.psp34_contract.mint_token_r( caller);
                let t = self.psp34_contract.total_psp34_lop( ).unwrap();
                 match r{
                    Ok(())=>{
                      
                    //emite evento
                   // self.env().emit_event(Transfer { from, to, id });
                   //self.env().emit_event( NewVoter_Mint{  caller});
                        true
                    
                    }
                    _ => {false}
                }
                
                                
            }else{
                false
            }
        }

        #[ink(message)]
        pub fn get_votes_with_ref(& self)-> i32 {
            let candidate = Self::env().caller();
            //se emite un voto por vez.
            self.voting_contract.get_votes(candidate)
        }
        
        #[ink(message)]
        pub fn add_voters_with_ref(&mut self, caller: AccountId, candidate: AccountId)-> bool {
           // let candidate = Self::env().caller();
            match self.voting_contract.add_voter_trait( caller, candidate){
                true  => {true}
                err=> {false }
                _ => {false}
            }
        }
        #[ink(message)]
        pub fn remove_voter_with_ref(&mut self,caller: AccountId, candidate: AccountId) ->  bool {
            // let candidate = Self::env().caller();
            //self.voting_contract.remove_voter_trait(candidate);
             match self.voting_contract.remove_voter_trait (caller, candidate){
                 true => {
                     //emitir evento
                   //  todo!();
                     true
                 }
                 
                 _ => {
                     //emitir evento
                   //  todo!();
                     false
                 }
             }
         }
         #[ink(message)]
        pub fn change_admin_with_ref (&mut self, new_admin: AccountId) ->bool{
          
            match self.voting_contract.change_admin ( new_admin){
                Ok(true) => {
                    //emitir evento
                   // todo!();
                    true
                }
                Err(err)=> {
                   //emitir evento
                 //  todo!();
                   false

               }
                _ => {
                    //emitir evento
                  //  todo!();
                    false
                }
            }
        }

//**************************************** */
      /*   #[ink(message)]
        pub fn vote_with_builder(
            &mut self,
            candidate: AccountId,
            voting_contract_address: AccountId,
        ) {
            build_call::<DefaultEnvironment>()
                .call(voting_contract_address)
                .gas_limit(0)
                .exec_input(
                    ExecutionInput::new(Selector::new(selector_bytes!("vote")))
                        .push_arg::<&[u8; 32]>(candidate.as_ref()),
                )
                .returns::<()>()
                .invoke();
        } */
/* 
     */
    }
    
  //********************************** */
    
}
