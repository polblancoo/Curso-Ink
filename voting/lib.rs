
#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[warn(unused_imports)]


#[ink::contract]
mod votantes {
    use ink::prelude::vec::Vec;
    use scale::{Decode, Encode};
  

    /// Defines the storage of your contract.
    #[ink(storage)]
    pub struct Votantes {

        admin: AccountId,
        voters: Vec<(AccountId, i8)>,
        
    }
    //Tipo de voto
    #[derive(Debug, Eq, PartialEq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
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
        pub fn change_admin(&mut self, voter: AccountId) -> bool {
            self.ensure_admin();
            if self.is_voter(&voter) {
                return false;
            }
            self.voters.push((voter, 0 ));
            true
        }
        
        #[ink(message)]
        pub fn add_voter(&mut self, voter: AccountId) -> bool {
            self.ensure_admin();
            if self.is_voter(&voter) {
                return false;
            }
            self.voters.push((voter, TipoVoto::Neutral as i8));
            true
        }

        #[ink(message)]
        pub fn remove_voter(&mut self, voter: AccountId) -> bool {
            self.ensure_admin();
            if let Some(index) = self.voters.iter().position(|&(v, _)| v == voter) {
                self.voters.swap_remove(index);
                true
            } else {
                false
            }
        }
        #[ink(message)]
        pub fn vote(&mut self, voter: AccountId, value: TipoVoto) -> bool {
            let sender = self.env().caller();
         
           if sender == voter {
             return false
           } 
            if self.is_voter(&voter) {
                if let Some(index) = self.voters.iter().position(|&(v, _)| v == voter) {
                    match value {
                        TipoVoto::Positive => self.voters[index].1 += 1,
                        TipoVoto::Neutral => {} // No change for neutral votes
                        TipoVoto::Negative => self.voters[index].1 -= 1,
                     }
        
                    self.env().emit_event(Voted {
                        voter,
                        value,
                    });
        
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
        





#[ink(message)]
        pub fn get_votes(&self, voter: AccountId) -> i8 {
            if let Some(index) = self.voters.iter().position(|&(v, _)| v == voter) {
                self.voters[index].1
            } else {
                0
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


    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`code 
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
   
   #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::{test::set_caller, DefaultEnvironment};

        
        
            #[ink::test]
             fn test_new()  {
                
                let admin = AccountId::from([u8::MAX; 32]);
                //inicializa el contrato 
                let contract = Votantes::new(admin);

                let user0 = AccountId::from([0; 32]);
                let user1 = AccountId::from([1; 32]);
                let user2 = AccountId::from([2; 32]);

                assert_eq!(contract.admin , admin);
                
              
            }
            #[ink::test]
            fn test_es_voter()  {
                let admin = AccountId::from([u8::MAX; 32]);
                //inicializa el contrato 
                let  contract = Votantes::new(admin);
                let user0 = AccountId::from([0; 32]);
                
                assert!(!contract.is_voter( &user0));
                
            }
            #[ink::test]
            fn test_configurar_votante_y_votar()  {
                let admin = AccountId::from([u8::MAX; 32]);
                //inicializa el contrato 
                let mut contract = Votantes::new(admin);
                let user0 = AccountId::from([0; 32]);
                let user1 = AccountId::from([0; 32]);
                let v = TipoVoto::Positive;
                //configuro admin y lo agrego a la lista de votantes
                set_caller::<DefaultEnvironment>(admin);
                assert!( contract.add_voter(user1));

                //configuro el caller y voto
               /* 
                set_caller::<DefaultEnvironment>(user1);
                
             */
            assert!( contract.vote(user0 , v));
            }

        
       

    }//fin mod test
    
}
