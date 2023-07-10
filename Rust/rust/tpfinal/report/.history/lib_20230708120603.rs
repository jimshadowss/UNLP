#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod report {
    use sistema::SistemaRef;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Report {
        /// Stores a single `bool` value on the storage.
        sistema: SistemaRef,
    }

    impl Report {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(sistema: SistemaRef) -> Self {
            Self { sistema }
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn verificacion_pagos_pendientes(&self) -> Result<Vec<u32>, String> {
            let res = self.sistema.consulta_pagos(None);
            let mut vec = Vec::new();
            match res {
                Ok(a) => {
                    for i in a {
                        if i.fuera_de_termino_no_pagado(self.env().block_timestamp()) {
                            if !vec.contains(&i.get_socio().dni) {
                                vec.push(i.get_socio().dni);
                            }
                        }
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
