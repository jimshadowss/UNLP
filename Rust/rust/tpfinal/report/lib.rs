#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod report {
    use crate::report::string::String;
    use crate::report::vec::Vec;
    use ink::prelude::*;
    use sistema::SistemaRef;

    #[ink(storage)]
    ///Referencia al sistema principal
    pub struct Report {
        sistema: SistemaRef,
    }
    #[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug, Default)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Socio {
        pub dni: u32,
        pub nombre: String,
        pub categoria: Categorias,
    }

    impl Socio {
        fn new(dni: u32, nombre: String, categoria: Categorias) -> Socio {
            Socio {
                dni,
                nombre,
                categoria,
            }
        }
    }
    #[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug, Default)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Categorias {
        #[default]
        A,
        B,
        C,
    }

    impl Report {
        #[ink(constructor)]
        pub fn new(sistema: SistemaRef) -> Self {
            Self { sistema }
        }
        #[ink(message)]
        pub fn solicitar_permiso(&mut self) -> Result<String, String> {
            self.sistema.solicitar_permiso(self.env().caller())
        }
        #[ink(message)]
        pub fn get_address(&self) -> AccountId {
            self.sistema.get_address()
        }
        #[ink(message)]
        pub fn verificacion_pagos_pendientes(&self) -> Result<Vec<Socio>, String> {
            let mut vec = Vec::new();
            match self.sistema.verificacion_pagos_pendientes() {
                Ok(a) => {
                    for i in a {
                        let socio = Socio::new(
                            i.dni,
                            i.nombre,
                            match i.categoria {
                                sistema::sistema::Categorias::A(_) => Categorias::A,
                                sistema::sistema::Categorias::B(_) => Categorias::B,
                                sistema::sistema::Categorias::C(_) => Categorias::C,
                            },
                        );
                        vec.push(socio);
                    }
                    Ok(vec)
                }
                Err(e) => Err(e),
            }
        }

        #[ink(message)]
        pub fn informe_recaudacion_mensual(&self, categoria: String) -> Result<u32, String> {
            Ok(self.sistema.informe_recaudacion_mensual(categoria)?)
        }
        #[ink(message)]
        pub fn informe_socios_no_morosos_actividad(
            &self,
            actividad: String,
        ) -> Result<Vec<Socio>, String> {
            let aux = self.sistema.get_no_morosos_act(actividad);
            let mut vec = Vec::new();
            match aux {
                Ok(a) => {
                    for i in a {
                        let socio = Socio::new(
                            i.dni,
                            i.nombre,
                            match i.categoria {
                                sistema::sistema::Categorias::A(_) => Categorias::A,
                                sistema::sistema::Categorias::B(_) => Categorias::B,
                                sistema::sistema::Categorias::C(_) => Categorias::C,
                            },
                        );
                        vec.push(socio);
                    }
                    Ok(vec)
                }
                Err(e) => Err(e),
            }
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {}
    }

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = ReportRef::default();

            // When
            let contract_account_id = client
                .instantiate("report", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get =
                build_message::<ReportRef>(contract_account_id.clone()).call(|report| report.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = ReportRef::new(false);
            let contract_account_id = client
                .instantiate("report", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get =
                build_message::<ReportRef>(contract_account_id.clone()).call(|report| report.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<ReportRef>(contract_account_id.clone())
                .call(|report| report.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get =
                build_message::<ReportRef>(contract_account_id.clone()).call(|report| report.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
