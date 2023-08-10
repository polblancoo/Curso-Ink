#[cfg(test)]
mod tests {
   use crate::src::votantes ; // Ajusta la ruta a tu módulo votantes
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
            // ... (resto del código para configurar el contexto)
        }
    }

    #[ink::test]
    fn test_change_admin() {
        let context = Context::new();

        // Implementa las pruebas para la función change_admin aquí
        // ...
    }

    #[ink::test]
    fn test_add_voter() {
        let context = Context::new();

        // Implementa las pruebas para la función add_voter aquí
        // ...
    }

    // Resto de las funciones de prueba...
}
