/*1-Definir la función llamada es_par que recibe como parámetro un número entero y retorna
true si el número es par, false caso contrario. */

pub fn es_par(dato: i32) -> bool {
    let mut es: bool = false;
    if dato % 2 == 0 {
        es = true;
    }
    es
}

/*2- Definir la función llamada es_primo que recibe un número entero positivo mayor a 1 y
retorna true si es primo, false caso contrario. */

pub fn es_primo(dato: u32) -> bool {
    let mut ret: bool;
    for i in 2..dato - 1 {
        if dato % i == 0 {
            ret = false;
            return ret;
        }
    }
    ret = true;
    ret
}

/*3- Definir la función llamada suma_pares que recibe como parámetro un arreglo de
números enteros y retorna la suma de los números pares. */

pub fn suma_pares(arr: [i32; 3]) -> i32 {
    let mut suma: i32 = 0;
    for i in 0..arr.len() {
        if arr[i] % 2 == 0 {
            suma += arr[i];
        }
    }
    suma
}

/*4- Definir la función llamada cantidad_impares que recibe como parámetro un arreglo de
números enteros y retorna la cantidad de números impares. */

pub fn cantidad_impares(arr: [i32; 3]) -> i32 {
    let mut cant = 0;
    for i in 0..arr.len() {
        if arr[i] % 2 != 0 {
            cant += 1;
        }
    }
    cant
}

/*5-Defina la función llamada duplicar_valores que recibe un arreglo de números flotantes y
retorna un arreglo nuevo con los valores duplicados del parámetro. */

pub fn duplicar_valores(arr: [f64; 3]) -> [f64; 3] {
    let mut aux: [f64; 3] = [0.0, 0.0, 0.0];
    for i in 0..arr.len() {
        aux[i] = arr[i] * 2.0;
    }
    return aux;
}

/*6-Definir la función llamada longitud_de_cadenas que recibe un arreglo de String y retorna
un arreglo con la longitud de las cadenas del parámetro, correspondiéndose en posición del
arreglo. */

pub fn longitud_de_cadenas(arr: [String; 3]) -> [i32; 3] {
    let mut aux: [i32; 3] = [0, 0, 0];
    for i in 0..arr.len() {
        aux[i] = arr[i].len() as i32;
    }
    aux
}

/*7-Definir la función llamada cantidad_de_mayores que recibe como parámetro un arreglo
de números enteros y un número entero llamado límite. Esta función retorna la cantidad de
números mayores al límite que tiene el arreglo. */

pub fn cantidad_de_mayores(arr: [i32; 3], limite: i32) -> i32 {
    let mut cant = 0;
    for i in 0..arr.len() {
        if arr[i] > limite {
            cant += 1;
        }
    }
    cant
}

/*8- Definir la función llamada sumar_arreglos que recibe 2 arreglos del mismo tamaño de
números flotantes y retorna un nuevo arreglo que contiene la suma de los elementos de los
arreglos pasados por parámetro, correspondiendose el resultado con cada posición de los
arreglos pasados por parámetro. */

pub fn sumar_arreglos(arr1: [f64; 3], arr2: [f64; 3]) -> [f64; 3] {
    let mut aux: [f64; 3] = [0.0, 0.0, 0.0];
    for i in 0..arr1.len() {
        aux[i] = arr1[i] + arr2[i];
    }
    aux
}

/*9-Definir la función llamada cantidad_en_rango que recibe 3 parámetros: 1 arreglo de
enteros, un número entero llamado inferior y otro número entero llamado superior. Esta
función retorna la cantidad de números del arreglo que están entre el rango de los
parámetros inferior y superior inclusive. */

pub fn cantidad_en_rango(arr: [i32; 3], inferior: i32, superior: i32) -> i32 {
    let mut cant = 0;
    for i in 0..arr.len() {
        if arr[i] > inferior && arr[i] < superior {
            cant += 1;
        }
    }
    cant
}

/*10-Definir la función llamada cantidad_de_cadenas_mayor_a que recibe como parámetros
un arreglo de String y un entero llamado límite. Esta función retorna la cantidad de Strings
del arreglo que son de longitud mayor al parámetro límite. */

pub fn cantidad_de_cadenas_mayor_a(arr: [String; 3], limite: i32) -> i32 {
    let mut cant = 0;
    for i in 0..arr.len() {
        if arr[i].len() as i32 > limite {
            cant += 1;
        }
    }
    cant
}

/*11-Definir la función llamada multiplicar_valores que recibe como parámetro un arreglo de
enteros y otro número entero llamado factor. Esta función multiplica los valores del arreglo
por el parámetro factor modificándolo. */

pub fn multiplicar_valores(arr1: &mut [i32; 3], factor: i32) {
    for i in 0..arr1.len() {
        arr1[i] *= factor;
    }
}

/*12-Definir una función llamada reemplazar_pares que recibe un arreglo de enteros y
reemplaza todos los números pares por -1. */

pub fn reemplazar_pares(arr1: &mut [i32; 3]) {
    for i in 0..arr1.len() {
        if arr1[i] % 2 == 0 {
            arr1[i] = -1;
        }
    }
}

/*13-Definir una función llamada ordenar_nombres que recibe un arreglo de String y los
ordena en orden alfabético. */

pub fn ordenar_nombres(arr: &mut [String; 3]) {
    let mut staux: String = String::new();
    for i in 0..arr.len() {
        for j in (i + 1..arr.len()).rev() {
            if arr[j].chars().nth(0).unwrap() < arr[i].chars().nth(0).unwrap() {
                staux = arr[i].clone();
                arr[i] = arr[j].clone();
                arr[j] = staux;
            }
        }
    }
}

/*14-Definir una función llamada incrementar que recibe como parámetro un número flotante
e incrementa en 1 su valor. */

pub fn incrementar(dato: &mut f64) {
    let aumento: f64 = 1.0;
    *dato = *dato + aumento;
}
