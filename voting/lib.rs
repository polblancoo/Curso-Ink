#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[ink::contract]
mod Votantes {
    use ink::prelude::vec::Vec;
    use ink::env::*;
    
    //use ink::storage::Mapping;

    /// Defines the storage of your contract.
    #[ink(storage)]
    pub struct Votantes {

        admin: AccountId,
        voters: Vec<(AccountId, i8)>,
        //voters: Mapping<AccountId, Balance>,
    }
    //Tipo de voto
    #[derive(Debug, PartialEq, Eq, Clone, Copy, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "ink-generate-abi", derive(type_metadata::Metadata))]
    pub enum TipoVoto {
        Positive  = 1 ,
        Neutral = 0,
        Negative = 2,
    }
   
    #[ink(event)]
    pub struct Voted {
        voter: AccountId,
        value: TipoVoto,
    }


    impl Votantes {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            Self {
                admin,
                voters: Default::default(),
            }
        }
        
        #[ink(message)]
        pub fn ChangeAdmin(&mut self, voter: AccountId) -> bool {
            self.ensure_admin();
            if self.is_voter(&voter) {
                return false;
            }
            self.voters.push((voter, 0 ));
            true
        }
        
        #[ink(message)]
        pub fn AddVotante(&mut self, voter: AccountId) -> bool {
            self.ensure_admin();
            if self.is_voter(&voter) {
                return false;
            }
            self.voters.push((voter, TipoVoto::Neutral as i8));
            true
        }

        #[ink(message)]
        pub fn RemoveVoter(&mut self, voter: AccountId) -> bool {
            self.ensure_admin();
            if let Some(index) = self.voters.iter().position(|&(v, _)| v == voter) {
                self.voters.swap_remove(index);
                true
            } else {
                false
            }
        }
        #[ink(message)]
        pub fn vote(&mut self, value: TipoVoto) -> bool {
            let sender = self.env().caller();
            if !self.is_voter(&sender) {
                return false;
            }
            
            if let Some(index) = self.voters.iter().position(|&(v, _)| v == sender) {
                match value {
                    TipoVoto::Positive => self.voters[index].1 += 1,
                    TipoVoto::Neutral => {} // No change for neutral votes
                    TipoVoto::Negative => self.voters[index].1 -= 1,
                }

                self.env().emit_event(Voted {
                    voter: sender,
                    value,
                });

                true
            } else {
                false
            }
        }




        //Si es votante
        fn is_voter(&self, account: &AccountId) -> bool {
            self.voters.iter().any(|&(v, _)| v == *account)
        }
        //Si es admin
        fn ensure_admin(&self) {
            assert_eq!(self.env().caller(), self.admin, "Solo el User Admin puede editar la lista de votantes.");
        }
       
       
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
   
    }
}
