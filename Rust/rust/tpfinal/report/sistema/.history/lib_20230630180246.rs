#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod Sistema {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Sistema {
        registro_pagos: Vec<Pago>,
        datos_socios: Vec<Socio>,
        precio_a: u32,
        precio_b: u32,
        precio_c: u32,
        porcentaje_descuento: u32,
        cantidad_pagos_consecutivos: u8,
    }
    #[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    struct Pago {
        monto: u32,
        vencimiento: u64,
        socio: Socio,
        pagado: bool,
        fecha_de_pago: Option<u64>,
        con_descuento: bool,
    }
    impl Clone for Pago {
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Pago {
        fn new(monto: u32, socio: Socio, fecha: u64, con_descuento: bool) -> Pago {
            let fecha_de_pago = None;
            let vencimiento = fecha + 10;
            Pago {
                monto,
                vencimiento,
                socio,
                pagado: false,
                fecha_de_pago,
                con_descuento,
            }
        }
    }
    #[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Socio {
        dni: u32,
        nombre: String,
        categoria: Categorias,
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
    impl Clone for Socio {
        fn clone(&self) -> Self {
            let socio = Socio::new(
                self.dni,
                self.nombre.clone(),
                match &self.categoria {
                    Categorias::A(a) => Categorias::A(a.clone()),
                    Categorias::B(b) => Categorias::B(b.clone()),
                    Categorias::C(c) => Categorias::C(c.clone()),
                },
            );
            socio
        }
    }

    #[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    enum Categorias {
        A(InfoCat),
        B(InfoCat),
        C(InfoCat),
    }
    #[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]

    struct InfoCat {
        precio_actual: u32,
        actividades: Vec<Actividades>,
    }
    impl Clone for InfoCat {
        fn clone(&self) -> Self {
            let precio_actual: u32 = self.precio_actual;
            let mut actividades = Vec::new();
            for i in &self.actividades {
                actividades.push(i.clone());
            }
            InfoCat {
                precio_actual,
                actividades,
            }
        }
    }
    //self.env() ver funciones envirmonment ink
    impl InfoCat {
        fn new_a_o_c(cat: Categorias, sistema: &Sistema) -> InfoCat {
            match cat {
                Categorias::A(_) => {
                    let precio_actual = sistema.precio_a;
                    let mut actividades = Vec::<Actividades>::new();
                    actividades.push(Actividades::Futbol);
                    actividades.push(Actividades::Gimnasio);
                    actividades.push(Actividades::Basquet);
                    actividades.push(Actividades::Rugby);
                    actividades.push(Actividades::Hockey);
                    actividades.push(Actividades::Natacion);
                    actividades.push(Actividades::Tenis);
                    actividades.push(Actividades::Paddle);
                    InfoCat {
                        precio_actual,
                        actividades,
                    }
                }

                Categorias::C(_) => {
                    let precio_actual = sistema.precio_c;
                    let mut actividades = Vec::<Actividades>::new();
                    actividades.push(Actividades::Gimnasio);
                    InfoCat {
                        precio_actual,
                        actividades,
                    }
                }
                _ => panic!("invalido"),
            }
        }
        fn new_b(cat: Categorias, sistema: &Sistema, actividad: Actividades) -> InfoCat {
            match cat {
                Categorias::B(_) => {
                    let mut actividades = Vec::new();
                    let precio_actual = sistema.precio_b;
                    actividades.push(actividad);
                    InfoCat {
                        precio_actual,
                        actividades,
                    }
                }
                _ => panic!("invalido"),
            }
        }
    }
    #[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]

    pub enum Actividades {
        Futbol,
        Gimnasio,
        Basquet,
        Rugby,
        Hockey,
        Natacion,
        Tenis,
        Paddle,
    }
    impl Clone for Actividades {
        fn clone(&self) -> Self {
            match self {
                Actividades::Futbol => Actividades::Futbol,
                Actividades::Gimnasio => Actividades::Gimnasio,
                Actividades::Basquet => Actividades::Basquet,
                Actividades::Rugby => Actividades::Rugby,
                Actividades::Hockey => Actividades::Hockey,
                Actividades::Natacion => Actividades::Natacion,
                Actividades::Paddle => Actividades::Paddle,
                Actividades::Tenis => Actividades::Tenis,
            }
        }
    }

    impl Sistema {
        #[ink(constructor)]
        pub fn new(
            precio_a: u32,
            precio_b: u32,
            precio_c: u32,
            porcentaje_descuento: u32,
            cantidad_pagos_consecutivos: u8,
        ) -> Sistema {
            let registro_pagos = Vec::new();
            let datos_socios = Vec::new();
            Sistema {
                registro_pagos,
                datos_socios,
                precio_a,
                precio_b,
                precio_c,
                porcentaje_descuento,
                cantidad_pagos_consecutivos,
            }
        }
        #[ink(message)]
        pub fn set_porcentaje(&mut self, porcentaje: u32) {
            self.porcentaje_descuento = porcentaje;
        }
        #[ink(message)]
        pub fn get_porcentaje(&self) -> u32 {
            self.porcentaje_descuento
        }
        #[ink(message)]
        pub fn set_cantidad_pagos_consecutivos(&mut self, cantidad: u8) {
            self.cantidad_pagos_consecutivos = cantidad;
        }
        #[ink(message)]
        pub fn get_cantidad_pagos_consecutivos(&self) -> u8 {
            self.cantidad_pagos_consecutivos
        }
        #[ink(message)]
        pub fn cambiar_precios(&mut self, precio_a: u32, precio_b: u32, precio_c: u32) {
            self.cambiar_precios2(precio_a, precio_b, precio_c)
        }
        fn cambiar_precios2(&mut self, precio_a: u32, precio_b: u32, precio_c: u32) {
            self.precio_a = precio_a;
            self.precio_b = precio_b;
            self.precio_c = precio_c;
        }
        #[ink(message)]
        pub fn agregar_socio(
            &mut self,
            dni: u32,
            nombre: String,
            cat: Categorias,
            actividad: Option<Actividades>,
        ) -> bool {
            self.agregar_socio2(dni, nombre, cat, actividad)
        }
        fn agregar_socio2(
            &mut self,
            dni: u32,
            nombre: String,
            cat: Categorias,
            actividad: Option<Actividades>,
        ) -> bool {
            let info: InfoCat;
            let mut hecho: bool = false;
            let categ: Categorias;
            let socio: Socio;
            let pago: Pago;
            let mut esta = false;
            for i in &self.datos_socios {
                if i.dni == dni {
                    esta = true;
                    break;
                }
            }
            if !esta {
                match cat {
                    Categorias::A(_) => {
                        //validar existencia previa
                        info = InfoCat::new_a_o_c(cat, self);
                        categ = Categorias::A(info);
                        socio = Socio::new(dni, nombre, categ);
                        pago = Pago::new(
                            self.precio_a,
                            socio.clone(),
                            0, //self.env().block_timestamp(),
                            false,
                        );
                        self.datos_socios.push(socio);
                        self.registro_pagos.push(pago);
                        hecho = true;
                    }
                    Categorias::B(_) => {
                        if let Some(a) = actividad {
                            let info = InfoCat::new_b(cat, self, a);
                            let categ = Categorias::B(info);
                            let socio = Socio::new(dni, nombre, categ);
                            let pago = Pago::new(
                                self.precio_c,
                                self.datos_socios.last().unwrap().clone(),
                                0, //self.env().block_timestamp(),
                                false,
                            );
                            self.datos_socios.push(socio);
                            self.registro_pagos.push(pago);
                            hecho = true;
                        }
                    }
                    Categorias::C(_) => {
                        info = InfoCat::new_a_o_c(cat, self);
                        categ = Categorias::C(info);
                        socio = Socio::new(dni, nombre, categ);
                        pago = Pago::new(
                            self.precio_c,
                            socio.clone(),
                            0, //self.env().block_timestamp(),
                            false,
                        );
                        self.datos_socios.push(socio);
                        self.registro_pagos.push(pago);
                        hecho = true;
                    }
                    _ => (),
                }
            }
            hecho
        }
        #[ink(message)]
        pub fn get_proximo_pago(&self, dni: u32) -> Option<Pago> {
            self.get_proximo_pago2(dni)
        }
        fn get_proximo_pago2(&self, dni: u32) -> Option<Pago> {
            let mut res: Option<Pago> = None;
            for i in &self.registro_pagos {
                if i.socio.dni == dni && !i.pagado {
                    res = Some(i.clone());
                    break;
                }
            }
            res
        }
        fn si_descuento(&self, dni: u32) -> bool {
            let mut es = false;
            let socios_iter = self.registro_pagos.iter();
            let check = |pago: &Pago, dni: u32| {
                if pago.socio.dni == dni {
                    true
                } else {
                    false
                }
            };
            let aux = socios_iter
                .filter(move |x| check(x, dni))
                .collect::<Vec<_>>();
            let mut cont: u8 = 0;
            for i in aux.len()..0 {
                if let Some(a) = aux[i].fecha_de_pago {
                    if !aux[i].con_descuento || aux[i].vencimiento >= a {
                        cont += 1;
                    } else {
                        break;
                    }
                }
                if cont == 3 {
                    es = true;
                }
            }
            es
        }
        #[ink(message)]
        pub fn registrar_pago(&mut self, dni: u32, monto_pagado: u32) -> bool {
            self.registrar_pago2(dni, monto_pagado)
        }
        fn registrar_pago2(&mut self, dni: u32, monto_pagado: u32) -> bool {
            let mut hecho = false;
            let timestamp = 0;
            for i in &self.datos_socios {
                if i.dni == dni {
                    for i in 0..self.registro_pagos.len() {
                        if self.registro_pagos[i].socio.dni == dni
                            && self.registro_pagos[i].pagado == false
                        {
                            let mut pago_aux = self.registro_pagos.remove(i);
                            pago_aux.pagado = true;
                            if self.si_descuento(dni) {
                                pago_aux.monto = pago_aux.monto * self.porcentaje_descuento / 100;
                            }
                            pago_aux.fecha_de_pago = Some(timestamp);
                            self.registro_pagos.insert(i, pago_aux);
                            hecho = true;
                            break;
                        }
                    }
                    break;
                }
            }
            hecho
        }
        #[ink(message)]
        pub fn consulta_pagos(&self, dni: Option<u32>) -> Vec<Pago> {
            self.consulta_pagos2(dni)
        }
        fn consulta_pagos2(&self, dni: Option<u32>) -> Vec<Pago> {
            let mut aux: Vec<_> = Vec::new();
            let aux_reg = &self.registro_pagos;
            if let Some(a) = dni {
                let socios_iter = aux_reg.iter();
                let check = |pago: &Pago, dni: u32| {
                    if pago.socio.dni == dni {
                        true
                    } else {
                        false
                    }
                };
                let aux2 = socios_iter.filter(move |x| check(x, a)).collect::<Vec<_>>();
                for i in aux2 {
                    aux.push(i.clone());
                }
            } else {
                for i in &self.registro_pagos {
                    aux.push(i.clone());
                }
            }

            return aux;
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
        // #[ink::test]
        fn default_works() {
            // let Sistema = Sistema::default();
            // assert_eq!(Sistema.get(), false);
        }

        /// We test a simple use case of our contract.
        // #[ink::test]
        fn it_works() {
            // let mut Sistema = Sistema::new(false);
            // assert_eq!(Sistema.get(), false);
            // Sistema.flip();
            // assert_eq!(Sistema.get(), true);
        }
    }
}
