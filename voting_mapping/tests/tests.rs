#[cfg(test)]
mod tests {
   use crate::src::votantes ; 
    use ink::env::{test::set_caller, DefaultEnvironment};

    pub struct Context {
        contract: votantes::Votantes,
        admin: AccountId,
        alice: AccountId,
        bob: AccountId,
        charlie: AccountId,
    }

    impl Context {
        pub fn new() -> Self {
            // ... (resto )
        }
    }

    #[ink::test]
    fn test_change_admin() {
        let context = Context::new();

        // 
        
    }

    #[ink::test]
    fn test_add_voter() {
        let context = Context::new();

       
    }

    /
}
