use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Fecha {
    año: u32,
    mes: u8,
    dia: u8,
    arreglo_meses: [u8; 12],
}
impl Fecha {
    /*➢ new: que pasando los parámetros correspondientes, crea una Fecha y la retorna. */
    pub fn new(año: u32, mes: u8, dia: u8) -> Fecha {
        let arreglo_meses = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let f = Fecha {
            año,
            mes,
            dia,
            arreglo_meses,
        };
        if f.es_fecha_valida() {
            f
        } else {
            panic!("Fecha Invalida en {} {} {}", año, mes, dia);
        }
    }

    /*➢ es_fecha_valida: retorna true si es una fecha valida, false caso contrario.//tenga en
    cuenta los años bisiestos también. */
    pub fn es_fecha_valida(&self) -> bool {
        let mut res = false;
        if self.año > 0 && self.dia > 0 && self.mes > 0 && self.mes <= 12 {
            if self.dia <= self.arreglo_meses[self.mes as usize - 1] as u8 {
                res = true;
            } else if self.mes == 2 && self.dia == 29 && self.es_bisiesto() {
                res = true;
            }
        }
        res
    }
    /*➢ es_bisiesto: retorna true si el año de la fecha pertenece a un año bisiesto. */
    pub fn es_bisiesto(&self) -> bool {
        let mut res = false;
        if self.año % 4 == 0 && self.año % 100 != 0 {
            res = true;
        }
        if self.año % 100 == 0 && self.año % 400 != 0 {
            res = false;
        }
        if self.año % 100 == 0 && self.año % 400 == 0 {
            res = true;
        }
        res
    }
    /*➢ sumar_dias(dias): suma la cantidad de días a la fecha, modificándose*/
    pub fn sumar_dias(&mut self, dias: usize) {
        let mut aux = self.dia as usize;
        aux += dias;
        while aux > self.arreglo_meses[(self.mes - 1) as usize] as usize {
            aux -= self.arreglo_meses[(self.mes - 1) as usize] as usize;
            if self.mes == 2 && self.es_bisiesto() {
                aux -= 1;
            }
            if self.mes < 12 {
                self.mes += 1;
            } else {
                self.mes = 1;
                self.año += 1;
            }
        }
        self.dia = aux as u8;
    }
    /*
    ➢ restar_dias(dias): resta la cantidad de días a la fecha, modificándose */
    pub fn restar_dias(&mut self, dias: isize) {
        let mut aux: isize = self.dia as isize;
        aux -= dias;
        while aux < 1 {
            aux += self.arreglo_meses[(self.mes - 1) as usize] as isize;
            if self.mes == 2 && self.es_bisiesto() {
                aux += 1;
            }
            if self.mes > 1 {
                self.mes -= 1;
            } else {
                self.mes = 12;
                self.año -= 1;
            }
        }
        self.dia = aux as u8;
    }
    /*
    ➢ es_mayor(una_fecha): que retorna true si la fecha que recibe el mensaje es mayor a
    la fecha pasada por parámetro.. */
    pub fn es_mayor(&self, f: &Fecha) -> bool {
        let mut res = false;
        if f.año < self.año {
            res = true;
        } else if f.año == self.año && f.mes < self.mes {
            res = true;
        } else if f.año == self.año && f.mes == self.mes && f.dia < self.dia {
            res = true;
        }
        res
    }
}

/*------------------------------------------------------------------------ */
#[derive(Debug, Default, Serialize, Deserialize)]

pub struct Mp {
    cvu: usize,
    url: String,
    id_transaccion: String,
    monto: f64,
}
impl Mp {
    //Crear MP
    pub fn new(cvu: usize, url: String, id_transaccion: String, monto: f64) -> Mp {
        Mp {
            cvu,
            url,
            id_transaccion,
            monto,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Tarjetas {
    #[default]
    Visa,
    Mastercard,
    American,
    Cabal,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Bancos {
    #[default]
    Santander,
    Provincia,
    Naranja,
    Galicia,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CredCard {
    cbu: usize,
    empresa_de_pago: Tarjetas,
    banco: Bancos,
    id_transaccion: String,
    monto: f64,
}
impl CredCard {
    //Crear tarjeta de credito
    pub fn new(
        cbu: usize,
        empresa_de_pago: Tarjetas,
        banco: Bancos,
        id_transaccion: String,
        monto: f64,
    ) -> CredCard {
        CredCard {
            cbu,
            empresa_de_pago,
            banco,
            id_transaccion,
            monto,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Transfer {
    cbu: usize,
    destino: usize,
    banco: Bancos,
    id_transaccion: String,
    monto: f64,
}
impl Transfer {
    //Crear Transferencia
    pub fn new(
        cbu: usize,
        destino: usize,
        banco: Bancos,
        id_transaccion: String,
        monto: f64,
    ) -> Transfer {
        Transfer {
            cbu,
            destino,
            banco,
            id_transaccion,
            monto,
        }
    }
}
#[derive(Debug, Default, Serialize, Deserialize)]

pub struct Crypto {
    red: String,
    token: String,
    id_transaccion: String,
    monto: f64,
}
impl Crypto {
    //Crear Cripto
    pub fn new(red: String, token: String, id_transaccion: String, monto: f64) -> Crypto {
        Crypto {
            red,
            token,
            id_transaccion,
            monto,
        }
    }
}
#[derive(Debug, Default, Serialize, Deserialize)]

pub struct Cash {
    monto: f64,
}
impl Cash {
    pub fn new(monto: f64) -> Cash {
        Cash { monto }
    }
}
trait SetMonto {
    fn set_monto(&mut self, monto: f64);
}
trait GetMonto {
    fn get_monto(&self) -> f64;
}
impl GetMonto for MediosPago {
    fn get_monto(&self) -> f64 {
        match self {
            MediosPago::Cripto(a) => a.monto,
            MediosPago::Efectivo(a) => a.monto,
            MediosPago::MercadoPago(a) => a.monto,
            MediosPago::TarjetaCredito(a) => a.monto,
            MediosPago::Transferencia(a) => a.monto,
        }
    }
}
impl SetMonto for Cash {
    fn set_monto(&mut self, monto: f64) {
        self.monto = monto;
    }
}
impl SetMonto for Transfer {
    fn set_monto(&mut self, monto: f64) {
        self.monto = monto;
    }
}
impl SetMonto for Crypto {
    fn set_monto(&mut self, monto: f64) {
        self.monto = monto;
    }
}
impl SetMonto for CredCard {
    fn set_monto(&mut self, monto: f64) {
        self.monto = monto;
    }
}
impl SetMonto for Mp {
    fn set_monto(&mut self, monto: f64) {
        self.monto = monto;
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum MediosPago {
    Cripto(Crypto),
    Efectivo(Cash),
    MercadoPago(Mp),
    TarjetaCredito(CredCard),
    Transferencia(Transfer),
}
impl Default for MediosPago {
    fn default() -> Self {
        let cr = Crypto::default();
        MediosPago::Cripto(cr)
    }
}
impl SetMonto for MediosPago {
    fn set_monto(&mut self, monto: f64) {
        match self {
            MediosPago::Cripto(a) => a.set_monto(monto),
            MediosPago::Efectivo(a) => a.set_monto(monto),
            MediosPago::MercadoPago(a) => a.set_monto(monto),
            MediosPago::TarjetaCredito(a) => a.set_monto(monto),
            MediosPago::Transferencia(a) => a.set_monto(monto),
        }
    }
}

impl PartialEq for MediosPago {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&MediosPago::MercadoPago(_), &MediosPago::MercadoPago(_)) => true,
            (&MediosPago::Cripto(_), &MediosPago::Cripto(_)) => true,
            (&MediosPago::Efectivo(_), &MediosPago::Efectivo(_)) => true,
            (&MediosPago::TarjetaCredito(_), &MediosPago::TarjetaCredito(_)) => true,
            (&MediosPago::Transferencia(_), &MediosPago::Transferencia(_)) => true,
            _ => false,
        }
    }
}
impl Eq for MediosPago {}
#[derive(Debug, Default, Serialize, Deserialize)]

pub struct Suscriptor {
    id: String,
}
impl Suscriptor {
    //Crear suscriptor
    pub fn new(id: String) -> Suscriptor {
        Suscriptor { id }
    }
}
#[derive(Debug, Default, Serialize, Deserialize)]

pub struct Suscripcion {
    costo_mensual: f64,
    duracion_meses: u8,
    fecha_inicio: Fecha,
    tipo_suscripcion: Option<CategoriaSuscripcion>,
    datos_suscriptor: Suscriptor,
    medio_pago: MediosPago,
}
#[derive(Debug, Serialize, Deserialize)]

pub enum CategoriaSuscripcion {
    Basic,
    Clasic,
    Super,
}

impl Suscripcion {
    //Crear nuevo suscriptor
    pub fn new(
        duracion_meses: u8,
        fecha_inicio: Fecha,
        tipo_suscripcion: Option<CategoriaSuscripcion>,
        datos_suscriptor: Suscriptor,
        medio_pago: MediosPago,
    ) -> Suscripcion {
        let costo_mensual = medio_pago.get_monto();
        Suscripcion {
            costo_mensual,
            duracion_meses,
            fecha_inicio,
            tipo_suscripcion,
            datos_suscriptor,
            medio_pago,
        }
    }
    pub fn set_monto(&mut self, monto: f64) {
        self.costo_mensual = monto;
        self.medio_pago.set_monto(monto);
    }
    pub fn upgrade_subscription(&mut self) {
        match &self.tipo_suscripcion {
            Some(a) => match a {
                CategoriaSuscripcion::Basic => {
                    self.tipo_suscripcion = Some(CategoriaSuscripcion::Clasic)
                }
                CategoriaSuscripcion::Clasic => {
                    self.tipo_suscripcion = Some(CategoriaSuscripcion::Super)
                }
                CategoriaSuscripcion::Super => (),
            },
            None => self.tipo_suscripcion = Some(CategoriaSuscripcion::Basic),
        }
    }
    pub fn downgrade_subscription(&mut self) {
        match &self.tipo_suscripcion {
            Some(a) => match a {
                CategoriaSuscripcion::Super => {
                    self.tipo_suscripcion = Some(CategoriaSuscripcion::Clasic)
                }
                CategoriaSuscripcion::Clasic => {
                    self.tipo_suscripcion = Some(CategoriaSuscripcion::Basic)
                }
                CategoriaSuscripcion::Basic => {
                    self.tipo_suscripcion = None;
                }
            },
            None => (),
        }
    }
    pub fn cancel_subscription(&mut self) {
        self.tipo_suscripcion = None;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamingRust<'a> {
    suscripciones: Vec<Suscripcion>,
    path: &'a str,
}
fn crear_file<'a>(path: &'a str, escritura: &impl Serialize) {
    // File::create(path).expect("Error");
    let file = OpenOptions::new().write(true).create(true).open(path);
    match file {
        Ok(mut a) => {
            if let Err(e) = writeln!(a, "{}", serde_json::to_string_pretty(escritura).unwrap()) {
                panic!("Error al escribir porque {}", e);
            }
        }
        Err(e) => panic!("No se pudo escribir porque {}", e),
    }
}
fn leer_suscripciones_file<'a>(path: &'a str) -> Vec<Suscripcion> {
    let mut file = OpenOptions::new().read(true).open(path).unwrap();
    let mut buf = String::new();
    if let Err(e) = file.read_to_string(&mut buf) {
        panic!("No se pudo leer porque {}", e);
    }
    let mut res: Vec<Suscripcion> = Vec::new();
    match serde_json::from_str(&buf) {
        Ok(a) => res = a,
        Err(_) => (),
    }

    res
}
fn actualiza_monto_max<'a>(susc: &'a Suscripcion, arr: &mut [f64; 5]) {
    match &susc.medio_pago {
        MediosPago::Cripto(_) => arr[0] += susc.costo_mensual,
        MediosPago::Efectivo(_) => arr[1] += susc.costo_mensual,
        MediosPago::MercadoPago(_) => arr[2] += susc.costo_mensual,
        MediosPago::TarjetaCredito(_) => arr[3] += susc.costo_mensual,
        MediosPago::Transferencia(_) => arr[4] += susc.costo_mensual,
    }
}
fn actualiza_arreglo_medios_pago(susc: &Suscripcion, arr: &mut [i32; 5]) {
    match susc.medio_pago {
        MediosPago::Cripto(_) => {
            if arr[0] == -1 {
                arr[0] = 1
            } else {
                arr[0] += 1
            }
        }
        MediosPago::Efectivo(_) => {
            if arr[1] == -1 {
                arr[1] = 1
            } else {
                arr[1] += 1
            }
        }
        MediosPago::MercadoPago(_) => {
            if arr[2] == -1 {
                arr[2] = 1
            } else {
                arr[2] += 1
            }
        }
        MediosPago::TarjetaCredito(_) => {
            if arr[3] == -1 {
                arr[3] = 1
            } else {
                arr[3] += 1
            }
        }
        MediosPago::Transferencia(_) => {
            if arr[4] == -1 {
                arr[4] = 1
            } else {
                arr[4] += 1
            }
        }
    }
}

impl<'a> StreamingRust<'a> {
    //Crear el streaming, al crearlo toma las suscripciones directamente del archivo
    //(no toma las suscripciones del archivo si lo creo usando StreamingRust::default();)
    pub fn new() -> StreamingRust<'a> {
        let path = "src/Suscripciones.json";
        let suscripciones: Vec<Suscripcion> = leer_suscripciones_file(path);
        StreamingRust {
            suscripciones,
            path,
        }
    }
    //borrar las suscripciones del streaming en proceso y en el archivo
    pub fn clear_suscripciones(&mut self) {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.path);
        self.suscripciones.clear();
    }
    //toma las suscripciones desde el archivo antes de editar (por si acaso creamos usando StreamingRust::default();)
    pub fn push(&mut self, sus: Suscripcion) {
        self.suscripciones = leer_suscripciones_file(self.path);
        self.suscripciones.push(sus);
        crear_file(&self.path, &self.suscripciones);
    }

    pub fn get_medio_pago_mas_recaudado(&mut self) -> MediosPago {
        let iter_sus = self.suscripciones.iter();
        let mut arr = [0.0, 0.0, 0.0, 0.0, 0.0];
        let checkea = |sus: &Suscripcion, arr: &mut [f64; 5]| actualiza_monto_max(sus, arr);
        iter_sus.for_each(|x| checkea(x, &mut arr));
        let mut aux = (0, -10.0);
        for i in 0..arr.len() {
            if aux.1 < arr[i] {
                aux.1 = arr[i];
                aux.0 = i;
            }
        }
        match aux.0 {
            0 => MediosPago::Cripto(Crypto::default()),
            1 => MediosPago::Efectivo(Cash::default()),
            2 => MediosPago::MercadoPago(Mp::default()),
            3 => MediosPago::TarjetaCredito(CredCard::default()),
            4 => MediosPago::Transferencia(Transfer::default()),
            _ => panic!("Invalido"),
        }
    }
    pub fn get_medio_pago_mas_recaudado_activas(&mut self) -> MediosPago {
        self.suscripciones = leer_suscripciones_file(self.path);
        let iter_sus = self.suscripciones.iter();
        let mut arr = [0.0, 0.0, 0.0, 0.0, 0.0];
        let check = |sus: &Suscripcion, arr: &mut [f64; 5]| match sus.tipo_suscripcion {
            Some(_) => actualiza_monto_max(sus, arr),
            None => (),
        };
        iter_sus.for_each(|x| check(x, &mut arr));
        let mut aux = (0, 0.0);
        for i in 0..arr.len() {
            if aux.1 < arr[i] {
                aux.1 = arr[i];
                aux.0 = i;
            }
        }
        match aux.0 {
            0 => MediosPago::Cripto(Crypto::default()),
            1 => MediosPago::Efectivo(Cash::default()),
            2 => MediosPago::MercadoPago(Mp::default()),
            3 => MediosPago::TarjetaCredito(CredCard::default()),
            4 => MediosPago::Transferencia(Transfer::default()),
            _ => panic!("Invalido"),
        }
    }
    pub fn get_medio_mas_usado(&mut self) -> MediosPago {
        self.suscripciones = leer_suscripciones_file(self.path);
        let iter_sus = self.suscripciones.iter();
        let mut arr = [-1, -1, -1, -1, -1];
        let check =
            |susc: &Suscripcion, arr: &mut [i32; 5]| actualiza_arreglo_medios_pago(susc, arr);

        iter_sus.for_each(|x| check(x, &mut arr));
        let check2 = |x, y: &i32| -> bool {
            if *y > 0 {
                if x == y {
                    true
                } else {
                    false
                }
            } else {
                panic!("Vector sin sucripciones activas");
            }
        };

        match arr
            .iter()
            .position(|x| check2(x, arr.iter().max().unwrap()))
        {
            Some(a) => match a {
                0 => MediosPago::Cripto(Crypto::default()),
                1 => MediosPago::Efectivo(Cash::default()),
                2 => MediosPago::MercadoPago(Mp::default()),
                3 => MediosPago::TarjetaCredito(CredCard::default()),
                4 => MediosPago::Transferencia(Transfer::default()),
                _ => panic!("Invalido"),
            },
            None => panic!("Invalido"),
        }
    }
    pub fn get_medio_mas_usado_activas(&mut self) -> MediosPago {
        let iter_sus = self.suscripciones.iter();
        let mut arr = [-1, -1, -1, -1, -1];
        let check = |susc: &Suscripcion, arr: &mut [i32; 5]| match &susc.tipo_suscripcion {
            Some(_) => actualiza_arreglo_medios_pago(susc, arr),
            None => (),
        };
        iter_sus.for_each(|x| check(x, &mut arr));
        let check2 = |x, y: &i32| -> bool {
            if *y > 0 {
                if x == y {
                    true
                } else {
                    false
                }
            } else {
                panic!("Vector sin sucripciones activas");
            }
        };

        match arr
            .iter()
            .position(|x| check2(x, arr.iter().max().unwrap()))
        {
            Some(a) => match a {
                0 => MediosPago::Cripto(Crypto::default()),
                1 => MediosPago::Efectivo(Cash::default()),
                2 => MediosPago::MercadoPago(Mp::default()),
                3 => MediosPago::TarjetaCredito(CredCard::default()),
                4 => MediosPago::Transferencia(Transfer::default()),
                _ => panic!("Invalido"),
            },
            None => panic!("Invalido"),
        }
    }
    /*➢ Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones
    activas. */
    pub fn get_suscripcion_mas_contratada(&mut self) -> Option<CategoriaSuscripcion> {
        let iter_sus = self.suscripciones.iter();
        let mut arr = [0, 0, 0, 0];
        let check = |x: &Suscripcion, arr: &mut [i32; 4]| match &x.tipo_suscripcion {
            Some(a) => match a {
                CategoriaSuscripcion::Basic => {
                    arr[1] += 1;
                    arr[0] = 0
                }
                CategoriaSuscripcion::Clasic => {
                    arr[2] += 1;
                    arr[0] = 0
                }
                CategoriaSuscripcion::Super => {
                    arr[3] += 1;
                    arr[0] = 0
                }
            },
            None => arr[0] += 1,
        };
        iter_sus.for_each(|x| check(x, &mut arr));
        match arr.iter().position(|x| x == arr.iter().max().unwrap()) {
            Some(a) => match a {
                0 => None,
                1 => Some(CategoriaSuscripcion::Basic),
                2 => Some(CategoriaSuscripcion::Clasic),
                3 => Some(CategoriaSuscripcion::Super),
                _ => panic!("Invalid"),
            },
            None => panic!("Invalid"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_fecha_test() {
        let año = 2015;
        let mes = 10;
        let dia = 9;
        let f = Fecha::new(año, mes, dia);
        assert!(f.año == año && f.mes == mes && f.dia == dia);
    }
    #[test]
    #[should_panic]
    fn new_fecha1_test() {
        let año = 15;
        let mes = 15;
        let dia = 9;
        let f = Fecha::new(año, mes, dia);
        assert!(f.año == año && f.mes == mes && f.dia == dia);
    }
    #[test]
    #[should_panic]
    fn new_fecha2_test() {
        let año = 2020;
        let mes = 10;
        let dia = 32;
        let f = Fecha::new(año, mes, dia);
        assert!(f.año == año && f.mes == mes && f.dia == dia);
    }
    #[test]
    fn es_fecha_valida_test() {
        let año = 2020;
        let mes = 2;
        let dia = 29;
        let f = Fecha::new(año, mes, dia);
        f.es_fecha_valida();
    }
    #[test]
    fn es_bisiesto_test() {
        let año = 500;
        let mes = 1;
        let dia = 20;
        let f = Fecha::new(año, mes, dia);
        assert!(!f.es_bisiesto());
    }
    #[test]
    fn es_bisiesto2_test() {
        let año = 1600;
        let mes = 1;
        let dia = 20;
        let f = Fecha::new(año, mes, dia);
        assert!(f.es_bisiesto());
    }
    #[test]
    fn sumar_dias_test() {
        let año = 1600;
        let mes = 1;
        let dia = 20;
        let mut f = Fecha::new(año, mes, dia);
        f.sumar_dias(366);
        assert!(f.año == año + 1 && f.mes == mes && f.dia == dia)
    }
    #[test]
    fn restar_dias_test() {
        let año = 1600;
        let mes = 12;
        let dia = 20;
        let mut f = Fecha::new(año, mes, dia);
        f.restar_dias(366);
        assert!(f.año == año - 1 && f.mes == mes && f.dia == dia)
    }
    #[test]
    fn es_mayor_test() {
        let año = 1600;
        let mes = 12;
        let dia = 20;
        let f = Fecha::new(año, mes, dia);
        let f2 = Fecha::new(año + 1, mes, dia);
        assert!(f2.es_mayor(&f));
    }
    #[test]
    fn es_mayor2_test() {
        let año = 1600;
        let mes = 10;
        let dia = 20;
        let f = Fecha::new(año, mes, dia);
        let f2 = Fecha::new(año, mes + 1, dia);
        assert!(f2.es_mayor(&f));
    }
    #[test]
    fn es_mayor3_test() {
        let año = 1600;
        let mes = 12;
        let dia = 20;
        let f = Fecha::new(año, mes, dia);
        let f2 = Fecha::new(año, mes, dia + 1);
        assert!(f2.es_mayor(&f));
    }

    /*-------------------------------------------------------------- */
    #[test]
    fn new_mp_test() {
        let cvu = 1;
        let url = "1".to_string();
        let id_transaccion = "1".to_string();
        let monto = 1.1;
        let mp = Mp::new(cvu, url.clone(), id_transaccion.clone(), monto);
        assert!(
            mp.cvu == cvu
                && mp.url == url
                && mp.id_transaccion == id_transaccion
                && mp.monto == monto
        )
    }
    #[test]
    fn new_cred_card_test() {
        let cbu = 1;
        let empresa_de_pago = Tarjetas::American;
        let banco = Bancos::Galicia;
        let id_transaccion = "".to_string();
        let monto = 1.1;
        let c = CredCard::new(cbu, empresa_de_pago, banco, id_transaccion.clone(), monto);
        assert!(
            c.cbu == cbu
                && c.id_transaccion == id_transaccion
                && c.monto == monto
                && match c.banco {
                    Bancos::Galicia => true,
                    _ => false,
                }
                && match c.empresa_de_pago {
                    Tarjetas::American => true,
                    _ => false,
                }
        )
    }
    #[test]
    fn new_transfer_test() {
        let cbu = 1;
        let destino = 2;
        let banco = Bancos::Galicia;
        let id_transaccion = "a".to_string();
        let monto = 2.2;
        let t = Transfer::new(cbu, destino, banco, id_transaccion.clone(), monto);
        assert!(
            t.cbu == cbu
                && t.destino == destino
                && t.id_transaccion == id_transaccion
                && t.monto == monto
                && match t.banco {
                    Bancos::Galicia => true,
                    _ => false,
                }
        )
    }
    #[test]
    fn new_crypto_test() {
        let red = "kgf".to_string();
        let token = "srhg".to_string();
        let id_transaccion = "sjkhdgb".to_string();
        let monto = 1.1;
        let c = Crypto::new(red.clone(), token.clone(), id_transaccion.clone(), monto);
        assert!(
            c.red.eq(&red)
                && c.token.eq(&token)
                && c.id_transaccion.eq(&id_transaccion)
                && c.monto == monto
        )
    }
    #[test]
    fn new_cash_test() {
        let monto = 1.5;
        let c = Cash::new(monto);
        assert!(c.monto == monto)
    }
    #[test]
    fn set_monto_cash_test() {
        let monto = 1.5;
        let otro = 45.5;
        let c = Cash::new(monto);
        let mut aux = MediosPago::Efectivo(c);
        aux.set_monto(otro);
        assert!(aux.get_monto() == otro)
    }
    #[test]
    fn set_monto_transfer_test() {
        let cbu = 1;
        let destino = 2;
        let banco = Bancos::Galicia;
        let id_transaccion = "a".to_string();
        let monto = 2.2;
        let t = Transfer::new(cbu, destino, banco, id_transaccion.clone(), monto);
        let otro = 46.5;
        let mut aux = MediosPago::Transferencia(t);
        aux.set_monto(otro);
        assert!(aux.get_monto() == otro)
    }
    #[test]
    fn set_monto_crypto_test() {
        let red = "kgf".to_string();
        let token = "srhg".to_string();
        let id_transaccion = "sjkhdgb".to_string();
        let monto = 1.1;
        let c = Crypto::new(red.clone(), token.clone(), id_transaccion.clone(), monto);
        let mut aux = MediosPago::Cripto(c);
        let otro = 46.5;
        aux.set_monto(otro);
        assert!(aux.get_monto() == otro)
    }
    #[test]
    fn set_monto_cred_card_test() {
        let cbu = 1;
        let empresa_de_pago = Tarjetas::American;
        let banco = Bancos::Galicia;
        let id_transaccion = "".to_string();
        let monto = 1.1;
        let c = CredCard::new(cbu, empresa_de_pago, banco, id_transaccion.clone(), monto);
        let mut aux = MediosPago::TarjetaCredito(c);
        let otro = 46.5;
        aux.set_monto(otro);
        assert!(aux.get_monto() == otro)
    }
    #[test]
    fn set_monto_mp_test() {
        let cvu = 1;
        let url = "1".to_string();
        let id_transaccion = "1".to_string();
        let monto = 1.1;
        let mp = Mp::new(cvu, url.clone(), id_transaccion.clone(), monto);
        let mut aux = MediosPago::MercadoPago(mp);
        let otro = 894.1;
        aux.set_monto(otro);
        assert!(aux.get_monto() == otro)
    }
    #[test]
    fn set_monto_suscripcion_test() {
        let mut s = Suscripcion::default();
        let monto = 4.56;
        s.set_monto(monto);
        assert!(s.costo_mensual == monto && s.medio_pago.get_monto() == monto)
    }
    #[test]
    fn partial_eq_medios_pago_test() {
        let aux = Crypto::default();
        let cr = MediosPago::Cripto(aux);
        let aux = Crypto::new("a".to_string(), "b".to_string(), "c".to_string(), 1.1);
        let cr2 = MediosPago::Cripto(aux);
        //--
        let aux = Cash::default();
        let c = MediosPago::Efectivo(aux);
        let aux = Cash::new(3.3);
        let c2 = MediosPago::Efectivo(aux);
        //--
        let aux = Mp::default();
        let mp = MediosPago::MercadoPago(aux);
        let aux = Mp::new(6841, "a".to_string(), "b".to_string(), 2.2);
        let mp2 = MediosPago::MercadoPago(aux);
        //--
        let aux = CredCard::default();
        let cred = MediosPago::TarjetaCredito(aux);
        let aux = CredCard::new(
            6581,
            Tarjetas::American,
            Bancos::Galicia,
            "a".to_string(),
            4.4,
        );
        let cred2 = MediosPago::TarjetaCredito(aux);
        //--
        let aux = Transfer::default();
        let tr = MediosPago::Transferencia(aux);
        let aux = Transfer::new(651, 564, Bancos::Galicia, "a".to_string(), 4.4);
        let tr2 = MediosPago::Transferencia(aux);
        assert!(
            cr.eq(&cr2) && c.eq(&c2) && mp.eq(&mp2) && cred.eq(&cred2) && tr.eq(&tr2) && cr != c2
        )
    }
    #[test]
    fn new_suscriptor_test() {
        let a = "a".to_string();
        let s = Suscriptor::new(a.clone());
        assert!(s.id == a)
    }
    #[test]
    fn new_suscripcion_test() {
        let f = Fecha::new(320, 10, 15);
        let categ = CategoriaSuscripcion::Basic;
        let tipo_suscripcion = Some(categ);
        let id = "id".to_string();
        let datos_suscriptor = Suscriptor::new(id.clone());
        let costo_mensual=4.5;
        let cr = Crypto::new(
            "red".to_string(),
            "token".to_string(),
            "id".to_string(),
            costo_mensual,
        );
        let medio_pago = MediosPago::Cripto(cr);
        let meses = 8;
        let sus = Suscripcion::new(meses, f, tipo_suscripcion, datos_suscriptor, medio_pago);
        let f = Fecha::new(320, 10, 15);
        let datos_suscriptor = Suscriptor::new(id);
        let cr = Crypto::new(
            "red".to_string(),
            "token".to_string(),
            "id".to_string(),
            2.2,
        );
        let categ = CategoriaSuscripcion::Basic;
        let tipo_suscripcion = Some(categ);
        let medio_pago = MediosPago::Cripto(cr);
        assert!(
            sus.costo_mensual == costo_mensual
                && sus.datos_suscriptor.id == datos_suscriptor.id
                && sus.duracion_meses == meses
                && sus.fecha_inicio == f
                && sus.medio_pago == medio_pago
                && match (sus.tipo_suscripcion, tipo_suscripcion) {
                    (Some(a), Some(b)) => match (a, b) {
                        (CategoriaSuscripcion::Basic, CategoriaSuscripcion::Basic)
                        | (CategoriaSuscripcion::Clasic, CategoriaSuscripcion::Clasic)
                        | (CategoriaSuscripcion::Super, CategoriaSuscripcion::Super) => true,
                        _ => false,
                    },
                    _ => false,
                }
        )
    }
    #[test]
    fn upgrade_subscription_test() {
        let sus = Suscripcion::default();
        assert!(match sus.tipo_suscripcion {
            None => true,
            _ => false,
        });
    }
    #[test]
    fn upgrade_subscription2_test() {
        let mut sus = Suscripcion::default();
        sus.upgrade_subscription();
        assert!(match &sus.tipo_suscripcion {
            Some(a) => match a {
                CategoriaSuscripcion::Basic => true,
                _ => false,
            },
            _ => false,
        });
    }
    #[test]
    fn upgrade_subscription3_test() {
        let mut sus = Suscripcion::default();
        sus.tipo_suscripcion = Some(CategoriaSuscripcion::Basic);
        sus.upgrade_subscription();
        assert!(match &sus.tipo_suscripcion {
            Some(a) => match a {
                CategoriaSuscripcion::Clasic => true,
                _ => false,
            },
            _ => false,
        });
    }
    #[test]
    fn upgrade_subscription4_test() {
        let mut sus = Suscripcion::default();
        sus.tipo_suscripcion = Some(CategoriaSuscripcion::Clasic);
        sus.upgrade_subscription();
        assert!(match &sus.tipo_suscripcion {
            Some(a) => match a {
                CategoriaSuscripcion::Super => true,
                _ => false,
            },
            _ => false,
        });
    }
    #[test]
    fn upgrade_subscription5_test() {
        let mut sus = Suscripcion::default();
        sus.tipo_suscripcion = Some(CategoriaSuscripcion::Super);
        sus.upgrade_subscription();
        assert!(match &sus.tipo_suscripcion {
            Some(a) => match a {
                CategoriaSuscripcion::Super => true,
                _ => false,
            },
            _ => false,
        });
    }
    #[test]
    fn downgrade_subscription_test() {
        let mut sus = Suscripcion::default();

        sus.tipo_suscripcion = Some(CategoriaSuscripcion::Super);
        sus.downgrade_subscription();
        assert!(match &sus.tipo_suscripcion {
            Some(a) => match a {
                CategoriaSuscripcion::Clasic => true,
                _ => false,
            },
            _ => false,
        });
    }
    #[test]
    fn downgrade_subscription2_test() {
        let mut sus = Suscripcion::default();
        sus.tipo_suscripcion = Some(CategoriaSuscripcion::Clasic);
        sus.downgrade_subscription();
        assert!(match &sus.tipo_suscripcion {
            Some(a) => match a {
                CategoriaSuscripcion::Basic => true,
                _ => false,
            },
            _ => false,
        });
    }
    #[test]
    fn downgrade_subscription3_test() {
        let mut sus = Suscripcion::default();
        sus.tipo_suscripcion = Some(CategoriaSuscripcion::Basic);
        sus.downgrade_subscription();
        assert!(match &sus.tipo_suscripcion {
            None => true,
            _ => false,
        });
    }
    #[test]
    fn downgrade_subscription4_test() {
        let mut sus = Suscripcion::default();
        sus.tipo_suscripcion = None;
        sus.downgrade_subscription();
        assert!(match &sus.tipo_suscripcion {
            None => true,
            _ => false,
        });
    }
    #[test]
    fn cancel_subscription_test() {
        let mut sus = Suscripcion::default();

        sus.tipo_suscripcion = Some(CategoriaSuscripcion::Super);
        sus.cancel_subscription();

        assert!(match &sus.tipo_suscripcion {
            None => true,
            _ => false,
        });
    }
    #[test]
    fn new_streaming_test() {
        let sus: Vec<Suscripcion> = Vec::new();
        let mut stre = StreamingRust::new();
        stre.clear_suscripciones();
        assert_eq!(stre.suscripciones.len(), sus.len())
    }
    #[test]
    fn push_streaming_test() {
        let s = Suscripcion::default();
        let mut stre = StreamingRust::new();
        stre.push(s);
        let s = leer_suscripciones_file(stre.path);
        assert_eq!(
            stre.suscripciones.get(0).unwrap().datos_suscriptor.id,
            s[0].datos_suscriptor.id
        )
    }

    #[test]
    fn get_medio_pago_mas_recaudado_crypto_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::Cripto(Crypto::default());
        s.costo_mensual = 451454.0;
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 10000.0;
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 20154.0;
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 541.0;
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = -20.2;
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        assert!(match st.get_medio_pago_mas_recaudado() {
            MediosPago::Cripto(_) => true,
            _ => false,
        })
    }

    #[test]
    fn get_medio_pago_mas_recaudado_cash_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 145.0;
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.costo_mensual = 15478541.0;
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 5415.0;
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = -1.2;
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.costo_mensual = 154.0;
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        assert!(match st.get_medio_pago_mas_recaudado() {
            MediosPago::Efectivo(_) => true,
            _ => false,
        })
    }

    #[test]
    fn get_medio_pago_mas_recaudado_mp_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 154.0;
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 154.0;
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.costo_mensual = 1357461.0;
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = -1.0;
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.costo_mensual = 12.1;
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        assert!(match st.get_medio_pago_mas_recaudado() {
            MediosPago::MercadoPago(_) => true,
            _ => false,
        })
    }

    #[test]
    fn get_medio_pago_mas_recaudado_cred_card_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 1245.0;
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 145.0;
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 5487.0;
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.costo_mensual = 14684731.1;
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.costo_mensual = -1.0;
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        assert!(match st.get_medio_pago_mas_recaudado() {
            MediosPago::TarjetaCredito(_) => true,
            _ => false,
        })
    }

    #[test]
    fn get_medio_pago_mas_recaudado_transfer_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 1454.4;
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 38774.5;
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 454.2;
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = -1.4;
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.costo_mensual = 186516354.5;
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        assert!(match st.get_medio_pago_mas_recaudado() {
            MediosPago::Transferencia(_) => true,
            _ => false,
        })
    }

    #[test]
    fn get_medio_pago_mas_recaudado_crypto_activas_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let s = Suscripcion::default();
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 5000.0;
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 45.45;
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 457.3;
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = -2.1;
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 154.1;
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.costo_mensual = 457848.6;
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        assert!(match st.get_medio_pago_mas_recaudado_activas() {
            MediosPago::Cripto(_) => true,
            _ => false,
        })
    }

    #[test]
    fn get_medio_pago_mas_recaudado_cash_activas_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let s = Suscripcion::default();
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 5000.0;
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 154.3;
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 4999.99;
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 145.0;
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 1245.3;
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.costo_mensual = 154848.4;
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        assert!(match st.get_medio_pago_mas_recaudado_activas() {
            MediosPago::Efectivo(_) => true,
            _ => false,
        })
    }

    #[test]
    fn get_medio_pago_mas_recaudado_mp_activas_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let s = Suscripcion::default();
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 5000.0;
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 4999.999;
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 145.2;
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = -12.4;
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 145.3;
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.costo_mensual = 13658451.4;
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        assert!(match st.get_medio_pago_mas_recaudado_activas() {
            MediosPago::MercadoPago(_) => true,
            _ => false,
        })
    }

    #[test]
    fn get_medio_pago_mas_recaudado_cred_card_activas_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let s = Suscripcion::default();
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 5000.0;
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 4999.99;
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 154.3;
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 132.2;
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = -12.2;
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.costo_mensual = 684635.2;
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        assert!(match st.get_medio_pago_mas_recaudado_activas() {
            MediosPago::TarjetaCredito(_) => true,
            _ => false,
        })
    }

    #[test]
    fn get_medio_pago_mas_recaudado_transfer_activas_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let s = Suscripcion::default();
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 5000.0;
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 0.0;
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 154.2;
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = 45.1;
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.costo_mensual = -1.3;
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.costo_mensual = 35416851.2;
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        assert!(match st.get_medio_pago_mas_recaudado_activas() {
            MediosPago::Transferencia(_) => true,
            _ => false,
        })
    }

    #[test]
    fn get_medio_mas_usado_transfer_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        assert!(match st.get_medio_mas_usado() {
            MediosPago::Transferencia(_) => true,
            _ => false,
        })
    }
    #[test]
    #[should_panic(expected = "Vector sin sucripciones activas")]
    fn get_medio_mas_usado_panic_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();
        st.get_medio_mas_usado();
    }
    #[test]
    fn get_medio_mas_usado_crypto_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);
        assert!(match st.get_medio_mas_usado() {
            MediosPago::Cripto(_) => true,
            _ => false,
        });
    }
    #[test]
    fn get_medio_mas_usado_cash_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);
        assert!(match st.get_medio_mas_usado() {
            MediosPago::Efectivo(_) => true,
            _ => false,
        });
    }
    #[test]
    fn get_medio_mas_usado_mp_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        assert!(match st.get_medio_mas_usado() {
            MediosPago::MercadoPago(_) => true,
            _ => false,
        });
    }
    #[test]
    fn get_medio_mas_usado_card_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);
        assert!(match st.get_medio_mas_usado() {
            MediosPago::TarjetaCredito(_) => true,
            _ => false,
        });
    }
    #[test]
    fn get_medio_mas_usado_crypto_activas_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        assert!(match st.get_medio_mas_usado_activas() {
            MediosPago::Cripto(_) => true,
            _ => false,
        })
    }
    #[test]
    fn get_medio_mas_usado_cash_activas_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        assert!(match st.get_medio_mas_usado_activas() {
            MediosPago::Efectivo(_) => true,
            _ => false,
        })
    }

    #[test]
    fn get_medio_mas_usado_transfer_activas_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        assert!(match st.get_medio_mas_usado_activas() {
            MediosPago::Transferencia(_) => true,
            _ => false,
        })
    }
    #[test]
    fn get_medio_mas_usado_mp_activas_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        assert!(match st.get_medio_mas_usado_activas() {
            MediosPago::MercadoPago(_) => true,
            _ => false,
        })
    }
    #[test]
    fn get_medio_mas_usado_cred_card_activas_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Efectivo(Cash::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::MercadoPago(Mp::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::TarjetaCredito(CredCard::default());
        st.push(s);

        let mut s = Suscripcion::default();
        s.upgrade_subscription();
        s.medio_pago = MediosPago::Transferencia(Transfer::default());
        st.push(s);

        assert!(match st.get_medio_mas_usado_activas() {
            MediosPago::TarjetaCredito(_) => true,
            _ => false,
        })
    }

    #[test]
    #[should_panic(expected = "Vector sin sucripciones activas")]
    fn get_medio_mas_usado_transfer_activas_panic_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();
        let mut s = Suscripcion::default();

        s.medio_pago = MediosPago::Cripto(Crypto::default());
        st.push(s);
        st.get_medio_mas_usado_activas();
    }

    #[test]
    fn get_suscripcion_mas_contratada_none_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();
        let s = Suscripcion::default();
        st.push(s);
        assert!(match st.get_suscripcion_mas_contratada() {
            None => true,
            _ => false,
        });
    }
    #[test]
    fn get_suscripcion_mas_contratada_basic_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();
        let s = Suscripcion::default();
        st.push(s);

        let mut s = Suscripcion::default();
        s.tipo_suscripcion = Some(CategoriaSuscripcion::Basic);
        st.push(s);

        let mut st = StreamingRust::new();
        let mut s = Suscripcion::default();
        s.tipo_suscripcion = Some(CategoriaSuscripcion::Basic);
        st.push(s);

        let mut st = StreamingRust::new();
        let mut s = Suscripcion::default();
        s.tipo_suscripcion = Some(CategoriaSuscripcion::Clasic);
        st.push(s);

        let mut st = StreamingRust::new();
        let mut s = Suscripcion::default();
        s.tipo_suscripcion = Some(CategoriaSuscripcion::Basic);
        st.push(s);

        let mut st = StreamingRust::new();
        let mut s = Suscripcion::default();
        s.tipo_suscripcion = Some(CategoriaSuscripcion::Super);
        st.push(s);

        let mut st = StreamingRust::new();
        let mut s = Suscripcion::default();
        s.tipo_suscripcion = Some(CategoriaSuscripcion::Super);
        st.push(s);

        assert!(match st.get_suscripcion_mas_contratada() {
            Some(a) => match a {
                CategoriaSuscripcion::Basic => true,
                _ => false,
            },
            _ => false,
        });
    }

    #[test]
    fn get_suscripcion_mas_contratada_clasic_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();
        let s = Suscripcion::default();
        st.push(s);

        let mut s = Suscripcion::default();
        s.tipo_suscripcion = Some(CategoriaSuscripcion::Clasic);
        st.push(s);

        let mut st = StreamingRust::new();
        let mut s = Suscripcion::default();
        s.tipo_suscripcion = Some(CategoriaSuscripcion::Basic);
        st.push(s);

        let mut st = StreamingRust::new();
        let mut s = Suscripcion::default();
        s.tipo_suscripcion = Some(CategoriaSuscripcion::Clasic);
        st.push(s);

        let mut st = StreamingRust::new();
        let mut s = Suscripcion::default();
        s.tipo_suscripcion = Some(CategoriaSuscripcion::Basic);
        st.push(s);

        let mut st = StreamingRust::new();
        let mut s = Suscripcion::default();
        s.tipo_suscripcion = Some(CategoriaSuscripcion::Clasic);
        st.push(s);

        let mut st = StreamingRust::new();
        let mut s = Suscripcion::default();
        s.tipo_suscripcion = Some(CategoriaSuscripcion::Super);
        st.push(s);

        assert!(match st.get_suscripcion_mas_contratada() {
            Some(a) => match a {
                CategoriaSuscripcion::Clasic => true,
                _ => false,
            },
            _ => false,
        });
    }
    #[test]
    fn get_suscripcion_mas_contratada_super_test() {
        let mut st = StreamingRust::new();
        st.clear_suscripciones();
        let s = Suscripcion::default();
        st.push(s);

        let mut s = Suscripcion::default();
        s.tipo_suscripcion = Some(CategoriaSuscripcion::Clasic);
        st.push(s);

        let mut st = StreamingRust::new();
        let mut s = Suscripcion::default();
        s.tipo_suscripcion = Some(CategoriaSuscripcion::Super);
        st.push(s);

        let mut st = StreamingRust::new();
        let mut s = Suscripcion::default();
        s.tipo_suscripcion = Some(CategoriaSuscripcion::Clasic);
        st.push(s);

        let mut st = StreamingRust::new();
        let mut s = Suscripcion::default();
        s.tipo_suscripcion = Some(CategoriaSuscripcion::Basic);
        st.push(s);

        let mut st = StreamingRust::new();
        let mut s = Suscripcion::default();
        s.tipo_suscripcion = Some(CategoriaSuscripcion::Super);
        st.push(s);

        let mut st = StreamingRust::new();
        let mut s = Suscripcion::default();
        s.tipo_suscripcion = Some(CategoriaSuscripcion::Super);
        st.push(s);

        assert!(match st.get_suscripcion_mas_contratada() {
            Some(a) => match a {
                CategoriaSuscripcion::Super => true,
                _ => false,
            },
            _ => false,
        });
    }
    #[test]
    #[should_panic]
    fn crear_file_panic_test() {
        let path = "coverage/";
        let esc = "ikasdfbg";
        crear_file(path, &serde_json::to_string_pretty(&esc).unwrap())
    }
}
