
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]
#[ink::contract]
mod organization {
    use ink::prelude::vec::Vec;
    //use ink::primitives::AccountId;
    use ink::{env::{call::{build_call, build_create, ExecutionInput, Selector},
            DefaultEnvironment, },
        selector_bytes, };
   
    use Voting::VotantesRef;
       
    #[ink(storage)]
    pub struct Organization {
        voting_contract: VotantesRef,
    }
   impl Organization {

    #[ink(constructor)]
        pub fn new_with_ref( caller_admin_Votos: AccountId , voting_contract_code_hash: Hash) -> Self {
           
            Self {
                voting_contract: Voting::VotantesRef::new( caller_admin_Votos)
                    .code_hash(voting_contract_code_hash)
                    .endowment(0)
                    .salt_bytes(Vec::new()) // Sequence of bytes
                    .instantiate(),
            }
        }
        #[ink(message)]
        pub fn vote_with_ref(&mut self, candidate: AccountId, voto: i32) {
            self.voting_contract.vote(candidate, voto);
        }


        
    }
}

