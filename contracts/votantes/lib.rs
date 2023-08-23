#![cfg_attr(not(feature = "std"), no_std,no_main)]
pub mod voting_organizacion;

pub use self::Voting::VotantesRef;
 

 #[ink::contract]
 mod Voting {
    
   use crate::voting_organizacion::{VotingOrganization, ContractError} ;
    use ink::storage::Mapping;
    
    //Eventos
    #[ink(event)]
    pub struct NewVoter {
        #[ink(topic)]
        voter: AccountId,
    }
    #[ink(event)]
    pub struct Voto {
        #[ink(topic)]
        voto: AccountId, //de
        #[ink(topic)]
        voto_a:AccountId, //a
        votos: i32,
    }

   
    #[derive(Debug)]
    #[ink::storage_item]
    pub struct Admin {
        address: AccountId,
        modified_date: u64,
    }

    

    #[ink(storage)]
    pub struct Votantes {
        //admin para 
        admin: Admin,
        //guarda votos
        voters: Mapping<AccountId, i32>,
        //agrega votantes x el admin
        enabled_voter: Mapping<AccountId, ()>,
        //total de votos
        total_votos: i32,
    }

    impl Votantes {
        #[ink(constructor)]
        pub fn new(admin_init: AccountId) -> Self {
            let now = Self::env().block_timestamp();
           Self{
               admin:  Admin {
                    address: admin_init,
                    modified_date: now,
                },
                voters: Mapping::default(),
                enabled_voter:Mapping::default(),
                total_votos:0,
           }
        }

        #[ink(message)]
        pub fn change_admin(&mut self, new_admin: AccountId) -> Result<bool, ContractError> {
            let now = Self::env().block_timestamp();
            match self.ensure_admin() {
                Ok(true) => {
                   
                        self.admin= Admin {
                            address: new_admin,
                            modified_date: now,
                        };
                        Ok(true)
                     
                },
                Ok(false) => Ok(false),
                Err(_) => Err(ContractError::NoAdmin),
            }
        }
        
        
        

        #[ink(message)]
        pub fn add_voter(&mut self, voter: AccountId) ->  Result<bool, ContractError> {
            self.ensure_admin()?;
           // if self.is_voter(&voter) {
           //     return Err(ContractError::YouAreNotVoter);
           // }
            //self.voters.insert(voter, &0);
            self.enabled_voter.insert(voter, &());
            //emite evento.
            self.env().emit_event(NewVoter { voter });
            //devuelve OK(bool o Err ), por el Result
            Ok(true)
        }

        #[ink(message)]
        pub fn remove_voter(&mut self, voter: AccountId) ->  Result<bool, ContractError>  {
           self.ensure_admin()?;
           if self.enabled_voter.take(&voter).is_some(){
               //Habria que eliminar de new_voters mapping tambien o no?
                Ok(true)
           }else{
                Err(ContractError::YouAreNotVoter)
           }
        }

        #[ink(message)]
        pub fn vote(&mut self, voter: AccountId, value: i32) -> bool {
           
            let sender = self.env().caller();
            if sender == voter || value < 1 {
                return false;
            }
            
            if self.is_voter(&voter) {
                //println!("es votante :{:?}", self.is_voter(&voter));
                let sender_votes = self.voters.get(&sender).unwrap_or(1);
               // println!("Cantidad de votos antes de Booster :{:?}", sender_votes);
                let booster = self.calculate_booster(sender_votes);
                //println!(" Booster :{:?}", booster);
                let added_votes = value * booster;
               
                // Verificar que el votante tenga suficientes votos para votar con el booster
                
                  //  println!(" votos totales con Booster :{:?}", added_votes);
                    
                    self.voters.insert(voter, &added_votes);
        
                    // Sumar los votos al total_votos
                    self.total_votos += added_votes;
        
                    // Emitir evento de voto
                    self.env().emit_event(Voto {
                        voto: sender,
                        voto_a: voter,
                        votos: added_votes,
                    });
        
                    true
               
            } else {
                false
            }
        
           

        }

        fn calculate_booster(&self, sender_votes: i32) -> i32 {
            let total_votes = self.total_votos;
            if total_votes == 0 {
                1
            } else {
                let booster = (sender_votes  / total_votes)  * 100;
                match booster {
                    _ if booster <= 25 => 2,
                    _ if booster <= 50 => 3,
                    _ if booster <= 75 => 4,
                    _ => 5,
                }
            }
        }
        
        #[ink(message)]
        pub fn get_votes(&self, voter: AccountId) -> i32 {
            self.voters.get(voter).unwrap_or(0)
            
        }
        #[ink(message)]
        pub fn get_total_votos(&self) -> i32 {
            self.total_votos
            
        }

        fn is_voter(&self, account: &AccountId) -> bool {
             self.enabled_voter.contains(account)
              // self.voters.contains(&voter)
          
        }

        fn ensure_admin(&self) -> Result<bool, ContractError> {
            if self.env().caller() == self.admin.address {
                Ok(true)
            } else {
                Err(ContractError::NoAdmin)
            }
        }
        
   
       
    }
    // Importa el trait VotingOrganization desde el archivo externo
    
     //**********************
      // Implementa el trait VotingOrganization para el contrato Votantes
      impl VotingOrganization for Votantes {
       
        #[ink(message)]
        fn vote(&mut self, voter_to: AccountId, value: i32) -> bool {
              // Implementa la lógica de votación específica del contrato aquí
             self.vote( voter_to , value)
          }
          #[ink(message)]
          fn get_votes(&self, voter: AccountId) -> i32{
           // let from = Self::env().caller();
            self.get_votes( voter)
           }
           #[ink(message)]
            fn add_voter(&mut self, voter: AccountId)-> bool {
             match self.add_voter(voter){
                 Ok(true) => {true },
                 Ok(false) => {false},
                 Err(_) =>{false}
             }
           }
          
           #[ink(message)]
           fn remove_voter(&mut self, voter: AccountId) ->bool{
            match self.remove_voter(voter){
                Ok(true) => {true },
                Ok(false) => {false},
                Err(_) =>{false}
            }
           }


      }
       //********************** 

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
            let result = context.contract.change_admin(context.bob);
                 //println!("Admin {:?} ---{:?}", context.contract.admin.address, context.bob);
                 assert_eq!(result, Ok(true));
                 assert_eq!(context.contract.admin.address, context.bob);
            //preguntamos si bob es el admin
            assert_eq!(context.contract.admin.address, context.bob);

              //una vez mas de bob --- a alice pasa el admin
            set_caller::<DefaultEnvironment>(context.bob);
           // assert!(context.contract.change_admin(context.alice));
            let result = context.contract.change_admin(context.alice);
                 assert_eq!(result, Ok(true));
            //preguntamos si alice es el admin
            assert_eq!(context.contract.admin.address, context.alice);
        
        } 
        #[ink::test]
        fn test_remove_voter() {
            let mut context = Context::new();
    
            // Verifica que un votante existente pueda ser eliminado
            set_caller::<DefaultEnvironment>(context.admin);
            context.contract.add_voter(context.alice);
            context.contract.add_voter(context.bob);
           
            //assert!(context.contract.remove_voter(context.alice));
            let result = context.contract.remove_voter(context.bob);
                 assert_eq!(result, Ok(true));
            //assert!(!context.contract.is_voter( &context.alice));
            let result = context.contract.remove_voter(context.alice);
             assert_eq!(result, Ok(true));
             assert_eq!(context.contract.is_voter(&context.alice), false);
             
            
            //sete un Account no autorizado como admin e intenta borrar otro Account 
          //  set_caller::<DefaultEnvironment>(context.bob);
          
        }
        #[ink::test]
        fn test_vote() {
            let mut context = Context::new();
            //se agregan a la lista de los q pueden votar
            set_caller::<DefaultEnvironment>(context.admin);
            //print!("caller :{:?}", context.admin);
            //enabled_voter
            let add0=context.contract.add_voter(context.alice);
            assert!(add0.unwrap() );
            let add1= context.contract.add_voter(context.bob);
            assert!(add1.unwrap()  );
            
            // Verifica que un votante no pueda votar por sí mismo
            set_caller::<DefaultEnvironment>(context.bob);
           // println!("{:?}", context.contract.is_voter(&context.alice));
            assert!(context.contract.vote(context.alice, 50));
            assert_eq!(context.contract.get_votes(context.alice), 50);
            // Verifica que un votante no pueda votar por sí mismo
            set_caller::<DefaultEnvironment>(context.bob);
            assert!(!context.contract.vote(context.bob, 80));
            assert_ne!(context.contract.get_votes(context.bob), 90);
          
          //intenta votar mas de 100 puntos ppt
          
          set_caller::<DefaultEnvironment>(context.bob);
          assert!(context.contract.vote(context.alice, 300));
         

           
            
    }

        

   


}
}
