#[allow(dead_code)]
use std::{collections::VecDeque, f64, fmt::Debug};

/*1- Escribir un programa que defina una estructura Persona que tenga campos para el
nombre, la edad y la dirección(que puede ser nulo al momento de la creación de una
persona). Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea una Persona y la retorna.
➢ imprimir: que imprime los datos de la persona sobre el mensaje ejecutado por ej:
person.imprimir() , donde person es una variable del tipo Persona.
➢ obtener_edad: retorna la edad de la persona.
➢ actualizar_direccion(nueva_direccion) */

pub struct Persona {
    nombre: String,
    edad: u32,
    direccion: String,
}
impl Persona {
    pub fn new(nombre: String, edad: u32, direccion: String) -> Persona {
        Persona {
            nombre,
            edad,
            direccion,
        }
    }
    pub fn imprimir(&self) -> String {
        let mut s = "Nombre: ".to_string();
        s.push_str(&self.nombre);
        s.push_str(" Edad: ");
        s.push_str(&self.edad.to_string());
        s.push_str(" Direccion: ");
        s.push_str(&self.direccion);
        s
    }
    pub fn obtener_edad(&self) -> u32 {
        self.edad
    }
    pub fn actualizar_direccion(&mut self, nueva_direccion: String) {
        self.direccion = nueva_direccion;
    }
}
/*2- Escribir un programa que defina la estructura Rectángulo que tenga campos para la
longitud y el ancho. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Rectángulo y lo
retorna.
➢ calcular_area: calcular el área y la retorna.
➢ calcular_perimetro: calcula el perímetro y lo retorna.
➢ es_cuadrado: retorna true si es cuadrado, false caso contrario */
pub struct Rectangulo {
    longitud: f64,
    ancho: f64,
}
impl Rectangulo {
    pub fn new(longitud: f64, ancho: f64) -> Rectangulo {
        Rectangulo { longitud, ancho }
    }
    pub fn calcular_area(self) -> f64 {
        self.longitud * self.ancho
    }
    pub fn calcular_perimetro(self) -> f64 {
        2.0 * (self.longitud + self.ancho)
    }
    pub fn es_cuadrado(self) -> bool {
        self.longitud == self.ancho
    }
}
/*3- Escribir un programa que defina una estructura Fecha que tenga campos para el día, el
mes y el año. Para dicha estructura implemente los siguientes métodos:
*/
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
//-------------------------------------------------------------

/*4- Escribir un programa que defina la estructura Triángulo que tenga campos para las
longitudes de sus tres lados. Para dicha estructura implemente los siguientes métodos:*/
#[derive(Debug)]
pub struct Triangulo {
    lado1: f64,
    lado2: f64,
    lado3: f64,
    tipo: TipoTriangulo,
}
/*tipo
➢ new: que pasando los parámetros correspondientes, crea un Triángulo y lo retorna.*/
#[derive(Debug)]
pub enum TipoTriangulo {
    Isosceles,
    Escaleno,
    Equilatero,
    Ninguno,
}
impl Triangulo {
    pub fn new(lado1: f64, lado2: f64, lado3: f64) -> Triangulo {
        let tipos: TipoTriangulo;
        if lado1 == lado2 && lado2 == lado3 {
            tipos = TipoTriangulo::Equilatero;
        } else if lado1 != lado2 && lado1 != lado3 && lado2 != lado3 {
            tipos = TipoTriangulo::Escaleno;
        } else {
            tipos = TipoTriangulo::Isosceles;
        }

        Triangulo {
            lado1,
            lado2,
            lado3,
            tipo: tipos,
        }
    }
    /*
    ➢ determinar_tipo: retorna el tipo del triángulo, los tipos pueden ser equilátero,
    isósceles o escaleno.*/
    pub fn determinar_tipo(&self) -> TipoTriangulo {
        let tipo: TipoTriangulo;
        if self.lado1 == self.lado2 && self.lado2 == self.lado3 {
            tipo = TipoTriangulo::Equilatero;
        } else if self.lado1 != self.lado2 && self.lado1 != self.lado3 && self.lado2 != self.lado3 {
            tipo = TipoTriangulo::Escaleno;
        } else {
            tipo = TipoTriangulo::Isosceles;
        }
        tipo
    }
    /*
    ➢ calcular_area: calcular el área y la retorna.*/
    pub fn calcular_area(&self) -> f64 {
        let s: f64;

        let res = match self.tipo {
            TipoTriangulo::Equilatero => ((3 as f64).sqrt() / 4.0) * self.lado1.powi(2),
            TipoTriangulo::Isosceles => {
                /* (1/4) * √[4a^2 - b^2] * √[4c^2 - b^2] */
                s = (self.lado1 + self.lado2 + self.lado3) / 2.0;
                (s * (s - self.lado1) * (s - self.lado2) * (s - self.lado3)).sqrt()
            }
            TipoTriangulo::Escaleno => {
                s = (self.lado1 + self.lado2 + self.lado3) / 2.0;
                (s * (s - self.lado1) * (s - self.lado2) * (s - self.lado3)).sqrt()
            }
            TipoTriangulo::Ninguno => -1.0,
        };
        res
    }
    /*
    ➢ calcular_perimetro: calcula el perímetro y lo retorna  */
    pub fn calcular_perimetro(&self) -> f64 {
        self.lado1 + self.lado2 + self.lado3
    }
}
/*5- Escribir un programa que defina una estructura Producto que tenga campos para el
nombre, el precio bruto y un número identificatorio. Para dicha estructura implemente los
siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Producto y lo retorna */
#[derive(Debug)]
pub struct Producto {
    nombre: String,
    precio: f64,
    id: i32,
}
impl Producto {
    pub fn new(nombre: String, precio: f64, id: i32) -> Producto {
        Producto { nombre, precio, id }
    }
    /*➢ calcular_impuestos(porcentaje_de_impuestos): retorna el valor de impuestos sobre
    el precio bruto */
    pub fn calcular_impuestos(&self, porcentaje: f64) -> f64 {
        self.precio + self.precio * porcentaje / 100.0
    }
    /*➢ aplicar_descuento(porcentaje_de_descuento): retorna el valor del porcentaje de
    descuento sobre el precio bruto */
    pub fn aplicar_descuento(&self, porcentaje: f64) -> f64 {
        self.precio - self.precio * porcentaje / 100.0
    }
    /*➢ calcular_precio_total(porcentaje_de_impuestos, porcentaje_descuento): retorna el
    precio total a pagar aplicando impuesto y descuento. Tenga en cuenta que los
    parámetros son opcionales */
    pub fn calcular_precio_total(&self, porcentaje_imp: f64, porcentaje_desc: f64) -> f64 {
        let aux = self.calcular_impuestos(porcentaje_imp);
        aux - aux * porcentaje_desc / 100.0
    }
}

/*6- Escribir un programa que defina una estructura Estudiante que tenga campos para el
nombre, el número de identificación y las calificaciones de exámenes. De cada Examen se
conoce el nombre de la materia y la nota. Para dichas estructuras implemente los siguientes
métodos:
❖ Examen:
➢ new: que pasando los parámetros correspondientes, crea un Examen y lo
retorna. */
use ::std::collections::HashMap;

#[derive(Debug)]
pub struct Estudiante {
    nombre: String,
    id: i32,
    examenes: HashMap<String, f32>,
}

/*❖ Estudiante:
➢ new: que pasando los parámetros correspondientes, crea un Estudiante y lo
retorna */
impl<'a> Estudiante {
    pub fn new(nombre: String, id: i32) -> Estudiante {
        Estudiante {
            nombre,
            id,
            examenes: HashMap::new(),
        }
    }
    pub fn añadir_examen(&mut self, materia: String, nota: f32) {
        self.examenes.insert(materia, nota);
    }
    /*➢ obtener_promedio: retorna el promedio de las notas. */

    pub fn obtener_promedio(self) -> f32 {
        let mut res = 0.0;
        let mut i = 0.0;
        let examenes = self.examenes;

        for (_, nota) in examenes {
            res += nota as f32;
            i += 1.0;
        }
        res / &i
    }
    /*➢ obtener_calificacion_mas_alta: retorna la nota más alta. */
    pub fn obtener_calificacion_mas_alta(&self) -> f32 {
        let mut nota_alta: f32 = -1.0;
        for (_, nota) in &self.examenes {
            if nota > &nota_alta {
                nota_alta = *nota;
            }
        }

        nota_alta
    }
    /*➢ obtener_calificacion_mas_baja: retorna la nota más baja.
    Nota: Tenga en cuenta que el Estudiante puede tener entre 0 y n notas de examen */
    pub fn obtener_calificacion_mas_baja(&self) -> f32 {
        let mut nota_baja: f32 = 9999.9;
        for (_, nota) in &self.examenes {
            if nota < &nota_baja {
                nota_baja = *nota;
            }
        }

        nota_baja
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
#[derive(Debug)]
pub struct ConcesionarioAuto {
    nombre: String,
    direccion: String,
    cant: usize,
    autos: [Auto; 8],
}
impl ConcesionarioAuto {
    pub fn new(nombre: String, direccion: String) -> ConcesionarioAuto {
        let autos: [Auto; 8] = [
            Auto::new(0, Colores::Amarillo, "".to_string(), 0, 0.0),
            Auto::new(0, Colores::Amarillo, "".to_string(), 0, 0.0),
            Auto::new(0, Colores::Amarillo, "".to_string(), 0, 0.0),
            Auto::new(0, Colores::Amarillo, "".to_string(), 0, 0.0),
            Auto::new(0, Colores::Amarillo, "".to_string(), 0, 0.0),
            Auto::new(0, Colores::Amarillo, "".to_string(), 0, 0.0),
            Auto::new(0, Colores::Amarillo, "".to_string(), 0, 0.0),
            Auto::new(0, Colores::Amarillo, "".to_string(), 0, 0.0),
        ];
        let cant = 0;
        ConcesionarioAuto {
            nombre,
            direccion,
            cant,
            autos,
        }
    }
    /*➢ agregar_auto(auto): agrega un auto a la lista de autos que tiene sin superar
    la máxima cantidad para albergarlos y retorna true, en caso de que lo supere
    no lo agrega y retorna false */
    pub fn agregar_auto(&mut self, auto: Auto) -> bool {
        let es: bool;
        if self.autos.len() > self.cant {
            self.autos[self.cant] = auto;
            self.cant += 1;
            es = true;
        } else {
            es = false;
        }
        es
    }
    /*➢ eliminar_auto(auto): elimina un auto de la lista de autos */
    pub fn eliminar_auto(&mut self, auto: Auto) {
        let aux = Auto::new(0, Colores::Amarillo, "".to_string(), 0, 0.0);
        let mut encontro = false;
        for i in 0..self.autos.len() - 1 {
            if self.autos[i].marca == auto.marca
                && self.autos[i].año == auto.año
                && self.autos[i].modelo == auto.modelo
            {
                encontro = true;
                for j in i..self.autos.len() {
                    if j < self.autos.len() - 1 {
                        self.autos[j].año = self.autos[j + 1].año;
                        self.autos[j].color = self.autos[j + 1].color;
                        self.autos[j].marca = self.autos[j + 1].marca.clone();
                        self.autos[j].modelo = self.autos[j + 1].modelo;
                        self.autos[j].precio = self.autos[j + 1].precio;
                    }
                }
            }
        }
        if encontro {
            self.autos[self.autos.len() - 1] = aux;
            self.cant -= 1;
        }
    }
    /*➢ buscar_auto(auto): busca un auto y si lo encuentra lo retorna */
    pub fn buscar_auto(&self, auto: Auto) -> Option<&Auto> {
        let mut car: Option<&Auto> = None;
        for i in 1..self.autos.len() {
            if self.autos[i].marca.eq(&auto.marca.clone())
                && self.autos[i].año == auto.año
                && self.autos[i].modelo == auto.modelo
            {
                car = Some(&self.autos[i]);
            }
        }
        car
    }
}
#[derive(Debug)]
pub struct Auto {
    marca: String,
    modelo: u32,
    año: u16,
    precio: f32,
    color: Colores,
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
#[derive(Clone, Copy, Debug)]
pub enum Colores {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

/*8- Defina la estructura Cancion con campos para el título, el artista y el género. El género
puede ser rock, pop, rap, jazz, otros. Luego modele una playlist. La playlist está compuesta
por una lista de canciones y un nombre, y se permiten hacer las siguientes acciones sobre
ella:
➔ agregar canción.
➔ eliminar canción.
➔ mover canción // mueve la canción a una determinada posición de la playlist.
➔ buscar canción por nombre.
➔ obtener las canciones de un determinado género.
➔ obtener las canciones de un determinado artista.
➔ modificar título de la playlist.
➔ eliminar todas las canciones */
#[derive(Debug)]
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
#[derive(Debug)]
pub struct PlayList<'a> {
    nombre: String,
    lista: Vec<&'a Cancion>,
    cant: usize,
}
impl<'a> PlayList<'a> {
    pub fn new(nombre: String, lista: Vec<&'a Cancion>, cant: usize) -> PlayList {
        PlayList {
            nombre,
            lista,
            cant,
        }
    }
    pub fn agregar_cancion(&mut self, song: &'a Cancion) {
        self.lista.insert(self.cant, song);
        self.cant += 1;
    }
    pub fn eliminar_cancion(&mut self, song: &Cancion) {
        for i in 0..self.cant - 1 {
            if self.lista[i].title.eq(&song.title) && self.lista[i].artist.eq(&song.artist) {
                self.lista.remove(i);
                self.cant -= 1;
            }
        }
    }
    pub fn mover_cancion(&mut self, song: &'a Cancion, pos: usize) {
        for i in 0..self.cant - 1 {
            if self.lista[i].title.eq(&song.title) && self.lista[i].artist.eq(&song.artist) {
                let aux = self.lista.remove(i);
                self.lista.insert(pos - 1, aux);
            }
        }
    }
    pub fn buscar_cancion_por_nombre(&self, nombre: String) -> Option<&Cancion> {
        let mut res: Option<&Cancion> = None;
        for i in 0..self.cant - 1 {
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
        let mut p = PlayList {
            nombre,
            lista,
            cant,
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
        let mut p = PlayList {
            nombre,
            lista,
            cant,
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
#[derive(Clone, Copy, Debug)]
pub enum Generos {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}
/*9.-Dada una cadena de veterinarias se desea implementar un sistema de atención de
pacientes para cada veterinaria, de la veterinaria se conoce el nombre, la dirección y un id.
Para la atención de mascotas se requiere administrar una cola de atención. De la mascota
se conoce el nombre, la edad, el tipo de animal(perro, gato, caballo, otros) y su dueño. Del
dueño se conoce el nombre, la dirección y un teléfono de contacto. Luego de la atención se
desea tener un registro de las atenciones realizadas guardando los datos de la mascota, el
diagnóstico final, tratamiento y fecha de la próxima visita si es que se requiere.
Dado todo lo mencionado anteriormente implemente los métodos para realizar las
siguientes acciones:*/

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

/*
➔ crear una veterinaria.
➔ agregar una nueva mascota a la cola de atención de la veterinaria.
➔ agregar una nueva mascota a la cola de atención pero que sea la siguiente
en atender porque tiene la máxima prioridad.
➔ atender la próxima mascota de la cola.
➔ eliminar una mascota específica de la cola de atención dado que se retira.
➔ registrar una atención.
➔ buscar una atención dado el nombre de la mascota, el nombre del dueño y el
teléfono.
➔ modificar el diagnóstico de una determinada atención.
➔ modificar la fecha de la próxima visita de una determinada atención.
➔ eliminar una determinada atención.
Nota: para la fecha utilice lo implementado en el punto 3 */

/*10-Para una biblioteca se desea implementar un sistema de préstamos de libros. De la
biblioteca se conoce el nombre y la dirección, las copias de los libros a disposición para
prestar y los préstamos efectuados. Los libros a disposición es un registro donde se indica
la cantidad de ejemplares que tiene a disposición para prestar de determinado libro. De
cada libro se conoce el título, autor, número de páginas, género(novela, infantil, técnico,
otros). Para registrar un préstamo se requiere el libro, el cliente, la fecha de vencimiento del
préstamo, la fecha de devolución y el estado que puede ser devuelto o en préstamo. Del
cliente se conoce el nombre, teléfono y dirección de correo electrónico.
Implemente los métodos necesarios para realizar las siguientes acciones */
#[derive(Debug)]

pub struct Cliente {
    nombre: String,
    telefono: u64,
    correo: String,
}
impl Cliente {
    pub fn new(nombre: String, telefono: u64, correo: String) -> Cliente {
        Cliente {
            nombre,
            telefono,
            correo,
        }
    }
    pub fn es_igual(&self, cliente: &Cliente) -> bool {
        let mut aux = false;
        if self.nombre.eq_ignore_ascii_case(&cliente.nombre) && self.telefono == cliente.telefono {
            aux = true;
        }
        aux
    }
}
#[derive(Debug)]

pub struct Libro {
    titulo: String,
    autor: String,
    numero_pags: u16,
    genero: GenerosLiterarios,
}
#[derive(Debug)]
pub enum GenerosLiterarios {
    Novela,
    Infantil,
    Técnico,
    Otros,
}
impl Libro {
    pub fn new(
        titulo: String,
        autor: String,
        numero_pags: u16,
        genero: GenerosLiterarios,
    ) -> Libro {
        Libro {
            titulo,
            autor,
            numero_pags,
            genero,
        }
    }
    pub fn es_igual(&self, libro: &Libro) -> bool {
        let mut aux = false;
        if self.autor.eq_ignore_ascii_case(&libro.autor)
            && self.titulo.eq_ignore_ascii_case(&libro.titulo)
        {
            aux = true;
        }
        aux
    }
}
#[derive(Debug)]

pub struct Prestamo<'a> {
    libro: &'a mut Libro,
    cliente: &'a mut Cliente,
    vencimiento: &'a mut Fecha,
    fecha_devolucion: Option<&'a Fecha>,
    estado: EstadoLibro,
}
impl<'a> Prestamo<'a> {
    pub fn new(
        libro: &'a mut Libro,
        cliente: &'a mut Cliente,
        vencimiento: &'a mut Fecha,
        fecha_devolucion: Option<&'a Fecha>,
        estado: EstadoLibro,
    ) -> Prestamo<'a> {
        Prestamo {
            libro,
            cliente,
            vencimiento,
            fecha_devolucion,
            estado,
        }
    }
    pub fn set_devuelto(&mut self, fecha: &'a Fecha) {
        self.estado = EstadoLibro::Devuelto;
        self.fecha_devolucion = Some(fecha);
    }
}
#[derive(Debug)]

pub enum EstadoLibro {
    Devuelto,
    Prestado,
}
#[derive(Debug)]
pub struct Biblioteca<'a> {
    nombre: String,
    direccion: String,
    libros: Vec<(&'a mut Libro, u16)>,
    prestamos: Vec<Prestamo<'a>>,
}
impl<'a> Biblioteca<'a> {
    pub fn new(nombre: String, direccion: String) -> Biblioteca<'a> {
        let libros = Vec::new();
        let prestamos = Vec::new();
        Biblioteca {
            nombre,
            direccion,
            libros,
            prestamos,
        }
    }
    pub fn set_libro(&mut self, libro: &'a mut Libro, cantidad: u16) {
        self.libros.push((libro, cantidad));
    }
    pub fn get_prestamo_index(&mut self, i: usize) -> Option<&Prestamo> {
        let mut auxi: Option<&Prestamo> = None;
        if self.prestamos.len() >= i {
            auxi = Some(&self.prestamos[i]);
        }
        auxi
    }
    /*➔ obtener cantidad de copias: dado un determinado libro retorna el retorna la
    cantidad de copias a disposición que hay para prestar de dicho libro.*/
    pub fn obtener_copias_disponibles(&mut self, libro: &Libro) -> u16 {
        //retorna los que estan para prestar
        let mut num: u16 = 0;
        let mut prestados: u16 = 0;
        for (libros, nums) in &self.libros {
            if libro.autor.eq_ignore_ascii_case(&libros.autor)
                || libro.titulo.eq_ignore_ascii_case(&libros.titulo)
            {
                num = *nums
            }
        }
        for i in 0..self.prestamos.len() {
            if self.prestamos[i]
                .libro
                .autor
                .eq_ignore_ascii_case(&libro.autor)
                || self.prestamos[i]
                    .libro
                    .titulo
                    .eq_ignore_ascii_case(&libro.titulo)
            {
                prestados += 1;
            }
        }
        num - prestados
    }
    /*➔ decrementar cantidad de copias a disposición; dado un libro decrementa en 1
    la cantidad de copias de libros a disposición para prestar. */
    pub fn decrementar_copias(&mut self, libro: &Libro) {
        for i in 0..self.libros.len() {
            if libro.titulo.eq_ignore_ascii_case(&self.libros[i].0.titulo) {
                self.libros[i].1 = self.libros[i].1 - 1;
            }
        }
    }
    /*➔ incrementar cantidad de copias a disposición: dado un libro incremente en 1
    la cantidad de copias del libro a disposición para ser prestado. */
    pub fn incrementar_copias(&mut self, libro: &Libro) {
        for i in 0..self.libros.len() {
            if libro.titulo.eq_ignore_ascii_case(&self.libros[i].0.titulo) {
                self.libros[i].1 = self.libros[i].1 + 1;
            }
        }
    }
    /*➔ contar préstamos de un cliente: devuelve la cantidad de préstamos en estado
    “en préstamo” de un determinado cliente. */
    pub fn contar_prestamos_cliente(&mut self, cliente: &Cliente) -> i32 {
        let mut cant = 0;
        for i in 0..self.prestamos.len() {
            if self.prestamos[i]
                .cliente
                .nombre
                .eq_ignore_ascii_case(&cliente.nombre)
                || self.prestamos[i]
                    .cliente
                    .correo
                    .eq_ignore_ascii_case(&cliente.correo)
            {
                match self.prestamos[i].estado {
                    EstadoLibro::Prestado => cant += 1,
                    EstadoLibro::Devuelto => (),
                }
            }
        }
        cant
    }
    /*➔ ver la cantidad disponible de un determinado libro: retorna la cantidad de
    libros disponibles del registro de “copias a disposición” de un determinado
    libro. */
    pub fn obtener_cantidad_copias(&mut self, libro: &Libro) -> u16 {
        //retorna el total de copias de un libro
        let mut num: u16 = 0;
        for (libros, nums) in &self.libros {
            if libro.autor.eq_ignore_ascii_case(&libros.autor)
                || libro.titulo.eq_ignore_ascii_case(&libros.titulo)
            {
                num = *nums
            }
        }
        num
    }
    /*➔ realizar un préstamo de un libro para un cliente: crea un préstamo de un libro
    para un determinado cliente cumpliendo con lo siguiente
    ◆ el cliente no tenga más de 5 préstamos en el estado “en préstamo”
    ◆ haya al menos una copia disponible en el registro de copias a
    disposición.
    De ser así descuenta 1 en el registro de “copias a disposición” y
    retorna true, si no cumple con alguna de las condiciones retorna false. */
    pub fn realizar_prestamo(
        &mut self,
        cliente: &'a mut Cliente,
        libro: &'a mut Libro,
        vencimiento: &'a mut Fecha,
    ) -> bool {
        let mut es = false;
        if self.contar_prestamos_cliente(&cliente) <= 5
            && self.obtener_copias_disponibles(&libro) >= 1
        {
            let prest = Prestamo::new(libro, cliente, vencimiento, None, EstadoLibro::Prestado);

            self.prestamos.push(prest);
            es = true;
        }
        es
    }
    /*➔ ver préstamos a vencer el los próximos días: retorna una lista de préstamos a
    vencer el los próximos días, el valor de días es pasado por parámetro. */
    pub fn prestamos_por_vencer(
        &mut self,
        plazo: usize,
        fecha_actual: &mut Fecha,
    ) -> Vec<&Prestamo> {
        let mut vec = Vec::new();
        for i in 0..self.prestamos.len() {
            let aux = self.prestamos.get(i);
            match aux {
                Some(dato) => {
                    fecha_actual.sumar_dias(plazo);
                    if !dato.vencimiento.es_mayor(fecha_actual) {
                        vec.push(&self.prestamos[i]);
                    }
                }
                None => panic!("Out of index"),
            }
        }
        vec
    }
    /*➔ ver los préstamos vencidos: retorna una lista de préstamos en el estado “en
    préstamos” donde la fecha de vencimiento es menor a la fecha actual.*/
    pub fn prestamos_vencidos(&mut self, fecha_actual: &Fecha) -> Vec<&Prestamo> {
        let mut vect = Vec::new();
        for i in 0..self.prestamos.len() {
            if !self.prestamos[i].vencimiento.es_mayor(&fecha_actual) {
                vect.push(&self.prestamos[i]);
            }
        }
        vect
    }
    /*➔ buscar préstamo: dado un libro y un cliente busca un préstamo y lo retorna si
    existe. */
    pub fn buscar_prestamo(&self, libro: &mut Libro, cliente: &mut Cliente) -> Option<&Prestamo> {
        let mut aux: Option<&Prestamo> = None;
        for i in 0..self.prestamos.len() {
            if self.prestamos[i].libro.es_igual(&libro)
                && self.prestamos[i].cliente.es_igual(&cliente)
            {
                aux = Some(&self.prestamos[i]);
            }
        }
        aux
    }
    /*
    ➔ devolver libro: dado un libro y un cliente se busca el préstamo y se cambia al
    estado “devuelto”, se registra la fecha de devolución y se incrementa la
    cantidad de libros en 1 del libro devuelto en el registro de copias a
    disposición. */
    pub fn devolver_libro(&mut self, libro: &Libro, cliente: &Cliente, fecha: &'a Fecha) {
        for i in 0..self.prestamos.len() {
            if self.prestamos[i].libro.es_igual(&libro)
                && self.prestamos[i].cliente.es_igual(&cliente)
            {
                match self.get_prestamo_index(i) {
                    Some(_) => {
                        self.prestamos[i].set_devuelto(fecha);
                    }
                    None => (),
                }
            }
        }
    }
}
