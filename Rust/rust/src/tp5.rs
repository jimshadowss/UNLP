/*importo todo el ej para hacerlo mas prolijo */
// #[allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use date::Date;
use debug_ignore::DebugIgnore;
pub struct Datos {
    field: [u8; 12],
}
#[derive(Debug)]
pub struct Fecha {
    pub dato: date::Date,
    arreglo_meses: DebugIgnore<Datos>,
}

impl Fecha {
    /*➢ new: que pasando los parámetros correspondientes, crea una Fecha y la retorna. */
    pub fn new(año: u32, mes: u8, dia: u8) -> Fecha {
        let dato = date::Date::new(año, mes, dia);
        let arreglo_meses = Datos {
            field: [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
        }
        .into();

        let f = Fecha {
            dato,
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
        if self.año > 0 && self.dato.day > 0 && self.dato.month > 0 && self.dato.month <= 12 {
            if self.dato.day <= self.arreglo_meses.field[self.dato.month as usize - 1] as u8 {
                res = true;
            } else if self.dato.month == 2 && self.dato.day == 29 && self.es_bisiesto() {
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
        let mut aux = self.dato.day as usize;
        aux += dias;
        while aux > self.arreglo_meses.field[(self.dato.month - 1) as usize] as usize {
            aux -= self.arreglo_meses.field[(self.dato.month - 1) as usize] as usize;
            if self.dato.month == 2 && self.es_bisiesto() {
                aux -= 1;
            }
            if self.dato.month < 12 {
                self.dato.month += 1;
            } else {
                self.dato.month = 1;
                self.año += 1;
            }
        }
        self.dato.day = aux as u8;
    }
    /*
    ➢ restar_dias(dias): resta la cantidad de días a la fecha, modificándose */
    pub fn restar_dias(&mut self, dias: usize) {
        let mut aux: usize = self.dato.day as usize;
        aux -= dias;
        while aux < 1 {
            aux += self.arreglo_meses.field[(self.dato.month - 1) as usize] as usize;
            if self.dato.month == 2 && self.es_bisiesto() {
                aux += 1;
            }
            if self.dato.month > 1 {
                self.dato.month -= 1;
            } else {
                self.dato.month = 12;
                self.año -= 1;
            }
        }
        self.dato.day = aux as u8;
    }
    /*
    ➢ es_mayor(una_fecha): que retorna true si la fecha que recibe el mensaje es mayor a
    la fecha pasada por parámetro.. */
    pub fn es_mayor(&self, f: &Fecha) -> bool {
        let mut res = false;
        if f.dato < self.dato {
            res = true;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_fecha() {
        let año = 2015;
        let mes = 10;
        let dia = 9;
        let f = Fecha::new(año, mes, dia);
        assert!(f.año == año && f.dato.month == mes && f.dato.day == dia);
    }
    #[test]
    #[should_panic]
    fn new_fecha1() {
        let año = 15;
        let mes = 15;
        let dia = 9;
        let f = Fecha::new(año, mes, dia);
        assert!(f.año == año && f.dato.month == mes && f.dato.day == dia);
    }
    #[test]
    #[should_panic]
    fn new_fecha2() {
        let año = 2020;
        let mes = 10;
        let dia = 32;
        let f = Fecha::new(año, mes, dia);
        assert!(f.año == año && f.dato.month == mes && f.dato.day == dia);
    }
    #[test]
    fn es_fecha_valida() {
        let año = 2020;
        let mes = 2;
        let dia = 29;
        let f = Fecha::new(año, mes, dia);
        f.es_fecha_valida();
    }
}
/*7- Defina una estructura llamada ConcesionarioAuto donde se conoce el nombre, la
dirección y tiene una capacidad máxima para albergar X cantidad de autos. De los autos se
conocen los campos de la marca, modelo, año, precio bruto y color que pueden ser:rojo,
verde, azul, amarillo, blanco o negro.
Para dichas estructuras implemente los siguientes métodos:
❖ ConcesionarioAuto:
➢ new: que pasando los parámetros correspondientes, crea un
ConcesionarioAuto y lo retorna. */

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
#[test]
#[should_panic]

fn crear_file_test() {
    let path = "coverage/";
    let esc = "ikasdfbg";
    crear_file(path, &serde_json::to_string_pretty(&esc).unwrap())
}
#[derive(Debug)]
pub struct ConcesionarioAuto<'a> {
    nombre: String,
    direccion: String,
    cant_max: usize,
    autos: Vec<&'a Auto>,
    path: &'a str,
}
impl<'a> ConcesionarioAuto<'a> {
    pub fn new(nombre: String, direccion: String, cant_max: usize) -> ConcesionarioAuto<'a> {
        let autos = Vec::new();
        let path = "src/autos.json";
        ConcesionarioAuto {
            nombre,
            direccion,
            cant_max,
            autos,
            path,
        }
    }
    /*➢ agregar_auto(auto): agrega un auto a la lista de autos que tiene sin superar
    la máxima cantidad para albergarlos y retorna true, en caso de que lo supere
    no lo agrega y retorna false */
    pub fn agregar_auto(&mut self, auto: &'a Auto) -> Result<bool, bool> {
        let check = |x, y| -> Ordering {
            if x < y {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        };
        match check(self.autos.len(), self.cant_max) {
            Ordering::Greater => {
                self.autos.push(auto);
                crear_file(self.path, &self.autos);
                Ok(true)
            }
            _ => Err(false),
        }
    }
    /*➢ eliminar_auto(auto): elimina un auto de la lista de autos */
    pub fn eliminar_auto(&mut self, auto: &Auto) -> bool {
        let mut es = false;
        if !self.autos.is_empty() {
            for i in 0..self.autos.len() {
                if !es && self.autos[i] == auto {
                    self.autos.remove(i);
                    self.cant_max -= 1;

                    //--

                    crear_file(self.path, &self.autos);

                    //--
                    es = true;
                }
            }
        }
        es
    }
    /*➢ buscar_auto(auto): busca un auto y si lo encuentra lo retorna */
    pub fn buscar_auto(&self, auto: &Auto) -> Option<&Auto> {
        let mut car: Option<&Auto> = None;
        if !self.autos.is_empty() {
            for i in 0..self.autos.len() {
                if self.autos[i] == auto {
                    car = Some(&self.autos[i]);
                }
            }
        }
        car
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Auto {
    marca: String,
    modelo: u32,
    año: u16,
    precio: f32,
    color: Colores,
}

impl PartialEq for Auto {
    fn eq(&self, other: &Self) -> bool {
        if self.marca.eq(&other.marca) && self.año == other.año && self.modelo == other.modelo {
            true
        } else {
            false
        }
    }
}
/*❖ Auto:
➢ new: que pasando los parámetros correspondientes, crea un Auto y lo
retorna */
impl Auto {
    pub fn new(año: u16, color: Colores, marca: String, modelo: u32, precio: f32) -> Auto {
        Auto {
            año,
            marca,
            modelo,
            precio,
            color,
        }
    }
    /*➢ calcular_precio: retorna el precio del auto aplicando los siguientes criterios:
    ■ si es de color primario le aplica un recargo del 25%, sino le aplica un
    descuento del 10%.
    ■ si la marca es BMW le aplica un recargo del 15%-
    ■ si el año es menor a 2000 le aplica un descuento del 5% */

    pub fn calcular_precio(&self) -> f32 {
        let mut precio: f32 = match self.color {
            Colores::Rojo => self.precio * 1.25,
            Colores::Azul => self.precio * 1.25,
            Colores::Amarillo => self.precio * 1.25,
            Colores::Blanco => self.precio * 0.9,
            Colores::Negro => self.precio * 0.9,
            Colores::Verde => self.precio * 0.9,
        };
        if self.marca.eq("BMW") {
            precio *= 1.15;
        }

        if self.año < 2000 {
            precio *= 0.95;
        }
        precio
    }
}
#[test]
fn calcular_precio_test() {
    let precio = 10.0;
    let auto_amarillo = Auto::new(2020, Colores::Amarillo, "".to_string(), 3000, precio);
    let auto_blanco = Auto::new(2020, Colores::Blanco, "".to_string(), 3000, precio);
    let aux3 = auto_blanco.calcular_precio();
    let auto_negro = Auto::new(2020, Colores::Negro, "".to_string(), 3000, precio);
    let aux4 = auto_negro.calcular_precio();
    let auto_verde = Auto::new(2020, Colores::Verde, "".to_string(), 3000, precio);
    let aux5 = auto_verde.calcular_precio();
    let auto_rojo = Auto::new(2020, Colores::Rojo, "".to_string(), 3000, precio);
    let aux6 = auto_rojo.calcular_precio();
    let auto_bmw_90s = Auto::new(1999, Colores::Azul, "BMW".to_string(), 1999, precio);
    let aux = auto_amarillo.calcular_precio();
    let aux2 = auto_bmw_90s.calcular_precio();
    assert_eq!(aux, precio * 1.25);
    assert_eq!(aux2, precio * 1.25 * 1.15 * 0.95);
    assert_eq!(aux3, precio * 0.9);
    assert_eq!(aux4, precio * 0.9);
    assert_eq!(aux5, precio * 0.9);
    assert_eq!(aux6, precio * 1.25);
}
#[test]
fn agregar_auto_test() {
    let auto = Auto::default();
    let mut conc = ConcesionarioAuto::new("".to_string(), "".to_string(), 10);
    let res = conc.agregar_auto(&auto);
    assert!(match res {
        Ok(_) => true,
        Err(_) => false,
    })
}
#[test]
#[allow(unused_must_use)]

fn agregar_auto_test2() {
    let auto = Auto::default();
    let auto2 = Auto::default();
    let mut conc = ConcesionarioAuto::new("".to_string(), "".to_string(), 1);
    conc.agregar_auto(&auto);
    let res = conc.agregar_auto(&auto2);
    assert!(match res {
        Ok(_) => false,
        Err(_) => true,
    })
}
#[test]
#[allow(unused_must_use)]
fn eliminar_auto_test() {
    let auto = Auto::default();
    let mut conc = ConcesionarioAuto::new("".to_string(), "".to_string(), 10);
    conc.agregar_auto(&auto);
    conc.eliminar_auto(&auto);
    let res = conc.buscar_auto(&auto);
    assert!(match res {
        Some(_) => false,
        None => true,
    })
}
#[test]
#[allow(unused_must_use)]
fn eliminar_auto_test2() {
    let auto = Auto::default();
    let mut conc = ConcesionarioAuto::new("".to_string(), "".to_string(), 10);
    let res = conc.eliminar_auto(&auto);
    assert!(!res, "Esperaba false, pero dio {}", res)
}
#[test]
#[allow(unused_must_use)]

fn buscar_auto_test() {
    let auto = Auto::default();
    let auto2 = Auto::new(21, Colores::Blanco, "a".to_string(), 46, 45.0);
    let mut conc = ConcesionarioAuto::new("".to_string(), "".to_string(), 10);
    conc.agregar_auto(&auto);
    let res = conc.buscar_auto(&auto);
    assert!(match res {
        Some(_) => true,
        None => false,
    });
    let res2 = conc.buscar_auto(&auto2);
    assert!(match res2 {
        Some(_) => false,
        None => true,
    })
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum Colores {
    #[default]
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

/* 2- En base al ejercicio 8 del tp#3 implemente lo siguiente:
a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
de por lo menos 90%
b- Una vez obtenido dicho coverage, las canciones de la playlist deben ser
guardadas en un archivo en formato JSON, por lo tanto las operaciones que agreguen,
quiten o modifiquen la playlist deben estar respaldadas sobre dicho archivo.
No debe modificar los tests hechos en el punto a. Si puede agregar más en caso de que
haga métodos nuevos. Recuerde también que se debe seguir manteniendo un coverage de
al menos 90%,*/

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Cancion {
    title: String,
    artist: String,
    genre: Generos,
}
impl Cancion {
    pub fn new(title: String, artist: String, genre: Generos) -> Cancion {
        Cancion {
            title,
            artist,
            genre,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PlayList<'a> {
    nombre: String,
    lista: Vec<&'a Cancion>,
    cant: usize,
    path: &'a str,
}
impl<'a> PlayList<'a> {
    pub fn new(
        nombre: String,
        lista: Vec<&'a Cancion>,
        cant: usize,
        path: &'a str,
    ) -> PlayList<'a> {
        PlayList {
            nombre,
            lista,
            cant,
            path,
        }
    }
    pub fn agregar_cancion(&mut self, song: &'a Cancion) {
        self.lista.insert(self.cant, song);
        self.cant += 1;
        crear_file(self.path, &self.lista);
    }
    pub fn eliminar_cancion(&mut self, song: &Cancion) {
        let mut es = false;
        for i in 0..self.lista.len() {
            if !es && self.lista[i].title.eq(&song.title) && self.lista[i].artist.eq(&song.artist) {
                es = true;
                self.lista.remove(i);
                self.cant -= 1;
            }
        }
        crear_file(self.path, &self.lista);
    }
    pub fn mover_cancion(&mut self, song: &'a Cancion, pos: usize) {
        for i in 0..self.lista.len() {
            if self.lista[i] == song {
                let aux = self.lista.remove(i);
                self.lista.insert(pos - 1, aux);
            }
        }
        crear_file(self.path, &self.lista);
    }
    pub fn buscar_cancion_por_nombre(&self, nombre: String) -> Option<&Cancion> {
        let mut res: Option<&Cancion> = None;
        for i in 0..self.cant {
            if self.lista[i].title.eq(&nombre) {
                let aux = self.lista[i];
                res = Some(aux);
            }
        }
        res
    }
    pub fn obtener_genero(&self, genero: Generos) -> PlayList {
        let nombre: String = match genero {
            Generos::Rock => "Lista de Rock".to_string(),
            Generos::Jazz => "Lista de Jazz".to_string(),
            Generos::Pop => "Lista de Pop".to_string(),
            Generos::Rap => "Lista de Rap".to_string(),
            Generos::Otros => "Lista de Otros".to_string(),
        };
        let lista = Vec::new();
        let cant: usize = 0;
        let path = self.path;
        let mut p = PlayList {
            nombre,
            lista,
            cant,
            path,
        };
        for i in 0..self.cant {
            match (&self.lista[i].genre, &genero) {
                (Generos::Rock, Generos::Rock)
                | (Generos::Jazz, Generos::Jazz)
                | (Generos::Otros, Generos::Otros)
                | (Generos::Pop, Generos::Pop)
                | (Generos::Rap, Generos::Rap) => p.agregar_cancion(self.lista[i]),
                _ => (),
            }
        }
        p
    }
    pub fn obtener_artista(&self, artista: String) -> PlayList {
        let mut nombre = String::from("Lista de ");
        nombre.push_str(artista.as_str());
        let lista = Vec::new();
        let cant = 0;
        let path = self.path;
        let mut p = PlayList {
            nombre,
            lista,
            cant,
            path,
        };
        for i in 0..self.cant {
            if self.lista[i].artist.eq(&artista) {
                p.agregar_cancion(self.lista[i]);
            }
        }
        p
    }
    pub fn modificar_nombre(&mut self, nombre: String) {
        self.nombre = nombre;
    }
    pub fn eliminar_canciones(&mut self) {
        self.lista.clear();
        self.cant = 0;
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Generos {
    #[default]
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

#[test]
fn new_cancion_test() {
    let title = "Algo".to_string();
    let artist = "Algo".to_string();
    let genre = Generos::Jazz;
    let c = Cancion::new(title.clone(), artist.clone(), genre);
    assert!(c.artist == artist && c.genre == genre && c.title == title)
}

#[test]
fn new_playlist_test() {
    let nombre = "Algo".to_string();
    let lista = Vec::new();
    let cant = 0;
    let p = PlayList::new(nombre.clone(), lista.clone(), cant, "");
    assert!(p.nombre == nombre && p.lista == lista && p.cant == cant);
}

#[test]
fn buscar_cancion_por_nombre_test() {
    let mut pl = PlayList::default();
    pl.path = "src/canciones.json";
    let s1 = Cancion::default();
    let s2 = Cancion::new("t1".to_string(), "a".to_string(), Generos::Jazz);
    pl.agregar_cancion(&s1);
    pl.agregar_cancion(&s2);
    let res = pl.buscar_cancion_por_nombre(s2.title.clone()).unwrap();
    assert_eq!(res, &s2);
}

#[test]
fn agregar_cancion_test() {
    let mut pl = PlayList::default();
    pl.path = "src/canciones.json";
    let s1 = Cancion::default();
    let s2 = Cancion::new("t1".to_string(), "a".to_string(), Generos::Jazz);
    pl.agregar_cancion(&s1);
    pl.agregar_cancion(&s2);
    let mut es = false;
    for i in 0..pl.lista.len() {
        if pl.lista[i] == &s2 {
            es = true;
        }
    }
    assert!(es);
}

#[test]
fn eliminar_cancion_test() {
    let mut pl = PlayList::default();
    pl.path = "src/canciones.json";
    let s1 = Cancion::default();
    let s2 = Cancion::new("t1".to_string(), "a".to_string(), Generos::Jazz);
    pl.agregar_cancion(&s1);
    pl.agregar_cancion(&s2);
    pl.eliminar_cancion(&s2);
    assert_eq!(pl.buscar_cancion_por_nombre(s2.title.clone()), None);
}

#[test]
fn mover_cancion_test() {
    let mut pl = PlayList::default();
    pl.path = "src/canciones.json";
    let s1 = Cancion::default();
    let s2 = Cancion::new("t1".to_string(), "a".to_string(), Generos::Jazz);
    pl.agregar_cancion(&s1);
    pl.agregar_cancion(&s2);
    pl.mover_cancion(&s1, 2);
    assert!(pl.lista[0] == &s2);
    assert_ne!(pl.lista[1], &s2);
}

#[test]
fn obtener_genero_test() {
    let mut pl = PlayList::default();
    pl.path = "src/canciones.json";
    let c1 = Cancion::new("1".to_string(), "1".to_string(), Generos::Jazz);
    let c2 = Cancion::new("2".to_string(), "2".to_string(), Generos::Otros);
    let c3 = Cancion::new("3".to_string(), "3".to_string(), Generos::Jazz);
    let c4 = Cancion::new("4".to_string(), "4".to_string(), Generos::Pop);
    let c5 = Cancion::new("4".to_string(), "4".to_string(), Generos::Rap);
    let c6 = Cancion::new("4".to_string(), "4".to_string(), Generos::Rock);
    pl.agregar_cancion(&c1);
    pl.agregar_cancion(&c2);
    pl.agregar_cancion(&c3);
    pl.agregar_cancion(&c4);
    pl.agregar_cancion(&c5);
    pl.agregar_cancion(&c6);
    let res = pl.obtener_genero(Generos::Jazz);
    assert!(res.lista[0] == &c1 && res.lista[1] == &c3 && res.lista.len() == 2);
    let res = pl.obtener_genero(Generos::Otros);
    assert!(res.lista[0] == &c2 && res.lista.len() == 1);
    let res = pl.obtener_genero(Generos::Pop);
    assert!(res.lista[0] == &c4 && res.lista.len() == 1);
    let res = pl.obtener_genero(Generos::Rap);
    assert!(res.lista[0] == &c5 && res.lista.len() == 1);
    let res = pl.obtener_genero(Generos::Rock);
    assert!(res.lista[0] == &c6 && res.lista.len() == 1);
}
#[test]
fn obtener_artista_test() {
    let mut pl = PlayList::default();
    pl.path = "src/canciones.json";
    let c1 = Cancion::new("1".to_string(), "1".to_string(), Generos::Jazz);
    let c2 = Cancion::new("2".to_string(), "2".to_string(), Generos::Otros);
    let c3 = Cancion::new("1".to_string(), "1".to_string(), Generos::Rock);
    let c4 = Cancion::new("4".to_string(), "4".to_string(), Generos::Pop);
    pl.agregar_cancion(&c1);
    pl.agregar_cancion(&c2);
    pl.agregar_cancion(&c3);
    pl.agregar_cancion(&c4);
    let res = pl.obtener_artista("1".to_string());
    assert!(res.lista[0] == &c1 && res.lista[1] == &c3 && res.lista.len() == 2);
}
#[test]
fn modificar_nombre_test() {
    let mut pl = PlayList::default();
    pl.path = "src/canciones.json";
    pl.modificar_nombre("1".to_string());
    assert!(pl.nombre.eq("1"));
}

#[test]
fn eliminar_canciones_test() {
    let mut pl = PlayList::default();
    pl.path = "src/canciones.json";
    let c1 = Cancion::new("1".to_string(), "1".to_string(), Generos::Jazz);
    let c2 = Cancion::new("2".to_string(), "2".to_string(), Generos::Otros);
    let c3 = Cancion::new("1".to_string(), "1".to_string(), Generos::Rock);
    let c4 = Cancion::new("4".to_string(), "4".to_string(), Generos::Pop);
    pl.agregar_cancion(&c1);
    pl.agregar_cancion(&c2);
    pl.agregar_cancion(&c3);
    pl.agregar_cancion(&c4);
    pl.eliminar_canciones();
    assert!(pl.lista.len() == 0 && pl.cant == 0);
}

/* 3- En base al ejercicio 9 del tp#3 implemente lo siguiente:
a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
de por lo menos 90%
b - Ahora el registro de atenciones debe persistir en un archivo en formato JSON, es
decir todas la operaciones que lectura, agregar y modificación de atenciones se realizan
sobre un archivo.No debe modificar los tests hechos en el punto a. Si puede agregar más
en caso de que haga métodos nuevos para cumplir con este punto. Recuerde también que
se debe seguir manteniendo un coverage de al menos 90%*/

#[derive(Debug)]
pub struct Cadena {
    nombre: String,
    pub vets: Vec<Veterinaria>,
}
impl<'a> Cadena {
    pub fn new(nombre: String) -> Cadena {
        let vets = Vec::new();
        Cadena { nombre, vets }
    }
}
#[derive(Debug)]
pub struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u32,
    atencion: VecDeque<Mascota>,
    reg: Vec<DatosReg>,
}
#[derive(Debug)]
pub struct Mascota {
    nombre: String,
    edad: u8,
    especie: Animales,
    dueño: Dueño,
}
impl Mascota {
    pub fn new(nombre: String, edad: u8, especie: Animales, dueño: Dueño) -> Mascota {
        Mascota {
            nombre,
            edad,
            especie,
            dueño,
        }
    }
}
#[derive(Debug)]
pub enum Animales {
    Perro,
    Gato,
    Caballo,
    Otros,
}
#[derive(Debug)]
pub struct Dueño {
    nombre: String,
    direccion: String,
    contacto: u64,
}
impl Dueño {
    pub fn new(nombre: String, direccion: String, contacto: u64) -> Dueño {
        Dueño {
            nombre,
            direccion,
            contacto,
        }
    }
}
#[derive(Debug)]
pub struct DatosReg {
    datos: Mascota,
    diagnostico: String,
    tratamiento: String,
    prox_visita: Option<Fecha>,
}
impl DatosReg {
    pub fn new(
        datos: Mascota,
        diagnostico: String,
        tratamiento: String,
        prox_visita: Option<Fecha>,
    ) -> DatosReg {
        DatosReg {
            datos,
            diagnostico,
            tratamiento,
            prox_visita,
        }
    }
}
impl Veterinaria {
    pub fn new(nombre: String, direccion: String, id: u32) -> Veterinaria {
        let atencion = VecDeque::new();
        let reg = Vec::new();
        Veterinaria {
            nombre,
            direccion,
            id,
            atencion,
            reg,
        }
    }
    pub fn encolar_paciente(&mut self, paciente: Mascota) {
        self.atencion.push_back(paciente);
    }
    pub fn encolar_urgente(&mut self, paciente: Mascota) {
        self.atencion.push_front(paciente);
    }
    fn atender(&mut self) -> Mascota {
        match self.atencion.front() {
            Some(_) => self.atencion.pop_front().unwrap(),
            None => panic!("Cola vacia"),
        }
    }
    pub fn eliminar_paciente(&mut self, paciente: Mascota) {
        for i in 1..self.atencion.len() {
            if self.atencion[i]
                .nombre
                .eq_ignore_ascii_case(paciente.nombre.as_str())
                && self.atencion[i]
                    .dueño
                    .nombre
                    .eq_ignore_ascii_case(paciente.dueño.nombre.as_str())
            {
                self.atencion.remove(i);
            }
        }
    }
    pub fn registrar_atencion(
        &mut self,
        diagnostico: String,
        tratamiento: String,
        prox_fecha: Fecha,
    ) {
        let datos: DatosReg =
            DatosReg::new(self.atender(), diagnostico, tratamiento, Some(prox_fecha));
        self.procesar_atencion(datos);
    }
    pub fn procesar_atencion(&mut self, datos: DatosReg) {
        self.reg.push(datos);
    }
    pub fn buscar_atencion(
        &self,
        nombre_mascota: String,
        nombre_dueño: String,
        contacto: u64,
    ) -> Option<&DatosReg> {
        let mut datos = None;
        for i in 0..self.reg.len() {
            if self.reg[i]
                .datos
                .nombre
                .eq_ignore_ascii_case(&nombre_mascota)
                && self.reg[i]
                    .datos
                    .dueño
                    .nombre
                    .eq_ignore_ascii_case(&nombre_dueño)
                && self.reg[i].datos.dueño.contacto == contacto
            {
                datos = Some(&self.reg[i]);
            }
        }
        datos
    }
    pub fn modificar_diagnostico(&mut self, atencion: &DatosReg, nuevo_diagnostico: String) {
        for i in 0..self.reg.len() {
            if self.reg[i].datos.nombre == atencion.datos.nombre
                && self.reg[i].diagnostico == atencion.diagnostico
                && self.reg[i].datos.dueño.nombre == atencion.datos.dueño.nombre
            {
                self.reg[i].diagnostico = nuevo_diagnostico.clone();
            }
        }
    }
    pub fn modificar_prox_visita(&mut self, atencion: &DatosReg, nueva_fecha: Fecha) {
        let mut index = -1;
        for i in 0..self.reg.len() - 1 {
            if self.reg[i].datos.nombre == atencion.datos.nombre
                && self.reg[i].diagnostico == atencion.diagnostico
                && self.reg[i].datos.dueño.nombre == atencion.datos.dueño.nombre
            {
                index = i as i32;
            }
        }
        if index >= 0 {
            self.reg[index as usize].prox_visita = Some(nueva_fecha);
        } else {
            panic!("No se encontro atencion");
        }
    }
    pub fn eliminar_atencion(&mut self, atencion: &DatosReg) -> bool {
        let mut eliminado = false;
        for i in 0..self.reg.len() {
            if self.reg[i].datos.nombre == atencion.datos.nombre
                && self.reg[i].diagnostico == atencion.diagnostico
                && self.reg[i].datos.dueño.nombre == atencion.datos.dueño.nombre
            {
                self.reg.remove(i);
                eliminado = true;
            }
        }
        eliminado
    }
}
