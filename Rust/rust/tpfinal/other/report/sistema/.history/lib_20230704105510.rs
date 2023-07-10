#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod sistema {
    use ink::prelude::string::String;
    use ink::prelude::string::ToString;
    use ink::prelude::vec::Vec;

    /// Sistema es el struct donde se guardan los datos, el sistema mismo
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
    #[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug, Default)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]

    ///Pago cuenta con monto, fecha de vencimiento, socio asociado al pago, si
    ///esta pagado o no, fecha de pago (si ya fue pagado), si fue con descuento
    ///  (si ya fue pagado)
    pub struct Pago {
        monto: u32,
        vencimiento: u64,
        socio: Socio,
        pagado: bool,
        fecha_de_pago: Option<u64>,
        con_descuento: bool,
    }
    impl Clone for Pago {
        fn clone(&self) -> Self {
            let monto = self.monto;
            let vencimiento = self.vencimiento;
            let socio = self.socio.clone();
            let pagado = self.pagado;
            let fecha_de_pago = self.fecha_de_pago;
            let con_descuento = self.con_descuento;
            Pago {
                monto,
                vencimiento,
                socio,
                pagado,
                fecha_de_pago,
                con_descuento,
            }
        }
    }
    impl Pago {
        fn new(monto: u32, socio: Socio, vencimiento: u64) -> Pago {
            let fecha_de_pago = None;
            let con_descuento = false;
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

    ///Socio cuenta con DNI del socio, nombre del socio y categoria de socio
    #[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug, Default)]
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

    ///Categorias son las categorias de socio: A, B o C
    pub enum Categorias {
        A(InfoCat),
        B(InfoCat),
        C(InfoCat),
    }

    impl Default for Categorias {
        fn default() -> Self {
            let info = InfoCat::default();
            Categorias::A(info)
        }
    }

    /// InfoCat es la informacion relacionada a la categoria de socio, cuenta con el precio de la
    /// categoria y una lista con las actividades en las que puede participar el socio de esa categoria
    #[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug, Default)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct InfoCat {
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
    impl InfoCat {
        fn new_a_o_c(cat: char, sistema: &Sistema) -> InfoCat {
            match cat {
                'A' | 'a' => {
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

                'C' | 'c' => {
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
        fn new_b(cat: char, sistema: &Sistema, actividad: Actividades) -> InfoCat {
            match cat {
                'B' | 'b' => {
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

    ///Actividades son los deportes que se practican en el club
    #[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug, Default)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]

    pub enum Actividades {
        #[default]
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
        /// Constructor del sistema al momento de hacer deploy
        /// tiene los precios asociados a cada categoria, el porcentaje de descuento por pagos
        /// consecutivos y la cantidad de pagos consecutivos requeridos para acceder al descuento
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
        #[ink(constructor)]
        pub fn default() -> Sistema {
            let precio_a = 0;
            let precio_b = 0;
            let precio_c = 0;
            let porcentaje_descuento = 0;
            let cantidad_pagos_consecutivos: u8 = 0;
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
        ///cambiar el porcentaje de descuento para los pagos consecutivos
        #[ink(message)]
        pub fn set_porcentaje(&mut self, porcentaje: u32) {
            self.porcentaje_descuento = porcentaje;
        }
        ///ver el porcentaje de descuento para los pagos consecutivos
        #[ink(message)]
        pub fn get_porcentaje(&self) -> u32 {
            self.porcentaje_descuento
        }
        ///cambiar la cantidad de pagos consecutivos requeridos para acceder al descuento
        #[ink(message)]
        pub fn set_cantidad_pagos_consecutivos(&mut self, cantidad_pagos_consecutivos: u8) {
            self.cantidad_pagos_consecutivos = cantidad_pagos_consecutivos;
        }
        ///ver la cantidad de pagos consecutivos requeridos para acceder al descuento
        #[ink(message)]
        pub fn get_cantidad_pagos_consecutivos(&self) -> u8 {
            self.cantidad_pagos_consecutivos
        }
        ///ver el precio de la categoria A
        #[ink(message)]
        pub fn get_precio_a(&self) -> u32 {
            self.precio_a
        }
        ///ver el precio de la categoria B
        #[ink(message)]
        pub fn get_precio_b(&self) -> u32 {
            self.precio_b
        }
        ///ver el precio de la categoria C
        #[ink(message)]
        pub fn get_precio_c(&self) -> u32 {
            self.precio_c
        }
        ///cambiar el precio de la categoria A
        #[ink(message)]
        pub fn set_precio_a(&mut self, precio_a: u32) {
            self.set_precios_a2(precio_a)
        }
        fn set_precios_a2(&mut self, precio_a: u32) {
            self.precio_a = precio_a;
        }
        ///cambiar el precio de la categoria B
        #[ink(message)]
        pub fn set_precio_b(&mut self, precio_b: u32) {
            self.set_precios_b2(precio_b)
        }
        fn set_precios_b2(&mut self, precio_b: u32) {
            self.precio_b = precio_b;
        }
        ///cambiar el precio de la categoria C
        #[ink(message)]
        pub fn set_precio_c(&mut self, precio_c: u32) {
            self.set_precios_c2(precio_c)
        }
        fn set_precios_c2(&mut self, precio_c: u32) {
            self.precio_c = precio_c;
        }
        ///agregar un socio nuevo
        #[ink(message)]
        pub fn agregar_socio(
            &mut self,
            dni: u32,
            nombre: String,
            categoria: String,
            actividad: Option<Actividades>,
        ) -> bool {
            self.agregar_socio2(dni, nombre, categoria, actividad)
        }
        fn agregar_socio2(
            &mut self,
            dni: u32,
            nombre: String,
            cat: String,
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
                if (cat.eq(&"A".to_string()) || cat.eq(&"a".to_string())) && actividad == None {
                    info = InfoCat::new_a_o_c('A', self);
                    categ = Categorias::A(info);
                    socio = Socio::new(dni, nombre, categ);
                    pago = Pago::new(
                        self.precio_a,
                        socio.clone(),
                        self.env().block_timestamp() + 864_000_000,
                    );
                    self.datos_socios.push(socio);
                    self.registro_pagos.push(pago);
                    hecho = true;
                } else if cat.eq(&"B".to_string()) || cat.eq(&"b".to_string()) {
                    if let Some(b) = actividad {
                        let info = InfoCat::new_b('B', self, b);
                        let categ = Categorias::B(info);
                        let socio = Socio::new(dni, nombre, categ);
                        let pago =
                            Pago::new(self.precio_c, socio.clone(), self.env().block_timestamp());
                        self.datos_socios.push(socio);
                        self.registro_pagos.push(pago);
                        hecho = true;
                    }
                } else if (cat.eq(&"C".to_string()) || cat.eq(&"c".to_string()))
                    && actividad == None
                {
                    info = InfoCat::new_a_o_c('C', self);
                    categ = Categorias::C(info);
                    socio = Socio::new(dni, nombre, categ);
                    pago = Pago::new(self.precio_c, socio.clone(), self.env().block_timestamp());
                    self.datos_socios.push(socio);
                    self.registro_pagos.push(pago);
                    hecho = true;
                }
            }
            hecho
        }
        fn get_socio(&self, dni: u32) -> Option<Socio> {
            let socio: Socio;
            for i in &self.datos_socios {
                if i.dni == dni {
                    socio = i.clone();
                    return Some(socio);
                }
            }
            return None;
        }
        ///ver el proximo pago
        #[ink(message)]
        pub fn get_proximo_pago(&self, dni: u32) -> Option<Pago> {
            self.get_proximo_pago2(dni)
        }
        fn get_proximo_pago2(&self, dni: u32) -> Option<Pago> {
            let mut res: Option<Pago> = None;
            for i in &self.registro_pagos {
                if i.socio.dni == dni && !i.pagado {
                    let mut aux = Some(i.clone());
                    if self.si_descuento(dni) {
                        let mut pago_aux = aux.unwrap();
                        pago_aux.monto = pago_aux.monto * self.porcentaje_descuento / 100;
                        pago_aux.con_descuento = true;
                        aux = Some(pago_aux);
                    }
                    res = aux;
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
            let mut i = aux.len();
            while i >= 1 && aux.len() - i < self.cantidad_pagos_consecutivos as usize {
                i -= 1;
                if let Some(a) = aux[i].fecha_de_pago {
                    if !aux[i].con_descuento && aux[i].vencimiento >= a {
                        cont += 1;
                    } else {
                        break;
                    }
                }
                if cont == self.cantidad_pagos_consecutivos {
                    es = true;
                    break;
                }
            }

            es
        }
        ///ver timestamp actual
        #[ink(message)]
        pub fn get_timestamp(&self) -> u64 {
            self.env().block_timestamp()
        }
        ///marcar pago pendiente como pagado
        #[ink(message)]
        pub fn registrar_pago(&mut self, dni: u32, monto_pagado: u32) -> bool {
            self.registrar_pago2(dni, monto_pagado)
        }
        fn registrar_pago2(&mut self, dni: u32, monto_pagado: u32) -> bool {
            let mut hecho = false; //ver crear nuevo pago al registrar pago
            let timestamp = self.env().block_timestamp();
            if let Some(socio) = self.get_socio(dni) {
                for i in 0..self.registro_pagos.len() {
                    if socio.dni == self.registro_pagos[i].socio.dni
                        && self.registro_pagos[i].pagado == false
                    {
                        let mut pago_aux = self.registro_pagos.remove(i);
                        let monto = match socio.clone().categoria {
                            Categorias::A(_) => self.precio_a,
                            Categorias::B(_) => self.precio_b,
                            Categorias::C(_) => self.precio_c,
                        };
                        let pago =
                            Pago::new(monto, socio.clone(), pago_aux.vencimiento + 2_592_000_000);
                        self.registro_pagos.push(pago);
                        if self.si_descuento(dni)
                            && monto_pagado >= pago_aux.monto * self.porcentaje_descuento / 100
                        {
                            pago_aux.monto = pago_aux.monto * self.porcentaje_descuento / 100;
                            pago_aux.pagado = true;
                            pago_aux.con_descuento = true;
                            pago_aux.fecha_de_pago = Some(timestamp);
                        } else if pago_aux.monto <= monto_pagado {
                            pago_aux.pagado = true;
                            pago_aux.fecha_de_pago = Some(timestamp);
                        }
                        // let mut paid_time = pago_aux.vencimiento;
                        // let time = self.env().block_timestamp();
                        // while paid_time < time {
                        //     paid_time += 2_592_000_000;

                        //     let monto = match socio.clone().categoria {
                        //         Categorias::A(_) => self.precio_a,
                        //         Categorias::B(_) => self.precio_b,
                        //         Categorias::C(_) => self.precio_c,
                        //     };
                        //     let vencimiento = paid_time;
                        //     let pago = Pago::new(monto, socio.clone(), vencimiento);
                        //     self.registro_pagos.push(pago);
                        // }

                        self.registro_pagos.insert(i, pago_aux);

                        hecho = true;

                        break;
                    }
                }
            } else {
            }

            hecho
        }
        ///consultar todos los pagos de un socio con el dni, o consultar todos los pagos sin dni
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
        #[ink::test]
        fn pago_clone_test() {
            let pago = Pago::default();
            let p2 = pago.clone();
            assert_eq!(pago, p2)
        }
        #[ink::test]
        fn new_pago_test() {
            let monto = 0;
            let vencimiento = 0;
            let socio = Socio::default();
            let pagado = false;
            let pago = Pago::new(monto, socio.clone(), vencimiento);
            assert!(
                pago.monto == monto
                    && pago.vencimiento == vencimiento
                    && pago.socio == socio
                    && pago.pagado == pagado
                    && pago.fecha_de_pago == None
                    && pago.con_descuento == false
            )
        }

        #[ink::test]
        fn socio_clone_test() {
            let socio = Socio::default();
            let s2 = socio.clone();
            assert_eq!(socio, s2)
        }
        #[ink::test]
        fn socio_new_test() {
            let dni = 0;
            let nombre = "".to_string();
            let categoria = Categorias::default();
            let socio = Socio::new(dni, nombre.clone(), categoria);
            assert!(
                socio.dni == dni
                    && socio.nombre.eq(&nombre)
                    && match (socio.categoria, Categorias::default()) {
                        (Categorias::A(_), Categorias::A(_)) => true,
                        (Categorias::B(_), Categorias::B(_)) => true,
                        (Categorias::C(_), Categorias::C(_)) => true,
                        _ => false,
                    }
            )
        }
        #[ink::test]
        fn default_categorias() {
            let cat = Categorias::default();
            assert!(match cat {
                Categorias::A(_) => true,
                _ => false,
            })
        }
        #[ink::test]
        fn info_clone_test() {
            let info = InfoCat::default();
            let i2 = info.clone();
            assert_eq!(info, i2)
        }
        #[ink::test]
        fn new_a_o_c_test() {
            let mut sis = Sistema::default();
            sis.precio_a = 30;
            let info = InfoCat::new_a_o_c('a', &sis);
            assert!(info.precio_actual == sis.precio_a && info.actividades.len() == 8);
        }
        #[ink::test]
        #[should_panic]
        fn new_a_o_c_test2() {
            let sis = Sistema::default();
            InfoCat::new_a_o_c('b', &sis);
        }
        #[ink::test]
        fn new_a_o_c_test3() {
            let mut sis = Sistema::default();
            sis.precio_a = 30;
            let info = InfoCat::new_a_o_c('c', &sis);
            assert!(info.precio_actual == sis.precio_c && info.actividades.len() == 1);
        }
        #[ink::test]
        fn new_b_test() {
            let mut sis = Sistema::default();
            sis.precio_b = 30;
            let info = InfoCat::new_b('b', &sis, Actividades::default());
            assert!(info.precio_actual == sis.precio_b && info.actividades.len() == 1);
        }
        #[ink::test]
        #[should_panic]
        fn new_b_test2() {
            let sis = Sistema::default();
            InfoCat::new_b('a', &sis, Actividades::default());
        }
        #[ink::test]
        fn clone_actividades_test1() {
            let a = Actividades::Futbol;
            let b = a.clone();
            assert!(match b {
                Actividades::Futbol => true,
                _ => false,
            })
        }
        #[ink::test]
        fn clone_actividades_test2() {
            let a = Actividades::Gimnasio;
            let b = a.clone();
            assert!(match b {
                Actividades::Gimnasio => true,
                _ => false,
            })
        }
        #[ink::test]
        fn clone_actividades_test3() {
            let a = Actividades::Basquet;
            let b = a.clone();
            assert!(match b {
                Actividades::Basquet => true,
                _ => false,
            })
        }
        #[ink::test]
        fn clone_actividades_test4() {
            let a = Actividades::Rugby;
            let b = a.clone();
            assert!(match b {
                Actividades::Rugby => true,
                _ => false,
            })
        }
        #[ink::test]
        fn clone_actividades_test5() {
            let a = Actividades::Hockey;
            let b = a.clone();
            assert!(match b {
                Actividades::Hockey => true,
                _ => false,
            })
        }
        #[ink::test]
        fn clone_actividades_test6() {
            let a = Actividades::Natacion;
            let b = a.clone();
            assert!(match b {
                Actividades::Natacion => true,
                _ => false,
            })
        }
        #[ink::test]
        fn clone_actividades_test7() {
            let a = Actividades::Tenis;
            let b = a.clone();
            assert!(match b {
                Actividades::Tenis => true,
                _ => false,
            })
        }
        #[ink::test]
        fn clone_actividades_test8() {
            let a = Actividades::Paddle;
            let b = a.clone();
            assert!(match b {
                Actividades::Paddle => true,
                _ => false,
            })
        }
        #[ink::test]
        fn new_sistema_test() {
            let precio_a = 1;
            let precio_b = 2;
            let precio_c = 3;
            let porcentaje_descuento = 4;
            let cantidad_pagos_consecutivos = 5;
            let sis = Sistema::new(
                precio_a,
                precio_b,
                precio_c,
                porcentaje_descuento,
                cantidad_pagos_consecutivos,
            );
            assert!(
                sis.precio_a == 1
                    && sis.precio_b == 2
                    && sis.precio_c == 3
                    && sis.porcentaje_descuento == 4
                    && sis.cantidad_pagos_consecutivos == 5
                    && sis.datos_socios.len() == 0
                    && sis.registro_pagos.len() == 0
            )
        }
        #[ink::test]
        fn default_sistema_test() {
            let sis = Sistema::default();
            assert!(
                sis.precio_a == 0
                    && sis.precio_b == 0
                    && sis.precio_c == 0
                    && sis.porcentaje_descuento == 0
                    && sis.cantidad_pagos_consecutivos == 0
                    && sis.datos_socios.len() == 0
                    && sis.registro_pagos.len() == 0
            )
        }

        #[ink::test]
        fn agregar_socio_test() {
            let mut sis = Sistema::default();
            let dni = 1;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            sis.agregar_socio2(dni, nombre.clone(), categoria.clone(), actividad);
            let mut esta = false;
            for i in sis.datos_socios {
                if i.dni == dni && i.nombre.eq(&nombre) {
                    esta = true;
                }
            }
            assert!(
                esta && sis.registro_pagos[0].pagado == false
                    && sis.registro_pagos[0].vencimiento > 0
            )
        }
        #[ink::test]
        fn agregar_socio_test2() {
            let mut sis = Sistema::default();
            let dni = 1;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            sis.agregar_socio2(dni, nombre.clone(), categoria.clone(), actividad.clone());
            sis.agregar_socio2(dni, nombre.clone(), categoria, actividad);
            let mut veces = 0;
            for i in sis.datos_socios {
                if i.dni == dni && i.nombre.eq(&nombre) {
                    veces += 1;
                }
            }
            assert!(veces == 1)
        }
        #[ink::test]
        fn get_socio_test() {
            let mut sis = Sistema::default();
            let dni = 1;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            sis.agregar_socio2(dni, nombre.clone(), categoria.clone(), actividad.clone());
            let socio = sis.get_socio(dni);
            assert!(socio.clone().unwrap().dni == dni && socio.unwrap().nombre.eq(&nombre))
        }
        #[ink::test]
        fn get_proximo_pago_test() {
            let mut sis = Sistema::default();
            let dni = 1;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            sis.agregar_socio2(dni, nombre.clone(), categoria.clone(), actividad.clone());
            let mut pago = Pago::default();
            pago.pagado = false;
            sis.registro_pagos.push(pago.clone());
            let aux = sis.get_proximo_pago(dni);
            assert!(aux.unwrap().vencimiento != 0)
        }
        #[ink::test]
        fn registrar_pago_test() {
            let mut sis = Sistema::default();
            let dni = 1;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            sis.agregar_socio2(dni, nombre.clone(), categoria.clone(), actividad.clone());
            sis.registrar_pago2(dni, sis.precio_a);
            let venc = sis.get_proximo_pago2(dni).unwrap().vencimiento;
            sis.registrar_pago2(dni, sis.precio_a);
            assert!(
                sis.get_proximo_pago2(dni).unwrap().vencimiento >= venc
                    && sis.registro_pagos.len() == 3
            )
        }
        #[ink::test]
        fn si_descuento_test() {
            let mut sis = Sistema::default();
            sis.set_cantidad_pagos_consecutivos(1);
            let dni = 1;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            sis.agregar_socio2(dni, nombre.clone(), categoria.clone(), actividad.clone());
            let info = InfoCat::default();
            let socio = Socio::new(dni, nombre.clone(), Categorias::A(info));
            let mut pago = Pago::new(sis.precio_a, socio, 1);
            pago.pagado = true;
            sis.registro_pagos.push(pago.clone());
            sis.registro_pagos.push(pago.clone());
            sis.registro_pagos.push(pago.clone());
            sis.registro_pagos.push(pago.clone());
            assert!(sis.si_descuento(dni))
        }
        #[ink::test]
        fn consulta_pagos_test() {
            let mut sis = Sistema::default();
            sis.set_cantidad_pagos_consecutivos(1);
            let dni = 1;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            sis.agregar_socio2(dni, nombre.clone(), categoria.clone(), actividad.clone());

            let dni = 2;
            let nombre = "b".to_string();
            let categoria = "B".to_string();
            let actividad = None;
            sis.agregar_socio2(dni, nombre.clone(), categoria.clone(), actividad.clone());

            sis.registrar_pago(1, sis.precio_a);
            let vec = sis.consulta_pagos(Some(1));
            let mut esta_solo = true;
            for i in &vec {
                if i.socio.dni != 1 {
                    esta_solo = false;
                }
            }
            assert!(esta_solo && vec.len() == 2)
        }
    }
}
