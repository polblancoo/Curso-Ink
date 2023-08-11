#![cfg_attr(not(feature = "std"), no_std)]
#[warn(non_snake_case)]


#[ink::contract]
 mod votantes {
    
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Votantes {

        admin: AccountId,
        voters: Mapping<AccountId, i16>,
    }

    impl Votantes {
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            Self {
                admin,
                voters: Mapping::new(),
            }
        }

        #[ink(message)]
        pub fn change_admin(&mut self, new_admin: AccountId) -> bool {
            self.ensure_admin();
            self.admin = new_admin;
            true
        }

        #[ink(message)]
        pub fn add_voter(&mut self, voter: AccountId) -> bool {
            self.ensure_admin();
            if self.is_voter(&voter) {
                return false;
            }
            self.voters.insert(voter, &0);
            true
        }

        #[ink(message)]
        pub fn remove_voter(&mut self, voter: AccountId) -> bool {
            self.ensure_admin();
            self.voters.take(&voter).is_some()
        }

        #[ink(message)]
        pub fn vote(&mut self, voter: AccountId, value: i16) -> bool {
           // println!("{:?}", self.is_voter(&voter));
            let sender = self.env().caller();
           
            if sender == voter || value < 1 || value > 100 {
                return false;
            }
            //println!("votante existe : {:?}", self.is_voter(&voter));
            if self.is_voter(&voter) {
               
              
               // println!("Voter: {:?} y valor: {:?}, sender es: {:?}", voter, value, sender);
                let current_votes = self.voters.get(&voter);
                
                if current_votes.unwrap()  + value  <= 100  {
                    let tot=  current_votes.unwrap()  + value ;
                    self.voters.insert(voter,&tot );
                    true
                } else {
                    false
                }
            } else {
                false 
            }
        }

        #[ink(message)]
        pub fn get_votes(&self, voter: AccountId) -> i16 {
            self.voters.get(voter).unwrap_or(0)
            
        }

        fn is_voter(&self, account: &AccountId) -> bool {
             self.voters.contains(account)
              // self.voters.contains(&voter)
          
        }

        fn ensure_admin(&self) {
            assert_eq!(self.env().caller(), self.admin, "Solo el User Admin puede editar la lista de votantes.");
        
        
        }
    }

// Pruebas unitarias
#[cfg(test)]

mod tests {
    
    use super::*;
    //use votantes::Votantes;
    use ink::env::{test::set_caller, DefaultEnvironment};
   // use ink_storage_traits::*;
    
    pub struct Context {
        contract: Votantes,
        admin: AccountId,
        alice: AccountId,
        bob: AccountId,
        
    }

    /****************************/
    
    impl Context {
        
        pub fn new() -> Self {
            let admin = AccountId::from([u8::MAX; 32]);
            //inicializa el contrato
            let contract = Votantes::new(admin);

            let alice = AccountId::from([1; 32]);
            let bob = AccountId::from([2; 32]);
            //let charlie = AccountId::from([3; 32]);

           
            // Restaura el caller a su valor por defecto
            set_caller::<DefaultEnvironment>(admin);

            Self {
                contract,
                admin,
                alice,
                bob,
                
            }
        }
       
    }
        #[ink::test]
         fn test_change_admin() {
            let mut context = Context::new();
            set_caller::<DefaultEnvironment>(context.admin);
            assert!(context.contract.change_admin(context.bob));
            //preguntamos si bob es el admin
            assert_eq!(context.contract.admin, context.bob);
              //una vez mas de bob --- a alice pasa el admin
            set_caller::<DefaultEnvironment>(context.bob);
            assert!(context.contract.change_admin(context.alice));
            //preguntamos si alice es el admin
            assert_eq!(context.contract.admin, context.alice);
        
        } 
        #[ink::test]
        fn test_remove_voter() {
            let mut context = Context::new();
    
            // Verifica que un votante existente pueda ser eliminado
            set_caller::<DefaultEnvironment>(context.admin);
            context.contract.add_voter(context.alice);
            context.contract.add_voter(context.bob);
            assert!(context.contract.remove_voter(context.alice));
            assert!(!context.contract.is_voter( &context.alice));
    
            // Intenta eliminar un votante inexistente y verifica que falle
            assert!(!context.contract.remove_voter(context.alice));

            //sete un Account no autorizado como admin e intenta borrar otro Account 
          //  set_caller::<DefaultEnvironment>(context.bob);
          
        }
        #[ink::test]
        fn test_vote() {
            let mut context = Context::new();
            //se agregan a la lista de los q pueden votar
            set_caller::<DefaultEnvironment>(context.admin);
            context.contract.add_voter(context.alice);
            context.contract.add_voter(context.bob);
            

            // Verifica que un votante no pueda votar por sí mismo
            set_caller::<DefaultEnvironment>(context.bob);
            assert!(context.contract.vote(context.alice, 50));
            assert_eq!(context.contract.get_votes(context.alice), 50);
            // Verifica que un votante no pueda votar por sí mismo
            set_caller::<DefaultEnvironment>(context.bob);
            assert!(!context.contract.vote(context.bob, 80));
            assert_ne!(context.contract.get_votes(context.bob), 90);
          
          //intenta votar mas de 100 puntos ppt
          // Verifica que un votante no pueda votar por sí mismo
          set_caller::<DefaultEnvironment>(context.bob);
          assert!(!context.contract.vote(context.alice, 150));
         //assert_eq!(context.contract.get_votes(context.alice), 150);

           
            
    }

        

   


}
}