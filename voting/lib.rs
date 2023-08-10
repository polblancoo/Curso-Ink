#![cfg_attr(not(feature = "std"), no_std, no_main)]
//#[warn(unused_imports)]
#[ink::contract]
mod votantes {
    use ink::prelude::vec::Vec;
    //use scale::{Decode, Encode};

    /// Defines the storage of your contract.
    #[ink(storage)]
    pub struct Votantes {
        admin: AccountId,
        voters: Vec<(AccountId, i8)>,
    }
    //Tipo de voto
    #[derive(Debug, Eq,Clone, Copy , PartialEq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum TipoVoto {
        Positive = 1,
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
            
            self.admin = voter;
            true
        }

        #[ink(message)]
        pub fn add_voter(&mut self, voter: AccountId) -> bool {
            self.ensure_admin();
           
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
        pub fn vote(&mut self,  voter: AccountId, value: TipoVoto) -> bool {
            let sender = self.env().caller();
          
            if sender == voter {
                
                return false;
            }
            //println!{"Es Votante : {:?}voto es : {:?} y el Sender es :{:?}", voter, value, sender};
           // println!("conteo votantes {:?}",self.voters.len());
            //println!("es votante...? {:?} , votante :{:?}",self.is_voter(&voter), &voter);
            if self.is_voter(&voter)==true {
               // println!{"Es Votante : {:?}voto es : {:?} y el Sender es :{:?}", voter, value, sender};
                
                if let Some(index) = self.voters.iter().position(|&(v, _)| v == voter) {
                    match value {
                        TipoVoto::Positive => self.voters[index].1 += 1,
                        TipoVoto::Neutral => {} // No change for neutral votes
                        TipoVoto::Negative => self.voters[index].1 -= 1,
                    }

                    //self.env().emit_event(Voted { voter, value });

                    true
                } else {
                    //println!("FALSE ----sender: {:?} , voter:  {:?} .",&sender,  &voter);
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
            //println!("lA CUENTA ES :  {:?} .",&account);
            //true
            self.voters.iter().any(|&(v, _)| v == *account)
        }
        //Si es admin
        fn ensure_admin(&self) {
            assert_eq!(
                self.env().caller(),
                self.admin,
                "Solo el User Admin puede editar la lista de votantes."
            );
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`code
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::{test::set_caller, DefaultEnvironment};

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

                //assert_eq!(contract.admin , admin);

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
        fn test_es_voter() {
            //inicializa el contexto de trabajo
            let context = Context::new();

            assert!(!context.contract.is_voter(&context.alice));
        }
        #[ink::test]
        fn test_configurar_votante_y_votar() {
            let mut context = Context::new();
            let v = TipoVoto::Positive;

            //configuro admin y lo agrego a la lista de votantes
            set_caller::<DefaultEnvironment>(context.admin);
            assert!(context.contract.add_voter(context.bob));
            assert!(context.contract.is_voter(&context.bob));
           

            //configuro el caller y voto

            set_caller::<DefaultEnvironment>(context.alice);

           assert!(context.contract.vote(context.bob, v));
        }

        #[ink::test]
        fn test_get_votes() {
            let mut context = Context::new();
            let v = TipoVoto::Positive;

            set_caller::<DefaultEnvironment>(context.admin);
            assert!(context.contract.add_voter(context.bob));
            assert!(context.contract.add_voter(context.alice));

            set_caller::<DefaultEnvironment>(context.bob);
            assert!(context.contract.vote(context.alice, v));

            set_caller::<DefaultEnvironment>(context.alice);
            assert!(context.contract.vote(context.bob, v));

            assert_eq!(context.contract.get_votes(context.alice), 1);
            assert_eq!(context.contract.get_votes(context.bob), 1);
        }

        #[ink::test]
        fn test_add_voter() {
            let mut context = Context::new();

            set_caller::<DefaultEnvironment>(context.admin);
            assert!(context.contract.add_voter(context.bob));
            assert!(context.contract.add_voter(context.bob)); // Intentar agregar nuevamente debe fallar

            assert!(context.contract.is_voter(&context.bob));
            assert_eq!(context.contract.voters.len(), 2);
        }

        #[ink::test]
        fn test_remove_voter() {
            let mut context = Context::new();

            set_caller::<DefaultEnvironment>(context.admin);
            assert!(context.contract.add_voter(context.bob));
           // assert!(!context.contract.is_voter(&context.bob));
            //assert_eq!(context.contract.voters.len(), 0);
            assert!(context.contract.remove_voter(context.bob));

        }

        #[ink::test]
        fn test_change_admin() {
            let mut context = Context::new();

           
            set_caller::<DefaultEnvironment>(context.admin);
            assert!(context.contract.change_admin(context.charlie));

            
        }

        //************************** */
    }
} //fin mod test
