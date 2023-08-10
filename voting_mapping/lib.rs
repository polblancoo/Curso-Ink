#![cfg_attr(not(feature = "std"), no_std)]



#[ink::contract]
 mod votantes {
    
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Votantes {

        admin: AccountId,
        voters: Mapping<AccountId, Balance>,
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
        pub fn vote(&mut self, voter: AccountId, value: Balance) -> bool {
            let sender = self.env().caller();

            if sender == voter || value < 1 as Balance || value > 100 as Balance {
                return false;
            }

            if self.is_voter(&voter) {
                let current_votes = self.voters.get(&voter);
                if current_votes.unwrap() + value as u128 <= 100 as u128 {
                    let tot=  current_votes.unwrap()  + value as u128;
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
        pub fn get_votes(&self, voter: AccountId) -> Balance {
            self.voters.get(voter).unwrap_or(0)
            
        }

        fn is_voter(&self, account: &AccountId) -> bool {
            let t= self.voters.get(&account).unwrap_or(0);
           if t== 0 {false}else {true}
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
        charlie: AccountId,
    }

    /****************************/
    
    impl Context {
        
        pub fn new() -> Self {
            let admin = AccountId::from([u8::MAX; 32]);
            //inicializa el contrato
            let contract = Votantes::new(admin);

            let alice = AccountId::from([1; 32]);
            let bob = AccountId::from([2; 32]);
            let charlie = AccountId::from([3; 32]);

           
            // Restaura el caller a su valor por defecto
            set_caller::<DefaultEnvironment>(admin.clone());

            Self {
                contract,
                admin,
                alice,
                bob,
                charlie,
            }
        }
       
    }
        #[ink::test]
         fn test_change_admin() {
            let mut context = Context::new();
            set_caller::<DefaultEnvironment>(context.admin.clone());
            assert!(context.contract.change_admin(context.bob.clone() ));
            //preguntamos si bob es el admin
           // assert_eq!(context.contract.admin(), context.bob);
            //una vez mas de bob --- a alice pasa el admin
            set_caller::<DefaultEnvironment>(context.bob.clone());
            assert!(!context.contract.change_admin(context.alice.clone()));
            
         // Configura el admin y votantes en el contrato
         set_caller::<DefaultEnvironment>(context.admin.clone());
         assert!(context.contract.add_voter(context.alice.clone()));
         assert!(context.contract.add_voter(context.bob.clone()));
         assert!(context.contract.add_voter(context.charlie.clone()));
       
        } 
        
        #[ink::test]
         fn testffff() {


         }

   


}
}