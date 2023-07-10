use crate::tp3::Fecha;

/*1- Escriba una función que reciba un vector de números enteros y retorna la cantidad de
números primos. Cree un trait para la determinación del número primo e impleméntelo
según corresponda. Utilice la función iter sobre el vector y aplique un closure para
resolverlo */

use std::{cmp::Ordering, collections::HashMap, u8};

pub fn ej1(vector: Vec<i32>) -> i32 {
    let mut cant = 0;

    let check_pair = |x: i32, cant: &mut i32| {
        if x.es_par() {
            *cant += 1;
        }
    };
    vector.iter().for_each(|x| check_pair(*x, &mut cant));

    cant
}

pub trait DeterminarI32 {
    fn es_par(&self) -> bool;
}
impl DeterminarI32 for i32 {
    fn es_par(&self) -> bool {
        let es: bool;
        if *self % 2 == 0 {
            es = true;
        } else {
            es = false;
        }
        es
    }
}
#[derive(Debug)]
pub struct Persona<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    ciudad: &'a str,
    salario: f64,
    edad: u8,
}
impl<'a> Persona<'a> {
    pub fn new(
        nombre: &'a str,
        apellido: &'a str,
        direccion: &'a str,
        ciudad: &'a str,
        salario: f64,
        edad: u8,
    ) -> Persona<'a> {
        Persona {
            nombre,
            apellido,
            direccion,
            ciudad,
            salario,
            edad,
        }
    }
}
impl<'a> PartialEq for Persona<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.apellido == other.apellido && self.edad == other.edad && self.nombre == other.nombre
    }
}
impl<'a> Eq for Persona<'a> {}
pub trait AgregarEdad {
    fn agregar_edad<'a>(&mut self, persona: &'a Persona);
}

pub fn get_personas_salario_mayor_a<'a, T>(
    personas: &'a Vec<&'a Persona>,
    valor: f64,
) -> Vec<&'a &'a Persona<'a>> {
    let iter_pers = personas.iter();
    let check = |persona: &Persona, valor: f64| -> bool {
        if persona.salario > valor {
            true
        } else {
            false
        }
    };
    let aux: Vec<_> = iter_pers
        .filter(move |x| check(x, valor))
        .collect::<Vec<_>>();

    // let mut agregar = |persona| vec_aux.agregar_salario_mayor(persona, valor);
    // iter_pers.for_each(|x| agregar(x));
    aux
}
/*
b- Escriba una función que reciba un vector de personas, edad y el nombre de una ciudad,
y retorna las personas mayores al parámetro edad y que viven en el valor del parámetro
ciudad.*/
pub fn get_mayores_que_en<'a>(
    personas: &'a Vec<&'a Persona>,
    edad: u8,
    ciudad: &'a str,
) -> Vec<&'a &'a Persona<'a>> {
    let iter_pers = personas.iter();
    let check = |persona: &Persona, edad: u8, ciudad: &str| {
        if persona.edad > edad && persona.ciudad == ciudad {
            true
        } else {
            false
        }
    };
    let vec_aux = iter_pers
        .filter(|x| check(x, edad, ciudad))
        .collect::<Vec<_>>();
    vec_aux
}
/*
c- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
retorna true si todas las personas viven en la ciudad pasada por parámetro, false caso
contrario.*/
pub fn si_todos_en_ciudad<'a>(personas: Vec<&'a Persona>, ciudad: &'a str) -> bool {
    let si: bool;
    let mut iter_pers = personas.iter();
    let check = |persona: &Persona, ciudad| -> bool {
        if persona.ciudad == ciudad {
            true
        } else {
            false
        }
    };
    si = iter_pers.all(|persona| check(persona, ciudad));
    si
}
/*
d- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
retorna true si al menos vive una persona en la ciudad pasada por parámetro,, false caso
contrario.*/
pub fn si_alguno_en_ciudad<'a>(personas: Vec<&'a Persona>, ciudad: &'a str) -> bool {
    let si: bool;
    let mut iter_pers = personas.iter();
    let check = |persona: &Persona, ciudad| -> bool {
        if persona.ciudad == ciudad {
            true
        } else {
            false
        }
    };
    si = iter_pers.any(|persona| check(persona, ciudad));
    si
}
/*
e- Escriba una función que reciba un arreglo de personas y una persona y retorna true si la
persona existe en el arreglo, false caso contrario*/
pub fn si_persona_esta<'a>(personas: Vec<&'a Persona>, otra: &Persona) -> bool {
    let si: bool;
    let mut iter_pers = personas.iter();
    let check = |persona: &Persona, otra: &Persona| -> bool { persona == otra };
    si = iter_pers.any(|persona| check(persona, otra));
    si
}
/*
f -Escriba una función que reciba un arreglo de personas y retorna un arreglo con las
edades de las personas.*/
pub fn get_edades<'a>(personas: Vec<&'a Persona>) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    let iter_pers = personas.iter();
    let add = |persona: &Persona, vec: &mut Vec<u8>| {
        vec.push(persona.edad);
    };
    iter_pers.for_each(|x| add(x, &mut vec));
    vec
}

/*
g - Escriba una función que reciba un arreglo de personas y retorna la persona con el menor
salario y la persona con el mayor salario, en caso de que haya más de una persona en cada
categoría desempatar por la edad más grande.
Nota: Implemente todos los métodos y traits que considere para resolver los ejercicios.
Todos los ejercicios deben resolverse con iterator y closure.*/
pub fn get_menor_y_mayor_salario<'a>(
    personas: &'a Vec<&'a Persona>,
) -> (Option<&'a &'a Persona<'a>>, Option<&'a &'a Persona<'a>>) {
    let iter_pers = personas.iter();
    let check = |persona: &Persona, other: &Persona| -> Ordering {
        if persona.salario > other.salario {
            Ordering::Greater
        } else if persona.salario == other.salario && persona.edad > other.edad {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    };
    let max = iter_pers.max_by(|x, y| check(x, y));
    let iter_pers = personas.iter();
    let min = iter_pers.min_by(|x, y| check(x, y));
    (min, max)
}
#[derive(Debug)]

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
#[derive(Debug)]

pub enum Tarjetas {
    Visa,
    Mastercard,
    American,
    Cabal,
}
#[derive(Debug)]

pub enum Bancos {
    Santander,
    Provincia,
    Naranja,
    Galicia,
}
#[derive(Debug)]

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
#[derive(Debug)]

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
#[derive(Debug)]

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
#[derive(Debug)]

pub struct Cash {
    monto: f64,
}
impl Cash {
    pub fn new(monto: f64) -> Cash {
        Cash { monto }
    }
}
pub trait SetMonto {
    fn set_monto(&mut self, monto: f64);
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
#[derive(Debug)]
pub enum MediosPago {
    Cripto(Crypto),
    Efectivo(Cash),
    MercadoPago(Mp),
    TarjetaCredito(CredCard),
    Transferencia(Transfer),
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
            (&MediosPago::Cripto(_), &MediosPago::Cripto(_)) => true,
            (&MediosPago::Efectivo(_), &MediosPago::Efectivo(_)) => true,
            (&MediosPago::TarjetaCredito(_), &MediosPago::TarjetaCredito(_)) => true,
            (&MediosPago::Transferencia(_), &MediosPago::Transferencia(_)) => true,
            _ => false,
        }
    }
}
impl Eq for MediosPago {}
#[derive(Debug)]

pub struct Suscripcion {
    costo_mensual: f64,
    duracion_meses: u8,
    fecha_inicio: Fecha,
}
impl Suscripcion {
    //Crear suscripcion
    pub fn new(costo_mensual: f64, duracion_meses: u8, fecha_inicio: Fecha) -> Suscripcion {
        Suscripcion {
            costo_mensual,
            duracion_meses,
            fecha_inicio,
        }
    }
}
#[derive(Debug)]

pub enum Suscripciones {
    Basic,
    Clasic,
    Super,
}
#[derive(Debug)]

pub struct Suscriptor {
    id: String,
    tipo_suscripcion: Option<Suscripciones>,
    datos_suscripcion: Option<Suscripcion>,
    medio_pago: MediosPago,
}
impl Suscriptor {
    //Crear nuevo suscriptor
    pub fn new(
        id: String,
        tipo_suscripcion: Option<Suscripciones>,
        datos_suscripcion: Option<Suscripcion>,
        medio_pago: MediosPago,
    ) -> Suscriptor {
        Suscriptor {
            id,
            tipo_suscripcion,
            datos_suscripcion,
            medio_pago,
        }
    }
    pub fn upgrade_subscription(&mut self) {
        match &self.tipo_suscripcion {
            Some(a) => match a {
                Suscripciones::Basic => self.tipo_suscripcion = Some(Suscripciones::Clasic),
                Suscripciones::Clasic => self.tipo_suscripcion = Some(Suscripciones::Super),
                Suscripciones::Super => (),
            },
            None => (),
        }
    }
    pub fn downgrade_subscription(&mut self) {
        match &self.tipo_suscripcion {
            Some(a) => match a {
                Suscripciones::Super => self.tipo_suscripcion = Some(Suscripciones::Clasic),
                Suscripciones::Clasic => self.tipo_suscripcion = Some(Suscripciones::Basic),
                Suscripciones::Basic => {
                    self.tipo_suscripcion = None;
                    self.datos_suscripcion = None;
                }
            },
            None => (),
        }
    }
    pub fn cancel_subscription(&mut self) {
        self.datos_suscripcion = None;
        self.tipo_suscripcion = None;
    }
}
#[derive(Debug)]
pub struct StreamingRust {
    pub suscriptores: Vec<Suscriptor>,
}
use std::cmp::max;
impl StreamingRust {
    //Crear el streaming
    pub fn new() -> StreamingRust {
        let suscriptores: Vec<Suscriptor> = Vec::new();
        StreamingRust { suscriptores }
    }
    pub fn push(&mut self, sus: Suscriptor) {
        self.suscriptores.push(sus);
    }
    pub fn get_medio_pago_mas_usado(&self) -> MediosPago {
        let iter_sus = self.suscriptores.iter();
        let check = |sus: &Suscriptor, medio: MediosPago| match (&sus.medio_pago, medio) {
            (MediosPago::MercadoPago(_), MediosPago::MercadoPago(_)) => true,
            (MediosPago::Cripto(_), MediosPago::Cripto(_)) => true,
            (MediosPago::Efectivo(_), MediosPago::Efectivo(_)) => true,
            (MediosPago::TarjetaCredito(_), MediosPago::TarjetaCredito(_)) => true,
            (MediosPago::Transferencia(_), MediosPago::Transferencia(_)) => true,
            _ => false,
        };
        let cant_mp = iter_sus
            .filter(|x| {
                check(
                    x,
                    MediosPago::MercadoPago(Mp {
                        cvu: 0,
                        url: ("".to_string()),
                        id_transaccion: ("".to_string()),
                        monto: (0.0),
                    }),
                )
            })
            .collect::<Vec<_>>()
            .len();
        let iter_sus = self.suscriptores.iter();
        let cant_ef = iter_sus
            .filter(|x| check(x, MediosPago::Efectivo(Cash { monto: 0.0 })))
            .collect::<Vec<&Suscriptor>>()
            .len();
        let iter_sus = self.suscriptores.iter();
        let cant_card = iter_sus
            .filter(|x| {
                check(
                    x,
                    MediosPago::TarjetaCredito(CredCard {
                        cbu: 0,
                        empresa_de_pago: (Tarjetas::American),
                        banco: (Bancos::Galicia),
                        id_transaccion: ("".to_string()),
                        monto: (0.0),
                    }),
                )
            })
            .collect::<Vec<&Suscriptor>>()
            .len();
        let iter_sus = self.suscriptores.iter();
        let cant_crypto = iter_sus
            .filter(|x| {
                check(
                    x,
                    MediosPago::Cripto(Crypto {
                        red: "".to_string(),
                        token: ("".to_string()),
                        id_transaccion: ("".to_string()),
                        monto: (0.0),
                    }),
                )
            })
            .collect::<Vec<&Suscriptor>>()
            .len();
        let iter_sus = self.suscriptores.iter();
        let cant_transf = iter_sus
            .filter(|x| {
                check(
                    x,
                    MediosPago::Transferencia(Transfer {
                        cbu: 0,
                        destino: (0),
                        banco: (Bancos::Galicia),
                        id_transaccion: ("".to_string()),
                        monto: (0.0),
                    }),
                )
            })
            .collect::<Vec<&Suscriptor>>()
            .len();
        let maxes = [cant_crypto, cant_ef, cant_mp, cant_card, cant_transf];
        let check2 = |x, y| -> bool {
            if x >= y {
                true
            } else {
                false
            }
        };
        match maxes
            .iter()
            .position(|x| check2(x, maxes.iter().max().unwrap()))
        {
            Some(a) => match a {
                0 => MediosPago::Cripto(Crypto {
                    red: ("".to_string()),
                    token: ("".to_string()),
                    id_transaccion: ("".to_string()),
                    monto: (0.0),
                }),
                1 => MediosPago::Efectivo(Cash { monto: 0.0 }),
                2 => MediosPago::MercadoPago(Mp {
                    cvu: 0,
                    url: ("".to_string()),
                    id_transaccion: ("".to_string()),
                    monto: (0.0),
                }),
                3 => MediosPago::TarjetaCredito(CredCard {
                    cbu: 0,
                    empresa_de_pago: (Tarjetas::American),
                    banco: (Bancos::Galicia),
                    id_transaccion: ("".to_string()),
                    monto: (0.0),
                }),
                4 => MediosPago::Transferencia(Transfer {
                    cbu: 0,
                    destino: (0),
                    banco: (Bancos::Galicia),
                    id_transaccion: ("".to_string()),
                    monto: (0.0),
                }),
                _ => panic!("Invalido"),
            },
            None => panic!("Invalid"),
        }
    }
    pub fn get_medio_mas_usado(&self) -> MediosPago {
        let iter_sus = self.suscriptores.iter();
        let mut arr = [0, 0, 0, 0, 0];
        let check = |susc: &Suscriptor, arr: &mut [i32; 5]| match susc.medio_pago {
            MediosPago::Cripto(_) => arr[0] += 1,
            MediosPago::Efectivo(_) => arr[1] += 1,
            MediosPago::MercadoPago(_) => arr[2] += 1,
            MediosPago::TarjetaCredito(_) => arr[3] += 1,
            MediosPago::Transferencia(_) => arr[4] += 1,
        };
        iter_sus.for_each(|x| check(x, &mut arr));
        let check2 = |x, y| -> bool {
            if x == y {
                true
            } else {
                false
            }
        };
        match arr
            .iter()
            .position(|x| check2(x, arr.iter().max().unwrap()))
        {
            Some(a) => match a {
                0 => MediosPago::Cripto(Crypto {
                    red: ("".to_string()),
                    token: ("".to_string()),
                    id_transaccion: ("".to_string()),
                    monto: (0.0),
                }),
                1 => MediosPago::Efectivo(Cash { monto: 0.0 }),
                2 => MediosPago::MercadoPago(Mp {
                    cvu: 0,
                    url: ("".to_string()),
                    id_transaccion: ("".to_string()),
                    monto: (0.0),
                }),
                3 => MediosPago::TarjetaCredito(CredCard {
                    cbu: 0,
                    empresa_de_pago: (Tarjetas::American),
                    banco: (Bancos::Galicia),
                    id_transaccion: ("".to_string()),
                    monto: (0.0),
                }),
                4 => MediosPago::Transferencia(Transfer {
                    cbu: 0,
                    destino: (0),
                    banco: (Bancos::Galicia),
                    id_transaccion: ("".to_string()),
                    monto: (0.0),
                }),
                _ => panic!("Invalido"),
            },
            None => panic!("Invalido"),
        }
    }
    /*➢ Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones
    activas. */
    pub fn get_suscripcion_mas_contratada(&self) -> Option<Suscripciones> {
        let iter_sus = self.suscriptores.iter();
        let mut arr = [0, 0, 0, 0];
        let check = |x: &Suscriptor, arr: &mut [i32; 4]| match &x.tipo_suscripcion {
            Some(a) => match a {
                Suscripciones::Basic => arr[1] += 1,
                Suscripciones::Clasic => arr[2] += 1,
                Suscripciones::Super => arr[3] += 1,
            },
            None => arr[0] += 1,
        };
        iter_sus.for_each(|x| check(x, &mut arr));
        match arr.iter().position(|x| x == arr.iter().max().unwrap()) {
            Some(a) => match a {
                0 => None,
                1 => Some(Suscripciones::Basic),
                2 => Some(Suscripciones::Clasic),
                3 => Some(Suscripciones::Super),
                _ => panic!("Invalid"),
            },
            None => panic!("Invalid"),
        }
    }
}

/*3 -La plataforma de streaming "StreamingRust" ofrece distintos tipos de suscripciones
(Basic, Clasic, Super) a sus usuarios. Cada suscripción tiene un costo mensual y una
duración de meses y una fecha de inicio, además los usuarios pueden pagar por sus
suscripciones con distintos medios de pago que son Efectivo, MercadoPago, Tarjeta de
Crédito, Transferencia Bancaria, Cripto. Cada medio de pago tiene sus datos
correspondientes a excepción de Efectivo.
Los usuarios solo pueden tener una suscripción activa a la vez.
Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
siguientes acciones:
➢ Crear un usuario con una determinada suscripción y medio de pago.
➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic
pasa a Clasic y si está en Clasic pasa a Super.
➢ Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la
suscripción es del tipo Basic al hacerlo se cancelará la suscripción.
➢ Dado un usuario cancelar la suscripción.
➢ Saber el medio de pago que es más utilizado por los usuarios sobre las
suscripciones activas
➢ Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones
activas.
➢ Saber cuál fue el medio de pago más utilizado.
➢ Saber cuál fue la suscripción más contratada. */

/*4 -Se requiere implementar un sistema de ventas de productos. De cada producto se
conoce el nombre, una categoría y un precio base, y algunos productos pueden tener
descuentos aplicables dependiendo de la categoría. Además, se debe registrar al vendedor
que realizó la venta y al cliente. De ellos se conoce nombre, apellido, dirección, dni y del
vendedor nro de legajo, antigüedad y salario. Los clientes pueden tener un beneficio de
descuento si tienen suscripción al newsletter, de ser así se tiene el correo electrónico del
mismo.
El sistema debe permitir registrar las ventas realizadas y asociar el medio de pago utilizado.
Los medios de pago aceptados son: tarjeta de crédito, tarjeta de débito, transferencia
bancaria y efectivo.
Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
siguientes acciones:
 */
#[derive(Debug)]
pub enum Categorias {
    A,
    B,
    C,
}
impl PartialEq for Categorias {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Categorias::A, Categorias::A) => true,
            (Categorias::B, Categorias::B) => true,
            (Categorias::C, Categorias::C) => true,
            _ => false,
        }
    }
}
impl Eq for Categorias {}
#[derive(Clone, Copy, Debug)]

pub struct Producto<'a> {
    nombre: &'a str,
    categoria: &'a Categorias,
    precio_base: f64,
}
impl<'a> Producto<'a> {
    pub fn new(nombre: &'a str, categoria: &'a Categorias, precio_base: f64) -> Producto<'a> {
        Producto {
            nombre,
            categoria,
            precio_base,
        }
    }
}
#[derive(Debug)]
pub struct DatosPersona<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: String,
    dni: u32,
}
impl<'a> DatosPersona<'a> {
    pub fn new(
        nombre: &'a str,
        apellido: &'a str,
        direccion: String,
        dni: u32,
    ) -> DatosPersona<'a> {
        DatosPersona {
            nombre,
            apellido,
            direccion,
            dni,
        }
    }
}
#[derive(Debug)]
pub struct Vendedor<'a> {
    datos: &'a DatosPersona<'a>,
    legajo: u32,
    antiguedad: u32,
    salario: f64,
}
impl<'a> PartialEq for Vendedor<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.legajo == other.legajo {
            true
        } else {
            false
        }
    }
}
impl<'a> Eq for Vendedor<'a> {}
impl<'a> Vendedor<'a> {
    pub fn new(
        datos: &'a DatosPersona<'a>,
        legajo: u32,
        antiguedad: u32,
        salario: f64,
    ) -> Vendedor<'a> {
        Vendedor {
            datos,
            legajo,
            antiguedad,
            salario,
        }
    }
}
#[derive(Debug)]
pub struct Cliente<'a> {
    datos: &'a DatosPersona<'a>,
    descuento: Option<f64>,
    correo: String,
}
impl<'a> Cliente<'a> {
    pub fn new(datos: &'a DatosPersona<'a>, descuento: Option<f64>, correo: String) -> Cliente<'a> {
        Cliente {
            datos,
            descuento,
            correo,
        }
    }
}
#[derive(Debug)]
pub struct Venta<'a> {
    fecha: &'a Fecha,
    productos: &'a Vec<&'a Producto<'a>>,
    vendedor: &'a Vendedor<'a>,
    cliente: &'a Cliente<'a>,
    pago: Option<&'a MediosPago>,
}
impl<'a> Venta<'a> {
    /*➢ Crear una venta con: fecha, cliente, vendedor, medio de pago y un listado de
    productos. */
    pub fn new(
        fecha: &'a Fecha,
        cliente: &'a Cliente<'a>,
        vendedor: &'a Vendedor<'a>,
        pago: Option<&'a MediosPago>,
        productos: &'a Vec<&'a Producto<'a>>,
    ) -> Venta<'a> {
        Venta {
            fecha,
            productos,
            vendedor,
            cliente,
            pago,
        }
    }
    pub fn set_pago(&mut self, pago: &'a MediosPago) {
        self.pago = Some(pago);
    }
    /*➢ Calcular el precio final de una venta en base a los productos que hay en ella. Para
    calcularlo tenga en cuenta que pueden haber determinados productos de alguna
    categoría donde debería aplicarse un descuento. Tanto la categoría como el
    porcentaje de descuento a aplicar son datos que le brinda el sistema. Es decir el
    sistema tiene una lista de las categorías con el descuento a aplicar. Además se debe
    aplicar un porcentaje de descuento general si el cliente tiene suscripción al
    newsletter. */
    pub fn calcular_precio_total<MediosPago: SetMonto>(
        &mut self,
        lista_descuentos: Vec<(Categorias, f64)>,
        pago: Option<&mut MediosPago>,
    ) -> f64 {
        let iter_ventas = self.productos.iter();
        let mut total = 0.0;
        let mut check = |producto: Producto, lista: &Vec<(Categorias, f64)>| {
            let mut aux = producto.precio_base;
            for (i, j) in lista {
                if producto.categoria == i {
                    aux -= aux * (j / 100.0);
                }
                if let Some(desc) = self.cliente.descuento {
                    aux -= aux * (desc / 100.0);
                }
            }
            total += aux;
        };
        iter_ventas.for_each(|&&producto| check(producto, &lista_descuentos));
        if let Some(p) = pago {
            p.set_monto(total);
        }

        total
    }
}

pub fn filtrar_venta_categoria<'a>(
    venta: &Venta<'a>,
    categoria: &Categorias,
) -> Vec<&'a Producto<'a>> {
    let mut productos: Vec<&Producto> = Vec::new();
    let iter_venta = venta.productos.iter();
    let check = |producto: Producto| -> bool {
        if producto.categoria == categoria {
            true
        } else {
            false
        }
    };
    let res = iter_venta.filter(|&&x| check(*x)).collect::<Vec<_>>();
    for &i in res {
        productos.push(i);
    }

    productos
}
#[derive(Debug)]
pub struct SistemaVentas<'a> {
    pub ventas: Vec<&'a Venta<'a>>,
    pub lista_descuentos: Vec<(Categorias, f64)>,
}

/*➢ Para llevar un control de las ventas realizadas, se debe implementar un reporte que
permita visualizar las ventas totales por categoría de producto y otro por vendedor. */
impl<'a> SistemaVentas<'a> {
    pub fn new(
        ventas: Vec<&'a Venta<'a>>,
        lista_descuentos: Vec<(Categorias, f64)>,
    ) -> SistemaVentas<'a> {
        SistemaVentas {
            ventas,
            lista_descuentos,
        }
    }
    pub fn reporte_por_categoria(&mut self, categoria: &Categorias) -> Vec<&Producto> {
        let mut vec = Vec::new();
        for i in &self.ventas {
            vec.append(&mut filtrar_venta_categoria(i, categoria));
        }
        vec
    }
    pub fn reporte_por_vendedor(&self, vendedor: &Vendedor) -> Vec<&'a &'a Venta> {
        let iter = self.ventas.iter();
        let check = |venta: &Venta| -> bool {
            if venta.vendedor == vendedor {
                true
            } else {
                false
            }
        };
        let res = iter.filter(|&&x| check(x)).collect::<Vec<_>>();
        res
    }
}
/*5- La empresa XYZ es una plataforma de intercambio de criptoactivos que permite a los
usuarios comprar y vender distintas criptomonedas. La plataforma permite el registro de
usuarios y la gestión de sus balances en distintas criptomonedas y en dinero fíat.
Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
siguientes acciones relacionadas al usuario:





Además la empresa desea saber lo siguiente en base a sus operaciones:
➢ Saber cual es la criptomoneda que más cantidad de ventas tiene
➢ Saber cual es la criptomoneda que más cantidad de compras tiene
➢ Saber cual es la criptomoneda que más volumen de ventas tiene
➢ Saber cual es la criptomoneda que más volumen de compras tiene */
/*De los
usuarios se conoce nombre, apellido, email, dni, y si está validada su identidad o no. Cada
usuario tiene un balance de las criptomonedas que se ofrecen en la plataforma. */

use random_string::generate;
#[derive(Debug)]

pub enum TiposTransaccion {
    IngresoFiat,
    EgresoFiat,
    IngresoCripto,
    EgresoCripto,
}

/*Luego de ello se registra la transacción con los siguientes datos:
fecha, usuario, criptomoneda, tipo: compra de cripto, monto de cripto y cotización. */
#[derive(Debug)]

pub struct Transaccion<'a> {
    fecha: Option<&'a Fecha>,
    usuario: Option<&'a Usuario<'a>>,
    monto_fiat: Option<f64>,
    tipo_fiat: Option<&'a TiposTransaccion>,
    cripto: Option<&'a Cripto<'a>>,
    blockchain: Option<&'a Blockchain<'a>>,
    hash: Option<String>,
    monto_cripto: Option<f64>,
    tipo_cripto: Option<&'a TiposTransaccion>,
}
impl<'a> Transaccion<'a> {
    pub fn new(
        fecha: Option<&'a Fecha>,
        usuario: Option<&'a Usuario<'a>>,
        monto_fiat: Option<f64>,
        tipo_fiat: Option<&'a TiposTransaccion>,
        cripto: Option<&'a Cripto<'a>>,
        blockchain: Option<&'a Blockchain<'a>>,
        hash: Option<String>,
        monto_cripto: Option<f64>,
        tipo_cripto: Option<&'a TiposTransaccion>,
    ) -> Transaccion<'a> {
        Transaccion {
            fecha,
            usuario,
            monto_fiat,
            tipo_fiat,
            cripto,
            blockchain,
            hash,
            monto_cripto,
            tipo_cripto,
        }
    }
}
#[derive(Debug)]
pub struct Broker<'a> {
    usuarios: Vec<Usuario<'a>>,
    historial_transacciones: Vec<Transaccion<'a>>,
}

impl<'a> Broker<'a> {
    pub fn new() -> Broker<'a> {
        let usuarios = Vec::new();
        let historial_transacciones = Vec::new();
        Broker {
            usuarios,
            historial_transacciones,
        }
    }
    pub fn new_user(&mut self, user: Usuario<'a>) {
        self.usuarios.push(user);
    }
    /*➢ Ingresar dinero: se recibe un monto en fiat de un usuario y se acredita al balance de
    fiat de dicho usuario. Además se crea una transacción del hecho donde los datos
    que se guardan son:fecha, tipo(ingreso de dinero), monto, usuario. */
    fn ingresar_fiat(
        &mut self,
        user: &'a Usuario<'a>,
        fiat: f64,
        fecha: &'a Fecha,
    ) -> Transaccion<'a> {
        let tr: Transaccion;
        let mut camb = false;
        for i in 0..self.usuarios.len() {
            if self.usuarios[i].dni == user.dni {
                self.usuarios[i].fiat += fiat;
                camb = true;
            }
        }
        if camb {
            tr = Transaccion::new(
                Some(fecha),
                Some(user),
                Some(fiat),
                Some(&TiposTransaccion::IngresoFiat),
                None,
                None,
                None,
                None,
                None,
            );
        } else {
            panic!("Invalido");
        }
        tr
    }
    /*➢ Retirar fiat por determinado medio: dado un monto de fiat se le descuenta dicho
    monto del balance al usuario y se genera una transacción con la siguiente
    información: fecha, usuario, tipo: retiro fiat, monto y medio (puede ser MercadoPago
    o Transferencia Bancaria)
    Nota:: Tanto para comprar. vender, retirar el usuario debe estar validado.
    Se debe validar siempre que haya balance suficiente para realizar la operación en los casos
    de compra, venta, retiro. */
    pub fn comprar_fiat(&mut self, user: &'a Usuario<'a>, fiat: f64, fecha: &'a Fecha) {
        let a = self.ingresar_fiat(user, fiat, fecha);
        self.historial_transacciones.push(a);
    }
    pub fn retirar_fiat(&mut self, user: &'a Usuario<'a>, fiat: f64, fecha: &'a Fecha) {
        if let Some(a) = self.egresar_fiat(user, fiat, fecha) {
            self.historial_transacciones.push(a);
        }
    }
    fn egresar_fiat(
        &mut self,
        user: &'a Usuario<'a>,
        fiat: f64,
        fecha: &'a Fecha,
    ) -> Option<Transaccion<'a>> {
        if user.validado {
            let tr: Transaccion;
            for i in 0..self.usuarios.len() {
                if self.usuarios[i].dni == user.dni {
                    if self.usuarios[i].fiat > fiat {
                        self.usuarios[i].fiat -= fiat;
                    } else {
                        println!("Saldo insuficiente");
                        return None;
                    }
                }
            }
            tr = Transaccion::new(
                Some(fecha),
                Some(user),
                Some(fiat),
                Some(&TiposTransaccion::EgresoFiat),
                None,
                None,
                None,
                None,
                None,
            );
            Some(tr)
        } else {
            println!("Usuario {} no validado", user.dni);
            None
        }
    }
    /*➢ Comprar determinada criptomoneda: dado un monto de fiat se compra una cantidad
    de determinada criptomoneda, tenga en cuenta que al momento de realizar la
    operación se obtiene del sistema la cotización actual de la criptomoneda para
    acreditar la correspondiente proporción en el balance de la cripto y desacreditar en
    el balance de fiat. Luego de ello se registra la transacción con los siguientes datos:
    fecha, usuario, criptomoneda, tipo: compra de cripto, monto de cripto y cotización.     */
    pub fn comprar_cripto(
        &mut self,
        tipo_cambio: f64,
        monto_fiat: f64,
        user: &'a Usuario<'a>,
        fecha: &'a Fecha,
        cripto: &'a Cripto,
    ) {
        if let Some(tr1) = self.egresar_fiat(user, monto_fiat, fecha) {
            let tr2 = self.ingresar_cripto((cripto, monto_fiat * tipo_cambio), user, fecha);
            let transaccion = Transaccion::new(
                tr1.fecha,
                tr1.usuario,
                tr1.monto_fiat,
                tr1.tipo_fiat,
                tr2.cripto,
                tr2.blockchain,
                tr2.hash,
                tr2.monto_cripto,
                tr2.tipo_cripto,
            );
            self.historial_transacciones.push(transaccion);
        }
    }
    /*➢ Vender determinada criptomoneda: dado un monto de cripto se vende por fiat, tenga
    en cuenta que al momento de realizar la operación se obtiene del sistema la
    cotización actual de la criptomoneda para acreditar la correspondiente proporción en
    el balance de fiat y desacreditar en el balance de la criptomoneda. Luego de ello se
    registra la transacción con los siguientes datos: fecha, usuario, criptomoneda, tipo:
    venta de cripto,monto de cripto y cotización. */
    pub fn vender_cripto(
        &mut self,
        tipo_cambio: f64,
        monto_fiat: f64,
        user: &'a Usuario<'a>,
        fecha: &'a Fecha,
        cripto: &'a Cripto,
    ) {
        if let Some(tr2) = self.egresar_cripto((cripto, monto_fiat * tipo_cambio), user, fecha) {
            let tr1 = self.ingresar_fiat(user, monto_fiat, fecha);
            let transaccion = Transaccion::new(
                tr1.fecha,
                tr1.usuario,
                tr1.monto_fiat,
                tr1.tipo_fiat,
                tr2.cripto,
                tr2.blockchain,
                tr2.hash,
                tr2.monto_cripto,
                tr2.tipo_cripto,
            );
            self.historial_transacciones.push(transaccion);
        }
    }
    fn ingresar_cripto(
        &mut self,
        ingreso: (&'a Cripto<'a>, f64),
        user: &'a Usuario<'a>,
        fecha: &'a Fecha,
    ) -> Transaccion<'a> {
        let tr: Transaccion;
        let mut camb = false;

        for i in 0..self.usuarios.len() {
            let mut aux = self.usuarios.remove(i);
            if aux.dni == user.dni {
                camb = aux.push(ingreso);
            }
            self.usuarios.insert(i, aux);
        }
        if camb {
            tr = Transaccion::new(
                Some(fecha),
                Some(user),
                None,
                None,
                Some(ingreso.0),
                None,
                None,
                Some(ingreso.1),
                Some(&TiposTransaccion::IngresoCripto),
            );
        } else {
            panic!("Invalido");
        }
        tr
    }
    /*➢ Recibir criptomoneda de blockchain: dado un monto de una cripto y una blockchain
    se le acredita al balance de dicha cripto al usuario el monto. Luego se genera una
    transacción con los siguientes datos: fecha, usuario, tipo: recepción cripto,
    blockchain, cripto, monto, cotización(en su lugar voy a generar hash). */
    pub fn recibir_cripto(
        &mut self,
        monto: (&'a Cripto<'a>, f64),
        blockchain: &'a Blockchain,
        user: &'a Usuario,
        fecha: &'a Fecha,
    ) {
        for i in 0..monto.0.chains.len() {
            if monto.0.chains[i] == blockchain {
                let hash = blockchain.get_token(monto);
                let aux = self.ingresar_cripto(monto, user, fecha);
                let transaccion = Transaccion::new(
                    aux.fecha,
                    aux.usuario,
                    None,
                    None,
                    aux.cripto,
                    Some(blockchain),
                    Some(hash),
                    aux.monto_cripto,
                    aux.tipo_cripto,
                );
                self.historial_transacciones.push(transaccion);
            }
        }
    }

    /*➢ Retirar criptomoneda a blockchain: dado un monto de una cripto y una blockchain se
    le descuenta del balance de dicha cripto al usuario el monto, la blockchain devuelve
    un hash que representa una transacción en ella (esto hágalo retornando el nombre
    de la blockchain + un número random). Luego se genera una transacción con los
    siguientes datos: fecha, usuario, tipo: retiro cripto, blockchain, hash, cripto, monto,
    cotización.
     */
    pub fn retirar_cripto(
        &mut self,
        monto: (&'a Cripto<'a>, f64),
        user: &'a Usuario<'a>,
        fecha: &'a Fecha,
        blockchain: &'a Blockchain,
    ) {
        for i in 0..monto.0.chains.len() {
            if monto.0.chains[i] == blockchain {
                if let Some(aux) = self.egresar_cripto(monto, user, &fecha) {
                    let hash = blockchain.send_token(monto);
                    let transaccion = Transaccion::new(
                        aux.fecha,
                        aux.usuario,
                        None,
                        None,
                        aux.cripto,
                        Some(blockchain),
                        Some(hash),
                        aux.monto_cripto,
                        aux.tipo_cripto,
                    );
                    self.historial_transacciones.push(transaccion);
                }
            }
        }
    }
    fn egresar_cripto(
        &mut self,
        ingreso: (&'a Cripto<'a>, f64),
        user: &'a Usuario<'a>,
        fecha: &'a Fecha,
    ) -> Option<Transaccion<'a>> {
        if user.validado {
            let mut hecho: bool = false;
            let transaccion: Transaccion;
            for i in 0..self.usuarios.len() {
                if self.usuarios[i].dni == user.dni {
                    let mut aux = self.usuarios.remove(i);
                    hecho = aux.pull(ingreso);
                    self.usuarios.insert(i, aux);
                }
            }
            if hecho {
                transaccion = Transaccion::new(
                    Some(fecha),
                    Some(user),
                    None,
                    None,
                    Some(ingreso.0),
                    None,
                    None,
                    Some(ingreso.1),
                    Some(&TiposTransaccion::EgresoCripto),
                );
            } else {
                println!("Saldo insuficiente");
                return None;
            }
            Some(transaccion)
        } else {
            println!("Usuario {} no validado", user.dni);
            None
        }
    }
    pub fn get_mayor_cantidad_ventas(&'a self) -> Option<&'a Cripto> {
        let mut res: Vec<(&'a Cripto, i32)> = Vec::new();
        let iter = self.historial_transacciones.iter();
        let mut check = |tr: &'a Transaccion| {
            match tr.tipo_cripto {
                Some(a) => match a {
                    TiposTransaccion::EgresoCripto => {
                        if let Some(cr) = tr.cripto {
                            let mut es = false;
                            for i in 0..res.len() {
                                if cr.nombre == res[i].0.nombre {
                                    res[i].1 += 1;
                                    es = true;
                                }
                            }
                            if !es {
                                res.push((cr, 1));
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            };
        };
        iter.for_each(|x| check(x));
        let iter2 = res.iter();
        let check2 = |par: &(&Cripto, i32), other: &(&Cripto, i32)| -> Ordering {
            if par.1 > other.1 {
                Ordering::Greater
            } else if par.1 == other.1 {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        };
        let max = iter2.max_by(|x, y| check2(x, y));
        if let Some(a) = max {
            Some(a.0)
        } else {
            None
        }
    }
    pub fn get_mayor_cantidad_compras(&'a self) -> Option<&'a Cripto> {
        let mut res: Vec<(&'a Cripto, i32)> = Vec::new();
        let iter = self.historial_transacciones.iter();
        let mut check = |tr: &'a Transaccion| {
            match tr.tipo_cripto {
                Some(a) => match a {
                    TiposTransaccion::IngresoCripto => {
                        if let Some(cr) = tr.cripto {
                            let mut es = false;
                            for i in 0..res.len() {
                                if cr.nombre == res[i].0.nombre {
                                    res[i].1 += 1;
                                    es = true;
                                }
                            }
                            if !es {
                                res.push((cr, 1));
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            };
        };
        iter.for_each(|x| check(x));

        let iter2 = res.iter();
        let check2 = |par: &(&Cripto, i32), other: &(&Cripto, i32)| -> Ordering {
            if par.1 > other.1 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        };
        let max = iter2.max_by(|x, y| check2(x, y));
        if let Some(a) = max {
            Some(a.0)
        } else {
            None
        }
    }
    pub fn get_mayor_volumen_compras(&'a self) -> Option<&'a Cripto> {
        let mut res: Vec<(&'a Cripto, f64)> = Vec::new();
        let iter = self.historial_transacciones.iter();
        let mut check = |tr: &'a Transaccion| {
            match tr.tipo_cripto {
                Some(a) => match a {
                    TiposTransaccion::IngresoCripto => {
                        if let Some(cr) = tr.cripto {
                            let mut es = false;
                            for i in 0..res.len() {
                                if cr.nombre == res[i].0.nombre {
                                    res[i].1 += tr.monto_cripto.unwrap();
                                    es = true;
                                }
                            }
                            if !es {
                                res.push((cr, tr.monto_cripto.unwrap()));
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            };
        };
        iter.for_each(|x| check(x));
        let iter2 = res.iter();
        let check2 = |par: &(&Cripto, f64), other: &(&Cripto, f64)| -> Ordering {
            if par.1 > other.1 {
                Ordering::Greater
            } else if par.1 == other.1 {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        };
        let max = iter2.max_by(|x, y| check2(x, y));
        if let Some(a) = max {
            Some(a.0)
        } else {
            None
        }
    }
    pub fn get_mayor_volumen_ventas(&'a self) -> Option<&'a Cripto> {
        let mut res: Vec<(&'a Cripto, f64)> = Vec::new();
        let iter = self.historial_transacciones.iter();
        let mut check = |tr: &'a Transaccion| {
            {
                match tr.tipo_cripto {
                    Some(a) => match a {
                        TiposTransaccion::EgresoCripto => {
                            if let Some(cr) = tr.cripto {
                                let mut es = false;
                                for i in 0..res.len() {
                                    if cr.nombre == res[i].0.nombre {
                                        res[i].1 += tr.monto_cripto.unwrap();
                                        es = true;
                                    }
                                }
                                if !es {
                                    res.push((cr, tr.monto_cripto.unwrap()));
                                }
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                }
            };
        };
        iter.for_each(|x| check(x));
        let iter2 = res.iter();
        let check2 = |par: &(&Cripto, f64), other: &(&Cripto, f64)| -> Ordering {
            if par.1 > other.1 {
                Ordering::Greater
            } else if par.1 == other.1 {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        };
        let max = iter2.max_by(|x, y| check2(x, y));
        if let Some(a) = max {
            Some(a.0)
        } else {
            None
        }
    }
}
#[derive(Debug)]

pub struct Usuario<'a> {
    nombre: &'a str,
    apellido: &'a str,
    dni: u32,
    validado: bool,
    balance: HashMap<&'a Cripto<'a>, f64>,
    fiat: f64,
}
impl<'a> PartialEq for Usuario<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.dni == other.dni && self.apellido == other.apellido {
            true
        } else {
            false
        }
    }
}
impl<'a> Eq for Usuario<'a> {}
impl<'a> Usuario<'a> {
    pub fn new(nombre: &'a str, apellido: &'a str, dni: u32, validado: bool) -> Usuario<'a> {
        let balance = HashMap::new();
        let fiat = 0.0;
        Usuario {
            nombre,
            apellido,
            dni,
            validado,
            balance,
            fiat,
        }
    }
    pub fn push(&mut self, balance: (&'a Cripto<'a>, f64)) -> bool {
        let es;
        let bal = self.balance.get_mut(balance.0);
        match bal {
            Some(a) => {
                *a += balance.1;
                es = true;
            }
            None => {
                self.balance.insert(balance.0, balance.1);
                es = true;
            }
        }
        es
    }
    pub fn pull(&mut self, balance: (&'a Cripto<'a>, f64)) -> bool {
        let mut es = false;
        let bal = self.balance.get_mut(balance.0);
        match bal {
            Some(a) => {
                if *a >= balance.1 {
                    *a -= balance.1;
                    es = true;
                }
            }
            None => (),
        }
        es
    }
}
/* De las
criptomonedas se conoce: nombre, prefijo y un listado de blockchains donde se pueden
enviar o recibir. De cada blockchain se conoce el nombre, prefijo. */
#[derive(Hash, Debug)]

pub struct Cripto<'a> {
    nombre: &'a str,
    prefijo: &'a str,
    chains: Vec<&'a Blockchain<'a>>,
}
impl<'a> Cripto<'a> {
    pub fn new(nombre: &'a str, prefijo: &'a str, chains: Vec<&'a Blockchain<'a>>) -> Cripto<'a> {
        Cripto {
            nombre,
            prefijo,
            chains,
        }
    }
}
impl<'a> PartialEq for Cripto<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.nombre == other.nombre && self.prefijo == other.prefijo {
            true
        } else {
            false
        }
    }
}
impl<'a> Eq for Cripto<'a> {}
#[derive(Hash, Debug)]
pub struct Blockchain<'a> {
    nombre: &'a str,
    prefijo: &'a str,
}
impl<'a> PartialEq for Blockchain<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.nombre == other.nombre && self.prefijo == other.prefijo {
            true
        } else {
            false
        }
    }
}
impl<'a> Eq for Blockchain<'a> {}
/*Retirar criptomoneda a blockchain: dado un monto de una cripto y una blockchain se
le descuenta del balance de dicha cripto al usuario el monto, la blockchain devuelve
un hash que representa una transacción en ella (esto hágalo retornando el nombre
de la blockchain + un número random). Luego se genera una transacción con los
siguientes datos: fecha, usuario, tipo: retiro cripto, blockchain, hash, cripto, monto,
cotización */
#[allow(unused_variables)]
impl<'a> Blockchain<'a> {
    pub fn new(nombre: &'a str, prefijo: &'a str) -> Blockchain<'a> {
        Blockchain { nombre, prefijo }
    }
    pub fn send_token(&self, monto: (&'a Cripto<'a>, f64)) -> String {
        let charset = "abcdefghijklmnñopqrstuvwxyzABCDEFGHIJKLMNÑOPQRSTUVWXYZ1234567890";
        let hash = generate(20, charset);
        hash
    }
    pub fn get_token(&self, monto: (&'a Cripto<'a>, f64)) -> String {
        let charset = "abcdefghijklmnñopqrstuvwxyzABCDEFGHIJKLMNÑOPQRSTUVWXYZ1234567890";
        let hash = generate(20, charset);
        hash
    }
}
