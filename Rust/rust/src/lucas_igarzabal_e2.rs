//https://crates.io/crates/date
use date::Date;
use std::collections::VecDeque;
#[derive(Debug)]
pub struct Fecha {
    dato: date::Date,
    arreglo_meses: [u8; 12],
}
impl Fecha {
    /*➢ new: que pasando los parámetros correspondientes, crea una Fecha y la retorna. */
    pub fn new(año: u32, mes: u8, dia: u8) -> Fecha {
        let dato = date::Date::new(año, mes, dia);
        let arreglo_meses: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        let f = Fecha {
            dato,
            arreglo_meses,
        };
        if f.es_fecha_valida() {
            f
        } else {
            panic!("Fecha Invalida")
        }
    }
    /*➢ es_fecha_valida: retorna true si es una fecha valida, false caso contrario.//tenga en
    cuenta los años bisiestos también. */
    pub fn es_fecha_valida(&self) -> bool {
        let mut res = false;
        if self.año > 0 && self.dato.day > 0 && self.dato.month > 0 && self.dato.month < 12 {
            if self.dato.day <= self.arreglo_meses[self.dato.month as usize - 1] as u8 {
                res = true;
            } else if self.dato.month == 2 && self.dato.day == 29 && self.es_bisiesto() {
                res = true;
            }
        }
        res
    }
    pub fn es_igual(&self, other: &Self) -> bool {
        let mut es = false;
        if self.dato == other.dato {
            es = true;
        }
        es
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
        while aux > self.arreglo_meses[(self.dato.month - 1) as usize] as usize {
            aux -= self.arreglo_meses[(self.dato.month - 1) as usize] as usize;
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
            aux += self.arreglo_meses[(self.dato.month - 1) as usize] as usize;
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

// Desde aca la veterinaria

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
    /*veterinarias ejercicio 9
    implemente un método en la veterinaria con la siguiente firma: emails_a_enviar(fecha_de_hoy: Fecha, dias:u8) -> Vec<String>
    que reciba la fecha de hoy  y una cantidad de días, retornando los emails de los dueños de las mascotas que fueron atendidas donde tienen una próxima visita en los dias pasados por parámetro.
    No puede haber emails repetidos */
    pub fn emails_a_enviar(&self, fecha_de_hoy: Fecha, dias: u8) -> Vec<String> {
        let mut mails = Vec::new();
        let mut fecha = fecha_de_hoy;
        fecha.sumar_dias(dias as usize);
        for reg in &self.reg {
            match &reg.prox_visita {
                Some(a) => {
                    if a.es_igual(&fecha) {
                        mails.push(reg.datos.dueño.direccion.clone())
                    }
                }
                None => (),
            }
        }
        mails
    }
}
