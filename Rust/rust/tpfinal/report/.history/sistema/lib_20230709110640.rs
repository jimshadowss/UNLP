#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::sistema::SistemaRef;
#[ink::contract]
mod sistema {
    use ink::prelude::format;
    use ink::prelude::string::String;
    use ink::prelude::string::ToString;
    use ink::prelude::vec::Vec;

    /// Sistema es el struct donde se guardan los datos, el sistema mismo, tiene un registro de pagos donde se guardan los pagos realizados y los pagos pendientes de pago
    /// tiene un vector con los datos de socios
    /// tiene los precios de las categorias
    /// tiene el porcentaje de descuento y cantidad de pagos consecutivos requeridos
    /// tiene un option con el address del owner del contrato (siempre se guarda como Some(_) al instanciar con new)
    /// tiene un vector con los addresses de los miembros del staff, que tendran algunos permisos pero no todos
    /// y tiene como booleano si la politica de permisos esta activada o no
    #[ink(storage)]
    pub struct Sistema {
        registro_pagos: Vec<Pago>,
        datos_socios: Vec<Socio>,
        precio_a: u32,
        precio_b: u32,
        precio_c: u32,
        porcentaje_descuento: u32,
        cantidad_pagos_consecutivos: u8,
        owner: Option<AccountId>,
        staff: Vec<AccountId>,
        permisos_privados: bool,
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
    pub enum Permiso {
        Owner,
        Staff,
        Ninguno,
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
        ///pago se instancia sin fecha de pago y sin descuento aplicado, se verá si tiene descuento al buscarlo con getProximoPago
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
        ///para uso interno, checkea si el pago está fuera de termino (pagado o no)
        fn fuera_de_termino(&self, time: u64) -> bool {
            if (!self.pagado && time > self.vencimiento)
                || (self.pagado && self.fecha_de_pago.unwrap() > self.vencimiento)
            {
                true
            } else {
                false
            }
        }
        pub fn fuera_de_termino_no_pagado(&self, time: u64) -> bool {
            if !self.pagado && time > self.vencimiento {
                true
            } else {
                false
            }
        }
        pub fn get_socio(&self) -> Socio {
            self.socio.clone()
        }
    }

    ///Socio cuenta con DNI del socio, nombre del socio y categoria de socio
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
        ///socio se instancia requiriendo todas sus variables como argumentos
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
    ///InfoCat se instancia segun los argumentos dados, si es A o C no acepta opcion de Actividad, si es B, requiere opcion de Actividad
    impl InfoCat {
        fn new(
            cat: String,
            sistema: &Sistema,
            actividad: Option<Actividades>,
        ) -> Result<InfoCat, String> {
            let error="Las entradas validas son A y C sin opcion de actividad, y B con opcion de actividad".to_string();
            if let Some(act) = actividad {
                if cat.eq("B") || cat.eq("b") {
                    let mut actividades = Vec::new();
                    let precio_actual = sistema.precio_b;
                    actividades.push(act);
                    let info = InfoCat {
                        precio_actual,
                        actividades,
                    };
                    Ok(info)
                } else {
                    Err(error)
                }
            } else {
                if cat.eq("A") || cat.eq("a") {
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
                    let info = InfoCat {
                        precio_actual,
                        actividades,
                    };
                    Ok(info)
                } else if cat.eq("C") || cat.eq("c") {
                    let precio_actual = sistema.precio_c;
                    let mut actividades = Vec::<Actividades>::new();
                    actividades.push(Actividades::Gimnasio);
                    let info = InfoCat {
                        precio_actual,
                        actividades,
                    };
                    Ok(info)
                } else {
                    Err(error)
                }
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
        /// registroPagos, datosSocios, y staff se instancian como vectores vacios
        /// owner se instancia con el address del caller
        /// permisos privados se instancia siempre como true
        /// el resto de las variables son requeridas como argumentos
        #[ink(constructor)]
        pub fn new(
            precio_a: u32,
            precio_b: u32,
            precio_c: u32,
            porcentaje_descuento: u32,
            cantidad_pagos_consecutivos: u8,
        ) -> Sistema {
            let mut sis = Sistema::new2(
                precio_a,
                precio_b,
                precio_c,
                porcentaje_descuento,
                cantidad_pagos_consecutivos,
            );
            sis.set_owner();
            sis
        }
        fn new2(
            precio_a: u32,
            precio_b: u32,
            precio_c: u32,
            porcentaje_descuento: u32,
            cantidad_pagos_consecutivos: u8,
        ) -> Sistema {
            let registro_pagos = Vec::new();
            let datos_socios = Vec::new();
            let owner = None;
            let permisos_privados = true;
            let staff = Vec::new();
            Sistema {
                registro_pagos,
                datos_socios,
                precio_a,
                precio_b,
                precio_c,
                porcentaje_descuento,
                cantidad_pagos_consecutivos,
                owner,
                staff,
                permisos_privados,
            }
        }
        fn set_owner(&mut self) {
            self.owner = Some(self.env().caller());
        }
        ///Delegar ownership solo puede ser llamado por el owner del contrato, y otorga ese ownership a un address dado por parametro
        #[ink(message)]
        pub fn delegar_ownership(&mut self, acc: AccountId) -> Result<String, String> {
            let res = self.check_permisos();
            match res.0 {
                Permiso::Owner => {
                    self.owner = Some(acc);
                    Ok(res.1)
                }
                _ => Err(res.2),
            }
        }
        // #[allow(dead_code)]
        fn default() -> Sistema {
            let precio_a = 0;
            let precio_b = 0;
            let precio_c = 0;
            let porcentaje_descuento = 0;
            let cantidad_pagos_consecutivos: u8 = 0;
            let registro_pagos = Vec::new();
            let datos_socios = Vec::new();
            let owner = None;
            let staff = Vec::new();
            let permisos_privados = false;
            Sistema {
                registro_pagos,
                datos_socios,
                precio_a,
                precio_b,
                precio_c,
                porcentaje_descuento,
                cantidad_pagos_consecutivos,
                owner,
                staff,
                permisos_privados,
            }
        }
        ///Timestamp_into_date para uso interno, convierte un timestamp en una tupla con la fecha, formato aaaa/mm/dd
        pub fn timestamp_into_date(time: u64) -> (u32, u8, u8) {
            let mut años: u32 = 0;
            let mut mes: u8;
            let mut dia: u8;
            let arreglo_meses = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            let mut res = (0, 0, 0);
            let setenta_y_tres = 94_694_400_000;
            let un_dia = 86_400;
            let un_año = setenta_y_tres / 2;
            let mut aux = time - setenta_y_tres;

            while aux > un_año {
                let mut cont = 0;
                if cont < 4 {
                    cont += 1;
                    aux -= un_año;
                } else {
                    cont = 0;
                    aux -= un_año + 86_400_000;
                }
                años += 1;
            }

            res
        }
        //check permisos se usa solo internamente, devuelve una tupla formada de la siguiente manera:
        ///0 un enum con el nivel de permiso que posee el caller
        ///1 un String con el mensaje de permiso concedido
        ///2 un String con el mensaje de permiso denegado
        /// (ambos para manejo de errores en las funciones que llaman a check_permisos)
        fn check_permisos(&self) -> (Permiso, String, String) {
            let mut permiso = Permiso::Ninguno;
            let no = "No cuenta con los permisos requeridos".to_string();
            let si = "Permiso concedido".to_string();
            let mut not = false;
            if self.permisos_privados {
                if let Some(a) = self.owner {
                    if a == self.env().caller() {
                        permiso = Permiso::Owner;
                        not = true;
                    }
                }
                if !not && self.staff.contains(&self.env().caller()) {
                    permiso = Permiso::Staff;
                } else if !not {
                    permiso = Permiso::Ninguno;
                }
            } else {
                permiso = Permiso::Owner;
            }
            (permiso, si, no)
        }
        ///set_porcentaje solo puede ser llamado por el owner, cambia el porcentaje de descuento para los pagos consecutivos
        #[ink(message)]
        pub fn set_porcentaje(&mut self, porcentaje: u32) -> Result<String, String> {
            self.set_porcentaje2(porcentaje)
        }

        fn set_porcentaje2(&mut self, porcentaje: u32) -> Result<String, String> {
            let res = self.check_permisos();
            match res.0 {
                Permiso::Owner => {
                    self.porcentaje_descuento = porcentaje;
                    Ok(res.1)
                }
                _ => Err(res.2),
            }
        }
        ///get_porcentaje puede ser llamado por el staff, muestra el porcentaje de descuento para los pagos consecutivos
        #[ink(message)]
        pub fn get_porcentaje(&self) -> Result<u32, String> {
            self.get_porcentaje2()
        }

        fn get_porcentaje2(&self) -> Result<u32, String> {
            let res = self.check_permisos();
            match res.0 {
                Permiso::Ninguno => Err(res.2),
                _ => Ok(self.porcentaje_descuento),
            }
        }
        ///set_cantidad_pagos_consecutivos solo puede ser llamado por el owner, cambia la cantidad de pagos consecutivos requeridos para acceder al descuento
        #[ink(message)]
        pub fn set_cantidad_pagos_consecutivos(
            &mut self,
            cantidad_pagos_consecutivos: u8,
        ) -> Result<String, String> {
            self.set_cantidad_pagos_consecutivos2(cantidad_pagos_consecutivos)
        }

        fn set_cantidad_pagos_consecutivos2(
            &mut self,
            cantidad_pagos_consecutivos: u8,
        ) -> Result<String, String> {
            let res = self.check_permisos();
            match res.0 {
                Permiso::Owner => {
                    self.cantidad_pagos_consecutivos = cantidad_pagos_consecutivos;
                    Ok(res.1)
                }
                _ => Err(res.2),
            }
        }
        ///get_cantidad_pagos_consecutivos puede ser llamado por el staff, muestra la cantidad de pagos consecutivos requeridos para acceder al descuento
        #[ink(message)]
        pub fn get_cantidad_pagos_consecutivos(&self) -> Result<u8, String> {
            self.get_cantidad_pagos_consecutivos2()
        }

        fn get_cantidad_pagos_consecutivos2(&self) -> Result<u8, String> {
            let res = self.check_permisos();
            match res.0 {
                Permiso::Ninguno => Err(res.2),
                _ => Ok(self.cantidad_pagos_consecutivos),
            }
        }
        ///get_precio_a puede ser llamado por el staff, muestra el precio de la categoria A
        #[ink(message)]
        pub fn get_precio_a(&self) -> Result<u32, String> {
            self.get_precio_a2()
        }

        fn get_precio_a2(&self) -> Result<u32, String> {
            let res = self.check_permisos();
            match res.0 {
                Permiso::Ninguno => Err(res.2),
                _ => Ok(self.precio_a),
            }
        }
        ///get_precio_b puede ser llamado por el staff, muestra el precio de la categoria B
        #[ink(message)]
        pub fn get_precio_b(&self) -> Result<u32, String> {
            self.get_precio_b2()
        }
        fn get_precio_b2(&self) -> Result<u32, String> {
            let res = self.check_permisos();
            match res.0 {
                Permiso::Ninguno => Err(res.2),
                _ => Ok(self.precio_b),
            }
        }
        ///get_precio_c puede ser llamado por el staff, muestra el precio de la categoria C
        #[ink(message)]
        pub fn get_precio_c(&self) -> Result<u32, String> {
            self.get_precio_c2()
        }

        fn get_precio_c2(&self) -> Result<u32, String> {
            let res = self.check_permisos();
            match res.0 {
                Permiso::Ninguno => Err(res.2),
                _ => Ok(self.precio_c),
            }
        }
        ///set_precio_a solo puede ser llamado por el owner, cambia el precio de la categoria A
        #[ink(message)]
        pub fn set_precio_a(&mut self, precio_a: u32) -> Result<String, String> {
            self.set_precio_a2(precio_a)
        }
        fn set_precio_a2(&mut self, precio_a: u32) -> Result<String, String> {
            let res = self.check_permisos();
            match res.0 {
                Permiso::Owner => {
                    self.precio_a = precio_a;
                    Ok(res.1)
                }
                _ => Err(res.2),
            }
        }
        ///set_precio_b solo puede ser llamado por el owner, cambia el precio de la categoria B
        #[ink(message)]
        pub fn set_precio_b(&mut self, precio_b: u32) -> Result<String, String> {
            self.set_precio_b2(precio_b)
        }
        fn set_precio_b2(&mut self, precio_b: u32) -> Result<String, String> {
            let res = self.check_permisos();
            match res.0 {
                Permiso::Owner => {
                    self.precio_b = precio_b;
                    Ok(res.1)
                }
                _ => Err(res.2),
            }
        }
        ///set_precio_c solo puede ser llamado por el owner, cambia el precio de la categoria C
        #[ink(message)]
        pub fn set_precio_c(&mut self, precio_c: u32) -> Result<String, String> {
            self.set_precio_c2(precio_c)
        }
        fn set_precio_c2(&mut self, precio_c: u32) -> Result<String, String> {
            let res = self.check_permisos();
            match res.0 {
                Permiso::Owner => {
                    self.precio_c = precio_c;
                    Ok(res.1)
                }
                _ => Err(res.2),
            }
        }
        ///Agregar staff solo puede ser llamado por el owner, agrega un address a la lista de staff
        #[ink(message)]
        pub fn agregar_staff(&mut self, acc: AccountId) -> Result<String, String> {
            let res = self.check_permisos();
            match res.0 {
                Permiso::Owner => {
                    self.staff.push(acc);
                    Ok(res.1)
                }
                _ => Err(res.2),
            }
        }
        ///agregar_socio puede ser llamado por el staff, agrega un socio nuevo, requiere que sea categoria A o C sin opcion de Actividad o categoria B con una opcion de actividad
        /// al crearlo crea tambien un nuevo pago pendiente que agrega al vector de pagos, con vencimiento a diez dias luego del momento de crear el socio
        #[ink(message)]
        pub fn agregar_socio(
            &mut self,
            dni: u32,
            nombre: String,
            categoria: String,
            actividad: Option<Actividades>,
        ) -> Result<String, String> {
            self.agregar_socio2(dni, nombre, categoria, actividad)
        }
        fn agregar_socio2(
            &mut self,
            dni: u32,
            nombre: String,
            cat: String,
            actividad: Option<Actividades>,
        ) -> Result<String, String> {
            let res = self.check_permisos();
            match res.0 {
                Permiso::Ninguno => Err(res.2),
                _ => {
                    let info: InfoCat;
                    let mut categ = Categorias::default();
                    let socio: Socio;
                    let pago: Pago;
                    let mut precio = 0;
                    let res: Result<String, String>;
                    let mut esta = false;

                    for i in &self.datos_socios {
                        if i.dni == dni {
                            esta = true;
                            break;
                        }
                    }
                    if !esta {
                        match InfoCat::new(cat.clone(), self, actividad) {
                            Ok(a) => {
                                info = a;
                                res = Ok("Usuario agregado correctamente".to_string());
                                if cat.eq("A") || cat.eq("a") {
                                    categ = Categorias::A(info);
                                    precio = self.precio_a;
                                } else if cat.eq("B") || cat.eq("b") {
                                    categ = Categorias::B(info);
                                    precio = self.precio_b;
                                } else if cat.eq("C") || cat.eq("c") {
                                    categ = Categorias::C(info);
                                    precio = self.precio_c;
                                }

                                socio = Socio::new(dni, nombre, categ);
                                pago = Pago::new(
                                    precio,
                                    socio.clone(),
                                    self.env().block_timestamp() + 864_000_000,
                                );
                                self.datos_socios.push(socio);
                                self.registro_pagos.push(pago);
                            }
                            Err(e) => res = Err(e),
                        }
                    } else {
                        res = Err("Este socio ya existe".to_string())
                    }
                    res
                }
            }
        }
        ///get_socio es solo para uso interno, devuelve Some(socio) si se encuentra en el vector
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
        ///get_proximo_pago puede ser llamado por el staff, muestra el proximo pago de un socio pasando su dni como argumento
        /// aplicar para descuento por pagos consecutivos, muestra el monto con descuento y la variable con_descuento como verdadera (pero no modifica estos valores del vector, los mismos se modifican solo al momento de registrar el pago)
        #[ink(message)]
        pub fn get_proximo_pago(&self, dni: u32) -> Result<Pago, String> {
            self.get_proximo_pago2(dni)
        }
        fn get_proximo_pago2(&self, dni: u32) -> Result<Pago, String> {
            let res = self.check_permisos();
            let mut hecho = false;
            match res.0 {
                Permiso::Ninguno => Err(res.2),
                _ => {
                    let mut res = Pago::default();
                    for i in &self.registro_pagos {
                        if i.socio.dni == dni && !i.pagado {
                            let mut aux = i.clone();
                            if self.si_descuento(dni) {
                                let mut pago_aux = aux;
                                pago_aux.monto = pago_aux.monto * self.porcentaje_descuento / 100;
                                pago_aux.con_descuento = true;
                                aux = pago_aux;
                            }
                            hecho = true;
                            res = aux;
                            break;
                        }
                    }
                    if hecho {
                        Ok(res)
                    } else {
                        Err("Usuario inexistente".to_string())
                    }
                }
            }
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
            while i > 0 {
                i -= 1;
                if !aux[i].con_descuento && !aux[i].fuera_de_termino(self.env().block_timestamp()) {
                    cont += 1;
                } else {
                    break;
                }

                if cont >= self.cantidad_pagos_consecutivos {
                    es = true;
                    break;
                }
            }
            es
        }
        ///todos pueden llamar a get_timestamp, muestra el timestamp actual
        #[ink(message)]
        pub fn get_timestamp(&self) -> u64 {
            self.env().block_timestamp()
        }
        ///todos pueden llamar a get_address, muestra el address del caller
        #[ink(message)]
        pub fn get_address(&self) -> AccountId {
            self.env().caller()
        }
        ///todos pueden llamar a get_politica, muestra si la politica de permisos esta activada
        #[ink(message)]
        pub fn get_politica(&self) -> bool {
            self.permisos_privados
        }
        ///marcar pago pendiente como pagado, al hacerlo se crea el proximo pago 30 dias despues del que se pagó
        #[ink(message)]
        pub fn registrar_pago(&mut self, dni: u32, monto_pagado: u32) -> Result<String, String> {
            self.registrar_pago2(dni, monto_pagado)
        }
        fn registrar_pago2(&mut self, dni: u32, monto_pagado: u32) -> Result<String, String> {
            let mut hecho = Err("Pago fallido".to_string());
            let timestamp = self.env().block_timestamp();
            let vuelto;
            if let Some(socio) = self.get_socio(dni) {
                for i in 0..self.registro_pagos.len() {
                    if socio.dni == self.registro_pagos[i].socio.dni
                        && self.registro_pagos[i].pagado == false
                    {
                        let mut pago_aux = self.registro_pagos.remove(i);

                        if self.si_descuento(dni)
                            && monto_pagado
                                >= pago_aux.monto * (100 - self.porcentaje_descuento) / 100
                        {
                            let monto = match socio.clone().categoria {
                                Categorias::A(_) => self.precio_a,
                                Categorias::B(_) => self.precio_b,
                                Categorias::C(_) => self.precio_c,
                            };
                            let pago = Pago::new(
                                monto,
                                socio.clone(),
                                pago_aux.vencimiento + 2_592_000_000,
                            );
                            self.registro_pagos.push(pago);
                            pago_aux.monto =
                                pago_aux.monto * (100 - self.porcentaje_descuento) / 100;
                            pago_aux.pagado = true;
                            pago_aux.con_descuento = true;
                            pago_aux.fecha_de_pago = Some(timestamp);
                            vuelto = monto_pagado - pago_aux.monto;
                            hecho = Ok(format!(
                                "Pago registrado exitosamente, por haber pagado sin atrasos las ultimas {} veces cuenta con un descuento del {}%, el vuelto es: {}",self.cantidad_pagos_consecutivos,self.porcentaje_descuento,
                                vuelto
                            ));
                        } else if pago_aux.monto <= monto_pagado {
                            let monto = match socio.clone().categoria {
                                Categorias::A(_) => self.precio_a,
                                Categorias::B(_) => self.precio_b,
                                Categorias::C(_) => self.precio_c,
                            };
                            let pago = Pago::new(
                                monto,
                                socio.clone(),
                                pago_aux.vencimiento + 2_592_000_000,
                            );
                            self.registro_pagos.push(pago);
                            pago_aux.pagado = true;
                            pago_aux.fecha_de_pago = Some(timestamp);
                            vuelto = monto_pagado - pago_aux.monto;
                            hecho = Ok(format!(
                                "Pago registrado exitosamente, el vuelto es: {}",
                                vuelto
                            ));
                        } else {
                            hecho = Err("Monto incorrecto".to_string());
                        }
                        self.registro_pagos.insert(i, pago_aux);
                        break;
                    }
                }
            } else {
                hecho = Err("Socio inexistente".to_string());
            }

            hecho
        }
        ///consultar todos los pagos de un socio con el dni, o consultar todos los pagos sin dni
        #[ink(message)]
        pub fn consulta_pagos(&self, dni: Option<u32>) -> Result<Vec<Pago>, String> {
            self.consulta_pagos2(dni)
        }
        fn consulta_pagos2(&self, dni: Option<u32>) -> Result<Vec<Pago>, String> {
            let res = self.check_permisos();
            match res.0 {
                Permiso::Ninguno => Err(res.2),
                _ => {
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

                    Ok(aux)
                }
            }
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    #[allow(unused_must_use)]
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
            let info = InfoCat::default();
            let cat = Categorias::A(info);
            let socio = Socio::new(0, "".to_string(), cat);
            let s2 = socio.clone();
            assert_eq!(socio, s2)
        }
        #[ink::test]
        fn socio_clone_test2() {
            let info = InfoCat::default();
            let cat = Categorias::B(info);
            let socio = Socio::new(0, "".to_string(), cat);
            let s2 = socio.clone();
            assert_eq!(socio, s2)
        }
        #[ink::test]
        fn socio_clone_test3() {
            let info = InfoCat::default();
            let cat = Categorias::C(info);
            let socio = Socio::new(0, "".to_string(), cat);
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
        fn new_infocat_test() {
            let sis = Sistema::default();
            let res = InfoCat::new('a'.to_string(), &sis, Some(Actividades::default()));
            assert!(res==Err("Las entradas validas son A y C sin opcion de actividad, y B con opcion de actividad".to_string()))
        }
        #[ink::test]
        fn new_infocat_test1() {
            let mut sis = Sistema::default();
            sis.precio_a = 30;
            let info = InfoCat::new('a'.to_string(), &sis, None).unwrap();
            assert!(info.precio_actual == sis.precio_a && info.actividades.len() == 8);
        }
        #[ink::test]
        fn new_infocat_test2() {
            let mut sis = Sistema::default();
            sis.precio_b = 30;
            let info = InfoCat::new('b'.to_string(), &sis, Some(Actividades::default())).unwrap();
            assert!(info.precio_actual == sis.precio_b && info.actividades.len() == 1);
        }
        #[ink::test]
        fn new_infocat_test3() {
            let sis = Sistema::default();
            let res = InfoCat::new('b'.to_string(), &sis, None);
            assert!(res==Err("Las entradas validas son A y C sin opcion de actividad, y B con opcion de actividad".to_string()))
        }
        #[ink::test]
        fn new_infocat_test4() {
            let sis = Sistema::default();
            let res = InfoCat::new('c'.to_string(), &sis, Some(Actividades::Hockey));
            assert!(res==Err("Las entradas validas son A y C sin opcion de actividad, y B con opcion de actividad".to_string()));
        }

        #[ink::test]
        fn new_infocat_test5() {
            let mut sis = Sistema::default();
            sis.precio_a = 30;
            let info = InfoCat::new('c'.to_string(), &sis, None).unwrap();
            assert!(info.precio_actual == sis.precio_c && info.actividades.len() == 1);
        }
        #[ink::test]
        fn new_infocat_test6() {
            let sis = Sistema::default();
            let res = InfoCat::new('h'.to_string(), &sis, Some(Actividades::Futbol));
            assert!(res==Err("Las entradas validas son A y C sin opcion de actividad, y B con opcion de actividad".to_string()))
        }
        #[ink::test]
        fn new_infocat_test7() {
            let sis = Sistema::default();
            let res = InfoCat::new('z'.to_string(), &sis, None);
            assert!(res==Err("Las entradas validas son A y C sin opcion de actividad, y B con opcion de actividad".to_string()))
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
        fn get_porcentaje_test() {
            let mut sis = Sistema::new(1, 2, 3, 4, 5);
            sis.permisos_privados = false;
            assert!(sis.get_porcentaje() == Ok(4))
        }
        #[ink::test]
        fn get_cantidad_pagos_consecutivos_test() {
            let mut sis = Sistema::new(1, 2, 3, 4, 5);
            sis.permisos_privados = false;
            assert!(sis.get_cantidad_pagos_consecutivos() == Ok(5))
        }
        #[ink::test]
        fn get_precio_a_test() {
            let mut sis = Sistema::new(1, 2, 3, 4, 5);
            sis.permisos_privados = false;
            assert!(sis.get_precio_a() == Ok(1))
        }
        #[ink::test]
        fn get_precio_b_test() {
            let mut sis = Sistema::new(1, 2, 3, 4, 5);
            sis.permisos_privados = false;
            assert!(sis.get_precio_b() == Ok(2))
        }
        #[ink::test]
        fn get_precio_c_test() {
            let mut sis = Sistema::new(1, 2, 3, 4, 5);
            sis.permisos_privados = false;
            assert!(sis.get_precio_c() == Ok(3))
        }
        #[ink::test]
        fn set_precio_a_test() {
            let mut sis = Sistema::default();
            sis.permisos_privados = false;
            sis.set_precio_a(1);
            assert!(sis.precio_a == 1)
        }
        #[ink::test]
        fn set_precio_b_test() {
            let mut sis = Sistema::default();
            sis.set_precio_b(1);
            assert!(sis.precio_b == 1)
        }
        #[ink::test]
        fn set_precio_c_test() {
            let mut sis = Sistema::default();
            sis.set_precio_c(1);
            assert!(sis.precio_c == 1)
        }
        #[ink::test]
        fn agregar_socio_test() {
            let mut sis = Sistema::default();
            let dni = 1;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            let mut esta = false;
            if let Ok(_) = sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad) {
                let aux = &sis.datos_socios[0];
                if aux.dni == dni && aux.nombre.eq(&nombre) {
                    esta = true
                }
            }
            assert!(
                esta /*&& sis.registro_pagos[0].vencimiento > 0 */ && sis.registro_pagos[0].pagado == false
            )
        }
        #[ink::test]
        fn agregar_socio_test2() {
            let mut sis = Sistema::default();
            let dni = 1;
            let nombre = "b".to_string();
            let categoria = "B".to_string();
            let actividad = Some(Actividades::Futbol);
            let mut esta = false;
            if let Ok(_) = sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad) {
                let aux = &sis.datos_socios[0];
                if aux.dni == dni && aux.nombre.eq(&nombre) {
                    esta = true
                }
            }
            assert!(
                esta /*&& sis.registro_pagos[0].vencimiento > 0 */ && sis.registro_pagos[0].pagado == false
            )
        }
        #[ink::test]
        fn agregar_socio_test3() {
            let mut sis = Sistema::default();
            let dni = 1;
            let nombre = "c".to_string();
            let categoria = "C".to_string();
            let actividad = None;
            let mut esta = false;
            if let Ok(_) = sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad) {
                let aux = &sis.datos_socios[0];
                if aux.dni == dni && aux.nombre.eq(&nombre) {
                    esta = true
                }
            }
            assert!(
                esta /*&& sis.registro_pagos[0].vencimiento > 0 */ && sis.registro_pagos[0].pagado == false
            )
        }

        #[ink::test]
        fn agregar_socio_test4() {
            let mut sis = Sistema::default();
            let dni = 1;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            if let Ok(_) =
                sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad.clone())
            {
            }
            let res = sis.agregar_socio(dni, nombre.clone(), categoria, actividad);
            let e = "Este socio ya existe".to_string();

            assert!(res == Err(e))
        }

        #[ink::test]
        fn agregar_socio_test5() {
            let mut sis = Sistema::default();
            let dni = 1;
            let nombre = "b".to_string();
            let categoria = "B".to_string();
            let actividad = None;
            if let Ok(_) =
                sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad.clone())
            {
            }
            let res = sis.agregar_socio(dni, nombre.clone(), categoria, actividad);
            let e = "Las entradas validas son A y C sin opcion de actividad, y B con opcion de actividad".to_string();

            assert!(res == Err(e))
        }
        #[ink::test]
        fn agregar_socio_test6() {
            let mut sis = Sistema::default();
            let dni = 1;
            let nombre = "c".to_string();
            let categoria = "C".to_string();
            let actividad = None;
            if let Ok(_) =
                sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad.clone())
            {
            }
            assert!(
                sis.registro_pagos[0].vencimiento >= sis.env().block_timestamp() + 864_000_000
                    && sis.registro_pagos[0].vencimiento
                        < sis.env().block_timestamp() + 864_060_000
            )
        }
        #[ink::test]
        fn get_socio_test() {
            let mut sis = Sistema::default();
            let dni = 1;
            let mut socio = None;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            if let Ok(_) =
                sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad.clone())
            {
                socio = sis.get_socio(dni);
            }
            assert!(socio.clone().unwrap().dni == dni && socio.unwrap().nombre.eq(&nombre))
        }
        #[ink::test]
        fn get_socio_test2() {
            let mut sis = Sistema::default();
            let dni = 1;
            let mut socio = Some(Socio::default());
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            if let Ok(_) =
                sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad.clone())
            {
                socio = sis.get_socio(3);
            }
            assert!(socio == None)
        }
        #[ink::test]
        fn get_proximo_pago_test() {
            let mut sis = Sistema::default();
            let dni = 1;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            let mut aux = Pago::default();
            let mut pago = Pago::default();
            pago.pagado = false;
            if let Ok(_) =
                sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad.clone())
            {
                sis.registro_pagos.push(pago.clone());
                if let Ok(a) = sis.get_proximo_pago(dni) {
                    aux = a;
                }
            }

            assert!(aux.pagado == false /*&&aux.unwrap().vencimiento!=0 */)
        }
        #[ink::test]
        fn registrar_pago_test() {
            let mut sis = Sistema::default();
            sis.set_cantidad_pagos_consecutivos(2);
            let dni = 1;
            let mut venc = 0;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            if let Ok(_) =
                sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad.clone())
            {
                if let Ok(_) = sis.registrar_pago(dni, sis.precio_a) {
                    if let Ok(_) = sis.registrar_pago(dni, sis.precio_a) {
                        venc = sis.get_proximo_pago(dni).unwrap().vencimiento;
                    }
                }
            }

            assert!(
                sis.get_proximo_pago(dni).unwrap().vencimiento >= venc
                    && sis.registro_pagos.len() == 3
            )
        }
        #[ink::test]
        fn registrar_pago_test2() {
            let mut sis = Sistema::default();
            sis.set_cantidad_pagos_consecutivos(2);
            let dni = 1;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            if let Ok(_) =
                sis.agregar_socio2(dni, nombre.clone(), categoria.clone(), actividad.clone())
            {
                sis.set_precio_a(2000);
                if let Err(a) = sis.registrar_pago(dni, 1000) {
                    assert!(sis.registro_pagos.len() == 1 && a == "Monto incorrecto".to_string())
                }
            }
        }
        #[ink::test]
        fn registrar_pago_test3() {
            let mut sis = Sistema::default();
            sis.set_cantidad_pagos_consecutivos(2);
            let dni = 1;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            if let Ok(_) =
                sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad.clone())
            {
                if let Err(a) = sis.registrar_pago(2, sis.precio_a) {
                    assert!(sis.registro_pagos.len() == 1 && a == "Socio inexistente".to_string())
                }
            }
        }
        #[ink::test]
        fn registrar_pago_test4() {
            let mut sis = Sistema::default();
            sis.set_cantidad_pagos_consecutivos(1);
            let dni = 1;
            let mut venc = 0;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            if let Ok(_) =
                sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad.clone())
            {
                if let Ok(_) = sis.registrar_pago(dni, sis.precio_a) {
                    if let Ok(_) = sis.registrar_pago(dni, sis.precio_a) {
                        venc = sis.get_proximo_pago(dni).unwrap().vencimiento;
                    }
                }
            }

            assert!(
                sis.get_proximo_pago(dni).unwrap().vencimiento >= venc
                    && sis.registro_pagos.len() == 3
                    && sis.registro_pagos[1].con_descuento == true
            )
        }
        #[ink::test]
        fn registrar_pago_test5() {
            let mut sis = Sistema::default();
            sis.set_cantidad_pagos_consecutivos(1);
            let dni = 1;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            let es_desc: bool;
            if let Ok(_) =
                sis.agregar_socio2(dni, nombre.clone(), categoria.clone(), actividad.clone())
            {
                sis.set_precio_a(2000);
                if let Ok(_) = sis.registrar_pago(dni, 2000) {
                    es_desc = sis.si_descuento(dni);
                    if let Err(e) = sis.registrar_pago(dni, 300) {
                        assert!(
                            es_desc
                                && sis.registro_pagos.len() == 2
                                && e == "Monto incorrecto".to_string()
                        )
                    }
                }
            }
        }
        #[ink::test]
        fn registrar_pago_test6() {
            let mut sis = Sistema::default();
            sis.set_cantidad_pagos_consecutivos(1);
            let dni = 1;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            if let Ok(_) =
                sis.agregar_socio2(dni, nombre.clone(), categoria.clone(), actividad.clone())
            {
                sis.set_precio_a(2000);
                if let Ok(_) = sis.registrar_pago(dni, 2000) {
                    assert!(
                        sis.registro_pagos[1].vencimiento
                            >= sis.registro_pagos[0].vencimiento + 2_592_000_000
                            && sis.registro_pagos[1].vencimiento
                                < sis.registro_pagos[0].vencimiento + 2_592_060_000
                    )
                }
            }
        }
        #[ink::test]
        fn si_descuento_test() {
            let mut sis = Sistema::default();
            sis.set_cantidad_pagos_consecutivos(1);
            let dni = 1;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            if let Ok(_) =
                sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad.clone())
            {
                if let Ok(_) = sis.registrar_pago(dni, sis.precio_a) {}
            }
            assert!(sis.si_descuento(dni))
        }
        #[ink::test]
        fn si_descuento_test2() {
            let mut sis = Sistema::default();
            sis.set_cantidad_pagos_consecutivos(2);
            let dni = 1;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            if let Ok(_) =
                sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad.clone())
            {
                if let Ok(_) = sis.registrar_pago(dni, sis.precio_a) {}
            }
            assert!(!sis.si_descuento(dni))
        }
        #[ink::test]
        fn si_descuento_test3() {
            let mut sis = Sistema::default();
            sis.set_cantidad_pagos_consecutivos(3);
            let dni = 1;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let actividad = None;
            if let Ok(_) =
                sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad.clone())
            {
                if let Ok(_) = sis.registrar_pago(dni, sis.precio_a) {
                    if let Ok(_) = sis.registrar_pago(dni, sis.precio_a) {
                        if let Ok(_) = sis.registrar_pago(dni, sis.precio_a) {}
                    }
                }
            }
            assert!(sis.si_descuento(dni))
        }
        #[ink::test]
        fn consulta_pagos_test() {
            let mut sis = Sistema::default();
            sis.set_cantidad_pagos_consecutivos(1);
            let mut dni = 1;
            let mut nombre = "a".to_string();
            let mut categoria = "A".to_string();
            let mut esta_solo = true;
            let actividad = None;
            let mut vec = Vec::new();
            if let Ok(_) =
                sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad.clone())
            {
                dni = 2;
                nombre = "b".to_string();
                categoria = "B".to_string();
                let actividad = Some(Actividades::Futbol);
                if let Ok(_) = sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad)
                {
                    if let Ok(_) = sis.registrar_pago(1, sis.precio_a) {
                        vec = match sis.consulta_pagos(Some(1)) {
                            Ok(a) => a,
                            Err(e) => panic!("{}", e),
                        };
                        for i in &vec {
                            if i.socio.dni != 1 {
                                esta_solo = false;
                            }
                        }
                    }
                }
            }

            assert!(esta_solo && vec.len() == 2)
        }
        #[ink::test]
        fn consulta_pagos_test2() {
            let mut sis = Sistema::default();
            sis.set_cantidad_pagos_consecutivos(1);
            let dni = 1;
            let nombre = "a".to_string();
            let categoria = "A".to_string();
            let mut esta_solo = true;
            let mut vec = Vec::new();
            let actividad = None;
            if let Ok(_) =
                sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad.clone())
            {
                let dni = 2;
                let nombre = "b".to_string();
                let categoria = "B".to_string();
                let actividad = Some(Actividades::Futbol);
                if let Ok(_) = sis.agregar_socio(dni, nombre.clone(), categoria.clone(), actividad)
                {
                    if let Ok(_) = sis.registrar_pago(1, sis.precio_a) {
                        vec = match sis.consulta_pagos(None) {
                            Ok(a) => a,
                            Err(e) => panic!("{}", e),
                        };
                        for i in &vec {
                            if i.socio.dni != 1 {
                                esta_solo = false;
                            }
                        }
                    }
                }
            }

            assert!(!esta_solo && vec.len() == 3)
        }
        /*------------------------------------------------------- */
        use ink::codegen::Env;
        #[ink::test]
        fn check_permisos_test() {
            let mut sis = Sistema::default();
            sis.permisos_privados = true;
            let res = sis.check_permisos();
            assert!(match res.0 {
                Permiso::Ninguno => true,
                _ => false,
            })
        }
        #[ink::test]
        fn check_permisos_test2() {
            let mut sis = Sistema::default();
            sis.staff.push(sis.env().caller());
            sis.permisos_privados = true;
            let res = sis.check_permisos();
            assert!(match res.0 {
                Permiso::Staff => true,
                _ => false,
            })
        }
        #[ink::test]
        fn check_permisos_test3() {
            let mut sis = Sistema::default();
            sis.set_owner();
            sis.permisos_privados = true;
            let res = sis.check_permisos();
            assert!(match res.0 {
                Permiso::Owner => true,
                _ => false,
            })
        }
    }
}
